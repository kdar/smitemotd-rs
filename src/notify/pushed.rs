use std::error::Error;

use reqwest;

use crate::model::Model;

pub struct Pushed {
  app_key: String,
  app_secret: String,
}

impl Pushed {
  pub fn new(app_key: &str, app_secret: &str) -> Self {
    Self {
      app_key: app_key.to_string(),
      app_secret: app_secret.to_string(),
    }
  }
}

impl super::Notify for Pushed {
  fn notify(&self, m: Model) -> Result<(), Box<Error>> {
    reqwest::Client::new()
      .post("https://api.pushed.co/1/push")
      .form(&[
        ("app_key", self.app_key.clone()),
        ("app_secret", self.app_secret.clone()),
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
