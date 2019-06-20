use std::collections::HashMap;
use std::error::Error;

use serde_derive::{Deserialize, Serialize};

use crate::types;

#[derive(Debug, Default, Serialize, Deserialize)]
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

impl Model {
  pub fn parse(g: types::Gods, m: types::Motds) -> Result<Model, Box<dyn Error>> {
    if m.is_empty() {
      return Err("no motds".into());
    }

    let m = &m[0];
    let s = m.description.as_ref().unwrap().as_str();

    // This may seem like a weird way of parsing the attributes, but the API
    // will sometimes return attributes with missing </li>. Because of this,
    // we only care about the start of each <li> since there are no nested
    // html elements returned.
    let mut attrs = s
      .split("<li>")
      .map(|v| v.replace("</li>", "").trim().to_string());
    let description = attrs.next().unwrap();

    let attributes = attrs
      .map(|v| {
        let parts = v.splitn(2, ':').map(str::trim).collect::<Vec<&str>>();
        if parts.len() == 2 {
          (parts[0].to_string(), Some(parts[1].to_string()))
        } else {
          (parts[0].to_string(), None)
        }
      })
      .collect();

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
      attributes,
      description,
      gods,
      team1_gods,
      team2_gods,
      team1and2_gods,
    })
  }

  pub fn get_god_name(&self, id: i64) -> String {
    match self.gods.get(&id) {
      Some(v) => v.name.clone(),
      None => format!("{}", id),
    }
  }
}
