use std::error::Error;

use reqwest;
use serde_derive::Deserialize;

use crate::model::Model;

#[derive(Deserialize, Debug)]
pub struct PushedOpts {
  key: String,
  secret: String,
}

pub struct Pushed {
  opts: PushedOpts,
}

impl Pushed {
  pub fn new(opts: PushedOpts) -> Self {
    Self {
      opts,
    }
  }
}

impl super::Notify for Pushed {
  fn notify(&self, m: &Model) -> Result<(), Box<Error>> {
    reqwest::Client::new()
      .post("https://api.pushed.co/1/push")
      .form(&[
        ("app_key", self.opts.key.clone()),
        ("app_secret", self.opts.secret.clone()),
        ("target_type", "app".to_string()),
        (
          "content",
          m.to_string(),
        ),
      ])
      .send()?;

    Ok(())
  }
}
