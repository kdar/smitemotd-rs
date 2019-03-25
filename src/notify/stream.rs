use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

use serde_derive::Deserialize;
use yansi::{Paint, Color, Style};

use crate::model::Model;

#[derive(Deserialize, Debug)]
pub struct StreamOpts {
  stdout: Option<bool>,
  stderr: Option<bool>,
  file: Option<String>,
  color: Option<bool>,
}

pub struct Stream {
  w: RefCell<Box<Write>>,
  opts: StreamOpts,
}

fn model_ansi(m: &Model) -> String {
  #[cfg(windows)]
  Paint::enable_windows_ascii();
  
  let key_style = Style::new(Color::Cyan).bold();
  let title_style = Style::new(Color::Blue).bold();

  let mut s = String::new();
  s.push_str(&title_style.paint(&m.title).to_string());
  s.push_str("\n");
  s.push_str(&m.description);
  s.push_str("\n\n");
  s.push_str(&format!("{}: {}", key_style.paint("Game mode"), m.game_mode));

  for attr in &m.attributes {
    s.push_str("\n");
    match attr {
      (key, Some(value)) => s.push_str(&format!("{}: {}", key_style.paint(key), value)),
      (key, None) => s.push_str(&key_style.paint(key).to_string()),
    };
  }

  if !m.team1and2_gods.is_empty() {
    s.push_str("\n");
    let mut iter = m.team1and2_gods.iter();
    let v = iter
      .by_ref()
      .map(|id| m.gods.get(&id).unwrap().name.clone());
    s.push_str(&format!(
      "{}: {}",
      key_style.paint("Team 1 and 2"),
      v.collect::<Vec<_>>().join(", ")
    ));
  }

  if !m.team1_gods.is_empty() {
    s.push_str("\n");
    let mut iter = m.team1_gods.iter();
    let v = iter
      .by_ref()
      .map(|id| m.gods.get(&id).unwrap().name.clone());
    s.push_str(&format!(
      "{}: {}",
      key_style.paint("Team 1 only"),
      v.collect::<Vec<_>>().join(", ")
    ));
  }

  if !m.team2_gods.is_empty() {
    s.push_str("\n");
    let mut iter = m.team2_gods.iter();
    let v = iter
      .by_ref()
      .map(|id| m.gods.get(&id).unwrap().name.clone());
    s.push_str(&format!(
      "{}: {}",
      key_style.paint("Team 2 only"),
      v.collect::<Vec<_>>().join(", ")
    ));
  }

  s.push_str("\n");

  s
}

impl Stream {
  pub fn new(opts: StreamOpts) -> Result<Self, Box<Error>> {
    let w = RefCell::new(if let Some(true) = opts.stdout {
      Box::new(io::stdout()) as Box<Write>
    } else if let Some(true) = opts.stderr {
      Box::new(io::stderr()) as Box<Write>
    } else if let Some(v) = &opts.file {
      Box::new(File::open(v)?) as Box<Write>
    } else {
      return Err("need one of the following options: stdout, stderr, file".into());
    });

    Ok(Self { opts, w })
  }
}

impl super::Notify for Stream {
  fn notify(&self, m: &Model) -> Result<(), Box<Error>> {
    let mut w = self.w.borrow_mut();
    if let Some(true) = self.opts.color {
      let s = model_ansi(m);
      w.write_all(s.as_bytes())?;
      // w.write(b"\n")?;
    } else {      
      w.write_all(m.to_string().as_bytes())?;
      // w.write(b"\n")?;
    }

    Ok(())
  }
}
