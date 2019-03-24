use std::collections::HashMap;
use std::error::Error;

use crate::types;

#[derive(Debug, Default)]
pub struct Model {
  pub game_mode: String,
  pub title: String,
  pub attributes: Vec<(String, Option<String>)>,
  pub description: String,
  pub gods: HashMap<i64, types::God>,
  pub team1_gods: Vec<i64>,
  pub team2_gods: Vec<i64>,
  pub team1and2_gods: Vec<i64>,
}

impl ToString for Model {
  fn to_string(&self) -> String {
    let mut s = String::new();
    s.push_str(&self.title);
    s.push_str("\n");    
    s.push_str(&self.description);
    s.push_str("\n\n");
    s.push_str(&format!("Game mode: {}", self.game_mode));

    for attr in &self.attributes {
      s.push_str("\n");
      match attr {
        (key, Some(value)) => s.push_str(&format!("{}: {}", key, value)),
        (key, None) => s.push_str(&key),
      };
    }

    if !self.team1and2_gods.is_empty() {
      s.push_str("\n");
      let mut iter = self.team1and2_gods.iter();
      let v = iter.by_ref().map(|id| self.gods.get(&id).unwrap().name.clone());
      s.push_str(&format!("Team 1 and 2: {}", v.collect::<Vec<_>>().join(", ")));
    }

    if !self.team1_gods.is_empty() {
      s.push_str("\n");
      let mut iter = self.team1_gods.iter();
      let v = iter.by_ref().map(|id| self.gods.get(&id).unwrap().name.clone());
      s.push_str(&format!("Team 1 only: {}", v.collect::<Vec<_>>().join(", ")));
    }

    if !self.team2_gods.is_empty() {
      s.push_str("\n");
      let mut iter = self.team2_gods.iter();
      let v = iter.by_ref().map(|id| self.gods.get(&id).unwrap().name.clone());
      s.push_str(&format!("Team 2 only: {}", v.collect::<Vec<_>>().join(", ")));
    }

    s
  }
}

pub fn parse(g: types::Gods, m: types::Motds) -> Result<Model, Box<Error>> {
  if m.len() == 0 {
    return Err("no motds".into());
  }

  let m = &m[0];

  let mut description = String::new();
  let mut attributes = vec![];

  let mut s = m.description.as_ref().unwrap().as_str();
  // Sometimes descriptions start with <li>. Not sure why, but
  // we just strip it.
  if s.starts_with("<li>") {
    s = &s[4..s.len() - 4];
  }

  if let Some(index) = s.find("<li>") {
    description.push_str(&s[0..index].trim());
    s = &s[index..];

    while let Some(index) = s.find("<li>") {
      if let Some(index2) = s.find("</li>") {
        let parts = s[index + 4..index2]
          .splitn(2, ":")
          .map(|s| s.trim())
          .collect::<Vec<&str>>();
        s = &s[index2 + 5..];
        if parts.len() == 2 {
          attributes.push((parts[0].to_string(), Some(parts[1].to_string())));
        } else {
          attributes.push((parts[0].to_string(), None));
        }
      }
    }
  } else {
    description.push_str(s.trim());
  }

  let mut gods = HashMap::new();
  for god in g {
    gods.insert(god.id, god);
  }

  let mut team1and2_gods = vec![];
  let mut team1_gods: Vec<_> = m
    .team1_gods_csv
    .clone()
    .unwrap_or_else(|| "".to_string())
    .split(", ")
    .filter(|v| *v != "")
    .map(|v| v.parse().unwrap())
    .collect();
  let mut team2_gods: Vec<_> = m
    .team2_gods_csv
    .clone()
    .unwrap_or_else(|| "".to_string())
    .split(", ")
    .filter(|v| *v != "")
    .map(|v| v.parse().unwrap())
    .collect();

  let mut team1_index = 0;
  let mut team2_index = 0;
  'outer: while team1_index < team1_gods.len() {
    while team2_index < team2_gods.len() {
      if team1_gods[team1_index] == team2_gods[team2_index] {
        team1and2_gods.push(team1_gods[team1_index]);
        team1_gods.remove(team1_index);
        team2_gods.remove(team2_index);
        team2_index = 0;
        continue 'outer;
      }

      team2_index += 1;
    }

    team1_index += 1;
    team2_index = 0;
  }

  Ok(Model {
    game_mode: m.game_mode.clone().unwrap_or_else(|| "".to_string()),
    title: m.title.clone().unwrap_or_else(|| "".to_string()),
    attributes: attributes,
    description: description,
    gods: gods,
    team1_gods: team1_gods,
    team2_gods: team2_gods,
    team1and2_gods: team1and2_gods,
  })
}
