use crate::models::{Motd as MotdModel};

fn msg_format(s: &str) -> String {
  let mut result = String::new();
  let mut s = s;
  if s.starts_with("<li>") {
    s = &s[4..s.len()-4];
  }

  while let Some(index) = s.find("<li>") {
    result.push_str(&s[0..index]);
    result.push_str("\n");
    s = &s[index+4..];

    if let Some(index2) = s.find("</li>") {
      result.push_str(&s[0..index2]);
      s = &s[index2+5..];
    }
  }
  
  result
}

enum Attr {
  KeyValue(String, String),
  Key(String),
}

pub struct Motd {
  game_mode: String,
  title: String,
  attributes: Vec<Attr>,
  description: String,
}

impl Motd {
  pub fn new(m: MotdModel) -> Motd {
    Motd {
      game_mode: "Arena_V3".to_string(),
      title: "Dodgeball".to_string(),
      attributes: vec![
        Attr::Key("Nox Only".to_string()),
        Attr::KeyValue("Starting Cooldown".to_string(), "90%".to_string()),
        Attr::KeyValue("Maximum Cooldown".to_string(), "90%".to_string()),
        Attr::Key("Infinite Mana".to_string()),
        Attr::Key("Fountain Healing Disabled".to_string()),
        Attr::Key("Starting Level 5".to_string()),
        Attr::Key("Starting Gold 2500".to_string()),
        Attr::Key("Siphon Darkness / Shadow Step Disabled".to_string()),
        Attr::Key("Suggested by sureal8".to_string()),
      ],
      description: "If you can dodge a giant ball of death, you can dodge a ball! Nox is suited up and ready to throw down in Arena. Using only her Shadow Lock and Night Terror destroy the competition!".to_string(),
    }
  }

  pub fn text(&self) -> String {
    "".to_string()
  }

  pub fn pushover_html(&self) -> String {
    let mut s = String::new();
    s.push_str(&format!("<b>{}</b>", self.title));
    s.push_str("\n");
    s.push_str(&format!("Game mode: {}", self.game_mode));
    s.push_str("\n");
    s.push_str(&self.description);   
    s.push_str("\n"); 
    for attr in &self.attributes {
      s.push_str("\n");
      match attr {
        Attr::KeyValue(key, value) => s.push_str(&format!("{}: {}", key, value)),
        Attr::Key(key) => s.push_str(&key),
      };
    }
    // format!(
    //   "{}\n{}\n{}",
    //   motd[0].title.as_ref().unwrap(),
    //   motd[0].game_mode.as_ref().unwrap(),
    //   msg_format(motd[0].description.as_ref().unwrap()),
    // )
    s
  }
}
