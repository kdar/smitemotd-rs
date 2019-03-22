use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
  pub timestamp: String,
  pub session_id: String,
  pub ret_msg: String,
}

pub type Motd = Vec<MotdElement>;

#[derive(Serialize, Deserialize, Debug)]
pub struct MotdElement {
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
