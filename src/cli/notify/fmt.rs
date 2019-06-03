use yansi::{Color, Style};

use smitemotd::Model;

pub fn format(m: &Model, color: bool) -> String {
  let (key_style, title_style) = if color {
    #[cfg(windows)]
    {
      use yansi::Paint;
      Paint::enable_windows_ascii();
    }

    let key_style = Style::new(Color::Cyan).bold();
    let title_style = Style::new(Color::Blue).bold();

    (key_style, title_style)
  } else {
    let s = Style::new(Color::Unset);
    (s, s)
  };

  let mut s = String::new();
  s.push_str(&title_style.paint(&m.title).to_string());
  s.push_str("\n");
  s.push_str(&m.description);
  s.push_str("\n\n");
  s.push_str(&format!(
    "{}: {}",
    key_style.paint("Game mode"),
    m.game_mode
  ));

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
