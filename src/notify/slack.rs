use std::error::Error;

use reqwest;
use serde_derive::{Deserialize, Serialize};

use crate::model::{self, Model};

#[derive(Debug, Serialize, Deserialize, Default)]
struct Payload {
  text: String,
  username: String,
  attachments: Vec<Attachment>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Attachment {
  color: String,
  fields: Vec<Field>,
  footer: String,
  footer_icon: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Field {
  title: String,
  value: String,
  short: bool,
}

pub struct Slack {
  hook_url: String,
}

impl Slack {
  pub fn new(hook_url: &str) -> Self {
    Self {
      hook_url: hook_url.to_string(),
    }
  }
}

impl super::Notify for Slack {
  fn notify(&self, m: Model) -> Result<(), Box<Error>> {
    let mut payload = Payload::default();

    let mut fields = vec![Field {
      title: "Game mode".into(),
      value: m.game_mode.clone(),
      short: true,
    }];

    fields.extend(m.attributes.iter().map(|a| match a {
      model::Attr::KeyValue(key, value) => Field {
        title: key.to_string(),
        value: value.to_string(),
        short: true,
      },
      model::Attr::Key(key) => Field {
        title: key.to_string(),
        value: "".into(),
        short: true,
      },
    }));

    if !m.team1and2_gods.is_empty() {
      let mut iter = m.team1and2_gods.iter();
      let v = iter
        .by_ref()
        .map(|id| m.gods.get(&id).unwrap().name.clone());
      fields.push(Field {
        title: "Team 1 and 2".into(),
        value: v.collect::<Vec<_>>().join(", "),
        short: true,
      });
    }

    if !m.team1_gods.is_empty() {
      let mut iter = m.team1_gods.iter();
      let v = iter
        .by_ref()
        .map(|id| m.gods.get(&id).unwrap().name.clone());
      fields.push(Field {
        title: "Team 1 only".into(),
        value: v.collect::<Vec<_>>().join(", "),
        short: true,
      });
    }

    if !m.team2_gods.is_empty() {
      let mut iter = m.team2_gods.iter();
      let v = iter
        .by_ref()
        .map(|id| m.gods.get(&id).unwrap().name.clone());
      fields.push(Field {
        title: "Team 2 only".into(),
        value: v.collect::<Vec<_>>().join(", "),
        short: true,
      });
    }

    payload.text = format!("*{}*\n{}", m.title, m.description);
    payload.username = "smite".into();
    payload.attachments = vec![Attachment {
      color: "#3db156".into(),
      footer: "Smite MOTD".into(),
      footer_icon: "https://platform.slack-edge.com/img/default_application_icon.png".into(),
      fields: fields,
    }];

    reqwest::Client::new()
      .post(&self.hook_url)
      .json(&payload)
      .send()?;

    Ok(())
  }
}
