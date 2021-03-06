use std::error::Error;

use reqwest;
use serde_derive::Deserialize;
use serde_json::json;

use super::fmt;
use smitemotd::Model;

#[derive(Deserialize, Debug)]
pub struct PushbulletOpts {
  token: String,
  channel_tag: String,
}

pub struct Pushbullet {
  opts: PushbulletOpts,
}

impl Pushbullet {
  pub fn new(opts: PushbulletOpts) -> Self {
    Self { opts }
  }
}

impl super::Notify for Pushbullet {
  fn notify(&self, m: &Model) -> Result<(), Box<Error>> {
    reqwest::Client::new()
      .post("https://api.pushbullet.com/v2/pushes")
      .header("Access-Token", self.opts.token.clone())
      .json(&json!({
        "channel_tag": &self.opts.channel_tag,
        "type": "note",
        "title": m.title,
        "body": fmt::format(&m, false),
      }))
      .send()?
      .json()?;

    Ok(())
  }
}
