use std::error::Error;

use env_logger;
use serde_json;

use crate::notify::Notify;

mod api;
mod models;
mod motd;
mod notify;

fn main() -> Result<(), Box<Error>> {
  env_logger::init();

  let mut smite = api::Smite::new(env!("SMITE_DEV_ID"), env!("SMITE_AUTH_KEY"));
  // let motds = smite.get_motd()?;

  println!("{:?}", smite.get_gods()?);
  return Ok(());

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

  // let motds: models::Motd = serde_json::from_str(motds)?;

  // let motd = motd::parse(motds)?;
  // // println!("{:#?}", motd);

  // let slack = notify::slack::Slack::new(env!("SLACK_HOOK"));
  // slack.notify(motd)?;

  Ok(())
}
