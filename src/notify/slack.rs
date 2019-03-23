use std::error::Error;

use slack_hook::{Slack as SlackHook, PayloadBuilder, AttachmentBuilder, Field};

use crate::model::{self, Model};

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
    let slack = SlackHook::new(self.hook_url.as_str()).unwrap();

    let mut fields = vec![
      Field {
        title: "Game mode".to_string(),
        value: m.game_mode.clone().into(),
        short: Some(true),
      }
    ];

    fields.extend(m.attributes.iter().map(|a| {
      match a {
        model::Attr::KeyValue(key, value) => Field {
          title: key.to_string(),
          value: value.as_str().into(),
          short: Some(true),
        },
        model::Attr::Key(key) => Field {
          title: key.to_string(),
          value: "".into(),
          short: Some(true),
        }
      }
    }));

    if !m.team1and2_gods.is_empty() {
      let mut iter = m.team1and2_gods.iter();
      let v = iter.by_ref().map(|id| m.gods.get(&id).unwrap().name.clone());
      fields.push(Field {
        title: "Team 1 and 2".to_string(),
        value: v.collect::<Vec<_>>().join(", ").into(),
        short: Some(true),
      });
    }

    if !m.team1_gods.is_empty() {
      let mut iter = m.team1_gods.iter();
      let v = iter.by_ref().map(|id| m.gods.get(&id).unwrap().name.clone());
      fields.push(Field {
        title: "Team 1 only".to_string(),
        value: v.collect::<Vec<_>>().join(", ").into(),
        short: Some(true),
      });
    }

    if !m.team2_gods.is_empty() {
      let mut iter = m.team2_gods.iter();
      let v = iter.by_ref().map(|id| m.gods.get(&id).unwrap().name.clone());
      fields.push(Field {
        title: "Team 2 only".to_string(),
        value: v.collect::<Vec<_>>().join(", ").into(),
        short: Some(true),
      });
    }

    let attachments = vec![
      AttachmentBuilder::new("")
        .color("#3db156")
        .footer("Smite MOTD")
        .footer_icon("https://platform.slack-edge.com/img/default_application_icon.png")
        .fields(fields)
        .build()
        .unwrap(),
    ];

    let p = PayloadBuilder::new()
      .text(format!("*{}*\n{}", m.title, m.description))
      .username("smite")
      .attachments(attachments)
      .build()
      .unwrap();

    slack.send(&p)?;

    Ok(())
  }
}

