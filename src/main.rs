use std::error::Error;

use clap::{crate_authors, crate_description, crate_name, crate_version, App, AppSettings, Arg};
use env_logger;

use crate::notify::Notify;

mod api;
mod model;
mod notify;
mod types;
#[macro_use]
mod macros;

fn main() -> Result<(), Box<Error>> {
  env_logger::init();

  let matches = App::new(crate_name!())
    .setting(AppSettings::ColoredHelp)
    .setting(AppSettings::ColorAuto)
    .version(crate_version!())
    .author(crate_authors!())
    .about(crate_description!())
    .arg(
      Arg::with_name("dev-id")
        .long("dev-id")
        .value_name("ID")
        .help("Sets the dev ID for the Smite API")
        .takes_value(true)
        .required(true)
        .env("DEV_ID"),
    )
    .arg(
      Arg::with_name("auth-key")
        .long("auth-key")
        .value_name("KEY")
        .help("Sets the auth key for the Smite API")
        .takes_value(true)
        .required(true)
        .env("AUTH_KEY"),
    )
    .arg(
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
        .value_delimiter(";")
        .env("NOTIFY_PUSHED"),
    )
    .arg(
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
        .value_delimiter(";")
        .env("NOTIFY_PUSHBULLET"),
    )
    .arg(
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
        .value_delimiter(";")
        .env("NOTIFY_SLACK"),
    )
    .arg(
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
        .value_delimiter(";")
        .env("NOTIFY_STREAM"),
    )
    .get_matches();

  // println!("{:#?}", matches);

  // let t = suboptions!(matches.value_of("notify-pushbullet").unwrap(), notify::pushbullet::PushbulletOpts);
  // println!("{:#?}", t);

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

  let mut smite = api::Smite::new(env!("SMITE_DEV_ID"), env!("SMITE_AUTH_KEY"));
  let gods = smite.get_gods()?;
  // let motds = smite.get_motd()?;

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
  let motds = r#"[{
    "team2GodsCSV": "1956, 1668, 2034, 1898, 1748, 2037, 1809, 1678, 2008, 1966, 1921, 1784, 1978, 1763, 1848, 1673, 1677, 1993, 1915, 1872, 1958, 2000, 1988, 2005, 1747, 2030, 1924, 1991, 1723, 1864, 1926",
    "title": "Don't Pinch Me Bro",
    "gameMode": "Siege",
    "ret_msg": null,
    "startDateTime": "3/17/2019 9:00:00 AM",
    "description": "<li>The gods in green are having a St. Patricks day party in the most green battlefield they can, the Mayan Jungle.<li>Map: Siege (5v5)</li><li>Gods: Ones that have green on them.</li><li>Selection: Blind Pick</li></li>",
    "name": "Don't Pinch Me Bro",
    "maxPlayers": "5",
    "team1GodsCSV": "1956, 2056, 1668, 2034, 1898, 1748, 2037, 1809, 1678, 2008, 1966, 1921, 2075, 1784, 1978, 1763, 1848, 1673, 1677, 1993, 2051, 1915, 1872, 1958, 2000, 2113, 2065, 1988, 2005, 1747, 2030, 1924, 1991, 1723, 1864, 2072, 1926"
  }]"#;

  let motds: types::Motds = serde_json::from_str(motds)?;

  let model = model::parse(gods, motds)?;
  // println!("{}", model.to_string());
  // let slack = notify::slack::Slack::new(env!("SLACK_HOOK"));
  // slack.notify(model)?;

  for n in notifies {
    n.notify(&model);
  }

  Ok(())
}
