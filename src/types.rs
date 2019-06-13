use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Error {
  ret_msg: String,
  session_id: String,
  timestamp: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
  pub timestamp: String,
  pub session_id: String,
  pub ret_msg: String,
}

pub type Motds = Vec<Motd>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Motd {
  #[serde(rename = "team2GodsCSV")]
  pub team2_gods_csv: Option<String>,
  pub title: Option<String>,
  #[serde(rename = "gameMode")]
  pub game_mode: Option<String>,
  pub ret_msg: Option<String>,
  #[serde(rename = "startDateTime")]
  pub start_date_time: Option<String>,
  pub description: Option<String>,
  pub name: Option<String>,
  #[serde(rename = "maxPlayers")]
  pub max_players: Option<String>,
  #[serde(rename = "team1GodsCSV")]
  pub team1_gods_csv: Option<String>,
}

pub type Gods = Vec<God>;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct God {
  #[serde(rename = "Name")]
  pub name: String,
  #[serde(rename = "godCard_URL")]
  pub god_card_url: String,
  #[serde(rename = "godIcon_URL")]
  pub god_icon_url: String,
  pub id: i64,
}
