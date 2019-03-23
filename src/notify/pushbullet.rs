use std::error::Error;

use reqwest;
use serde_json::json;

use crate::model::Model;

pub struct Pushbullet {
  access_token: String,
  channel_tag: String,
}

impl Pushbullet {
  pub fn new(access_token: &str, channel_tag: &str) -> Self {
    Self {
      access_token: access_token.to_string(),
      channel_tag: channel_tag.to_string(),
    }
  }
}

impl super::Notify for Pushbullet {
  fn notify(&self, m: Model) -> Result<(), Box<Error>> {
    reqwest::Client::new()
      .post("https://api.pushbullet.com/v2/pushes")
      .header("Access-Token", self.access_token.clone())
      .json(&json!({
        "channel_tag": &self.channel_tag,
        "type": "note",
        "title": m.title,
        "body": m.to_string(),
      }))
      .send()?
      .json()?;

    Ok(())
  }
}
