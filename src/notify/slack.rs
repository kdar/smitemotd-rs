use std::error::Error;

use slack_hook::{Slack as SlackHook, PayloadBuilder, AttachmentBuilder, Field};

use crate::motd::{self, Motd};

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
  fn notify(&self, m: Motd) -> Result<(), Box<Error>> {
    let slack = SlackHook::new(self.hook_url.as_str()).unwrap();

    let mut fields = vec![
      Field {
        title: "Game mode".to_string(),
        value: m.game_mode.into(),
        short: Some(false),
      }
    ];

    fields.extend(m.attributes.iter().map(|a| {
      match a {
        motd::Attr::KeyValue(key, value) => Field {
          title: key.to_string(),
          value: value.as_str().into(),
          short: Some(false),
        },
        motd::Attr::Key(key) => Field {
          title: key.to_string(),
          value: "".into(),
          short: Some(false),
        }
      }
    }));

    let p = PayloadBuilder::new()
      .text(format!("*{}*\n{}", m.title, m.description))
      .username("smite")
      .attachments(vec![
        AttachmentBuilder::new("")
          .color("#3db156")
          .footer("Smite MOTD")
          .footer_icon("https://platform.slack-edge.com/img/default_application_icon.png")
          .fields(fields)
          .build()
          .unwrap(),
      ])
      .build()
      .unwrap();

    slack.send(&p)?;

    Ok(())
  }
}

