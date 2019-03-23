use std::error::Error;

use crate::types;

#[derive(Debug)]
pub enum Attr {
  KeyValue(String, String),
  Key(String),
}

#[derive(Debug, Default)]
pub struct Model {
  pub game_mode: String,
  pub title: String,
  pub attributes: Vec<Attr>,
  pub description: String,
}

pub fn parse(g: types::Gods, m: types::Motds) -> Result<Model, Box<Error>> {
  if m.len() == 0 {
    return Err("no motds".into());
  }

  let m = &m[0];

  let mut model = Model::default();
  let mut description = String::new();
  let mut attributes = vec![];

  let mut s = m.description.as_ref().unwrap().as_str();
  // Sometimes descriptions start with <li>. Not sure why, but 
  // we just strip it.
  if s.starts_with("<li>") {
    s = &s[4..s.len()-4];
  }
  
  if let Some(index) = s.find("<li>") {
    description.push_str(&s[0..index].trim());
    s = &s[index..];

    while let Some(index) = s.find("<li>") {
      if let Some(index2) = s.find("</li>") {
        let parts = s[index+4..index2].splitn(2, ":").map(|s| s.trim()).collect::<Vec<&str>>();
        s = &s[index2+5..];
        if parts.len() == 2 {
          attributes.push(Attr::KeyValue(parts[0].to_string(), parts[1].to_string()));
        } else {
          attributes.push(Attr::Key(parts[0].to_string()));
        }
      }
    }
  } else {
    description.push_str(s.trim());
  }

  Ok(Model {
    game_mode: m.game_mode.clone().unwrap_or("".to_string()),
    title: m.title.clone().unwrap_or("".to_string()),
    attributes: attributes,
    description: description,
  })
}

// pub fn pushover_html(&self) -> String {
//     let mut s = String::new();
//     s.push_str(&format!("<b>{}</b>", self.title));
//     s.push_str("\n");
//     s.push_str(&format!("Game mode: {}", self.game_mode));
//     s.push_str("\n");
//     s.push_str(&self.description);   
//     s.push_str("\n"); 
//     for attr in &self.attributes {
//       s.push_str("\n");
//       match attr {
//         Attr::KeyValue(key, value) => s.push_str(&format!("{}: {}", key, value)),
//         Attr::Key(key) => s.push_str(&key),
//       };
//     }
//     // format!(
//     //   "{}\n{}\n{}",
//     //   motd[0].title.as_ref().unwrap(),
//     //   motd[0].game_mode.as_ref().unwrap(),
//     //   msg_format(motd[0].description.as_ref().unwrap()),
//     // )
//     s
//   }
