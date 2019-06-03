#[macro_use]
extern crate log;

use std::error::Error;
use std::fs;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};
use dirs;
use pretty_env_logger;

use smitemotd::{Model, Smite};

mod notify;
#[macro_use]
mod macros;
mod pickledb;

macro_rules! arg_env {
  ($arg:expr, $str:expr) => {
    #[allow(clippy::let_and_return)]
    {
      #[allow(unused_mut)]
      let mut arg = $arg.env($str);
      #[cfg(feature = "compile_env")]
      {
        if let Some(v) = option_env!($str) {
          arg = arg.default_value(v);
        }
      }
      arg
    }
  };
}

fn main() -> Result<(), Box<Error>> {
  pretty_env_logger::init();

  let matches = App::new(crate_name!())
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::ColorAuto)
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .arg(arg_env!(
      Arg::with_name("dev-id")
        .long("dev-id")
        .value_name("ID")
        .help("Sets the dev ID for the Smite API")
        .takes_value(true)
        .required(true),
      "DEV_ID"
    ))
    .arg(arg_env!(
      Arg::with_name("auth-key")
        .long("auth-key")
        .value_name("KEY")
        .help("Sets the auth key for the Smite API")
        .takes_value(true)
        .required(true),
      "AUTH_KEY"
    ))
    .arg(arg_env!(
      Arg::with_name("notify-pushed")
        .long("notify-pushed")
        .value_name("OPTS")
        .help("Notifies via Pushed.co")
        .long_help(
          r#"Notifies via Pushed.co

Options:
  key - The app key for Pushed
  secret - The app secret for Pushed

Example: 'key="1234", secret="5678"'

Extra:"#,
        )
        .takes_value(true)
        .multiple(true)
        .use_delimiter(true)
        .value_delimiter(";"),
      "NOTIFY_PUSHED"
    ))
    .arg(arg_env!(
      Arg::with_name("notify-pushbullet")
        .long("notify-pushbullet")
        .value_name("OPTS")
        .help("Notifies via Pushbullet")
        .long_help(
          r#"Notifies via Pushbullet

Options:
  token - The token for Pushbullet
  channel_tag - The channel tag to push to

Example: 'token="1234", channel_tag="smitemotd"'

Extra:"#,
        )
        .takes_value(true)
        .multiple(true)
        .use_delimiter(true)
        .value_delimiter(";"),
      "NOTIFY_PUSHBULLET"
    ))
    .arg(arg_env!(
      Arg::with_name("notify-slack")
        .long("notify-slack")
        .value_name("OPTS")
        .help("Notifies via Slack")
        .long_help(
          r#"Notifies via Slack

Options:
  hook - The url for the slack hook

Example: 'hook="https://hooks.slack.com/services/..."'

Extra:"#,
        )
        .takes_value(true)
        .multiple(true)
        .use_delimiter(true)
        .value_delimiter(";"),
      "NOTIFY_SLACK"
    ))
    .arg(arg_env!(
      Arg::with_name("notify-stream")
        .long("notify-stream")
        .value_name("OPTS")
        .help("Notifies via stdout, stderr, or a file")
        .long_help(
          r#"Notifies via stdout, stderr, or a file

Options:
  stdout - Whether to use stdout [true/false]
  stderr - Whether to use stderr [true/false]
  file - The file name to write to
  color: Whether to colorize output (ANSI) [true/false]

stdout, stderr, and file are mutually exclusive

Examples: 
  'stdout=true'
  'stderr=true, color=true'
  'file="/tmp/output"'

Extra:"#,
        )
        .takes_value(true)
        .multiple(true)
        .use_delimiter(true)
        .value_delimiter(";"),
      "NOTIFY_STREAM"
    ))
    .arg(arg_env!(
      Arg::with_name("notify-email")
        .long("notify-email")
        .value_name("OPTS")
        .help("Notifies via email")
        .long_help(
          r#"Notifies via email

Options:
  from - The from email address
  recipients - A list of recipients to receive the email, 
  subject - Optional subject for email
  username - Optional username for SMTP server
  password - Optional password for SMTP server
  smtp - The smtp host to connect to

Example: 
  'from="hey@gmail.com", recipients = ["yolo@gmail.com"], subject = "Hey", smtp = "smtp.gmail.com"'

Extra:"#,
        )
        .takes_value(true)
        .multiple(true)
        .use_delimiter(true)
        .value_delimiter(";"),
      "NOTIFY_EMAIL"
    ))
    .get_matches();

  trace!("Initialized");

  let mut notifies: Vec<Box<notify::Notify>> = vec![];

  if let Some(values) = matches.values_of("notify-slack") {
    for value in values {
      let opts = suboptions!(value, notify::slack::SlackOpts);
      notifies.push(Box::new(notify::slack::Slack::new(opts)));
    }
  }

  if let Some(values) = matches.values_of("notify-pushed") {
    for value in values {
      let opts = suboptions!(value, notify::pushed::PushedOpts);
      notifies.push(Box::new(notify::pushed::Pushed::new(opts)));
    }
  }

  if let Some(values) = matches.values_of("notify-pushbullet") {
    for value in values {
      let opts = suboptions!(value, notify::pushbullet::PushbulletOpts);
      notifies.push(Box::new(notify::pushbullet::Pushbullet::new(opts)));
    }
  }

  if let Some(values) = matches.values_of("notify-stream") {
    for value in values {
      let opts = suboptions!(value, notify::stream::StreamOpts);
      let stream = notify::stream::Stream::new(opts)?;
      notifies.push(Box::new(stream));
    }
  }

  if let Some(values) = matches.values_of("notify-email") {
    for value in values {
      let opts = suboptions!(value, notify::email::EmailOpts);
      notifies.push(Box::new(notify::email::Email::new(opts)?));
    }
  }

  if notifies.is_empty() {
    let stream = notify::stream::Stream::new(notify::stream::StreamOpts {
      stdout: Some(true),
      color: Some(true),
      ..notify::stream::StreamOpts::default()
    })?;
    notifies.push(Box::new(stream));
  }

  let app_config_path = match dirs::config_dir() {
    Some(v) => v.join("smitemotd"),
    None => "".into(),
  };

  fs::create_dir_all(&app_config_path)?;

  let mut smite = Smite::new(
    matches.value_of("dev-id").unwrap(),
    matches.value_of("auth-key").unwrap(),
    Box::new(pickledb::PickleDb::new(app_config_path)),
  );

  let gods = smite.get_gods()?;
  let motds = smite.get_motd()?;

  // let motds = r#"[
  //   {
  //     "team2GodsCSV": "",
  //     "title": "Cooldowns Runneth Over",
  //     "gameMode": "Arena_V3",
  //     "ret_msg": null,
  //     "startDateTime": "3/3/2019 10:00:00 AM",
  //     "description": "<li>Fire abilities at will in this frantic MOTD where all your Abilities have a greatly reduced Cooldown!<li>Map: Arena</li><li>Infinite Mana</li><li>Starting/Maximum Cooldown Reduction: 75% (no use in stacking more CDR)</li><li>Starting Level:20</li><li>Starting Gold:100,000</li><li>Gods: All</li><li>Selection: Random</li></li>",
  //     "name": "Cooldowns Runneth Over",
  //     "maxPlayers": "",
  //     "team1GodsCSV": ""
  //   }
  // ]"#;
  // let motds = r#"[{
  //   "team2GodsCSV": "1956, 1668, 2034, 1898, 1748, 2037, 1809, 1678, 2008, 1966, 1921, 1784, 1978, 1763, 1848, 1673, 1677, 1993, 1915, 1872, 1958, 2000, 1988, 2005, 1747, 2030, 1924, 1991, 1723, 1864, 1926",
  //   "title": "Don't Pinch Me Bro",
  //   "gameMode": "Siege",
  //   "ret_msg": null,
  //   "startDateTime": "3/17/2019 9:00:00 AM",
  //   "description": "<li>The gods in green are having a St. Patricks day party in the most green battlefield they can, the Mayan Jungle.<li>Map: Siege (5v5)</li><li>Gods: Ones that have green on them.</li><li>Selection: Blind Pick</li></li>",
  //   "name": "Don't Pinch Me Bro",
  //   "maxPlayers": "5",
  //   "team1GodsCSV": "1956, 2056, 1668, 2034, 1898, 1748, 2037, 1809, 1678, 2008, 1966, 1921, 2075, 1784, 1978, 1763, 1848, 1673, 1677, 1993, 2051, 1915, 1872, 1958, 2000, 2113, 2065, 1988, 2005, 1747, 2030, 1924, 1991, 1723, 1864, 2072, 1926"
  // }]"#;

  // let motds: types::Motds = serde_json::from_str(motds)?;

  let model = Model::parse(gods, motds)?;
  for n in notifies {
    if let Err(e) = n.notify(&model) {
      eprintln!("Error: {}", e);
    }
  }

  Ok(())
}
