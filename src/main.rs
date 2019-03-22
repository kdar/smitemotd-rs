use std::error::Error;

use env_logger;
use lettre::smtp::authentication::Credentials;
use lettre::{EmailAddress, Envelope, SendableEmail, SmtpClient, Transport};
use reqwest;
use serde_json::json;

mod api;
mod models;
mod motd;

fn main() -> Result<(), Box<Error>> {
  env_logger::init();

  let mut smite = api::Smite::new(env!("SMITE_DEV_ID"), env!("SMITE_AUTH_KEY"));
  let motds = smite.get_motd()?;

  if motds.len() == 0 {
    println!("0 MOTDs returned");
    return Ok(());
  }

  // for entry in motd {
  //   println!("{}", entry.title.unwrap());
  //   println!("{}", entry.game_mode.unwrap());
  //   println!("{}", msg_format(&entry.description.unwrap()));
  //   break;
  // }

  // reqwest::Client::new()
  //   .post("https://api.pushed.co/1/push")
  //   .form(&[
  //     ("app_key", env!("PUSHED_APP_KEY")),
  //     ("app_secret", env!("PUSHED_APP_SECRET")),
  //     ("target_type", "app"),
  //     ("content", &format!("{}: {}", motd[0].title.as_ref().unwrap(), motd[0].description.as_ref().unwrap())),
  //   ])
  //   .send()?;

  // let resp: serde_json::Value = reqwest::Client::new()
  //   .post("https://api.pushbullet.com/v2/pushes")
  //   .header("Access-Token", env!("PB_ACCESS_TOKEN"))
  //   .json(&json!({
  //     "channel_tag": "smitemotd",
  //     "type": "note",
  //     "title": motd[0].title.as_ref().unwrap(),
  //     "body": "*hey*\n_there_\n:) ", //motd[0].description.as_ref().unwrap(),
  //   }))
  //   .send()?
  //   .json()?;

  // println!("{:?}", resp);

  let motd = motd::Motd::new(motds);

  // use slack_hook::{Slack, PayloadBuilder};
  // let slack = Slack::new(env!("SLACK_HOOK")).unwrap();
  // let p = PayloadBuilder::new()
  //   .text("test message")
  //   .channel("#testing")
  //   .username("My Bot")
  //   .icon_emoji(":chart_with_upwards_trend:")
  //   .build()
  //   .unwrap();

  // slack.send(&p)?;

//   let recipients = env!("MAIL_RECIPIENTS");
//   let recipients = recipients.split(",").map(|s| EmailAddress::new(s.to_string()).unwrap()).collect();

//   let email = SendableEmail::new(
//     Envelope::new(
//       Some(EmailAddress::new(env!("GMAIL_USERNAME").to_string()).unwrap()),
//       recipients,
//     )
//     .unwrap(),
//     "id".to_string(),
//     motd.pushover_html().into_bytes(),
//   );

//   let creds = Credentials::new(
//     env!("GMAIL_USERNAME").to_string(),
//     env!("GMAIL_PASSWORD").to_string(),
//   );

//   // Open a remote connection to gmail
//   let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
//     .unwrap()
//     .credentials(creds)
//     .transport();

//   // Send the email
//   mailer.send(email)?;

   Ok(())
}

// #[cfg(test)]
// mod tests {
//   use super::*;

//   #[test]
//   fn format() {
//     let f = msg_format("<li>If you can dodge a giant ball of death, you can dodge a ball! Nox is suited up and ready to throw down in Arena. Using only her Shadow Lock and Night Terror destroy the competition!<li>Nox Only</li><li>Starting Cooldown : 90%</li><li>Maximum Cooldown : 90%</li><li>Infinite Mana</li><li>Fountain Healing Disabled</li><li>Starting Level 5</li><li>Starting Gold 2500</li><li>Siphon Darkness / Shadow Step Disabled</li><li>Suggested by sureal8</li></li>");
//     assert_eq!(f, r#"If you can dodge a giant ball of death, you can dodge a ball! Nox is suited up and ready to throw down in Arena. Using only her Shadow Lock and Night Terror destroy the competition!
// Nox Only
// Starting Cooldown : 90%
// Maximum Cooldown : 90%
// Infinite Mana
// Fountain Healing Disabled
// Starting Level 5
// Starting Gold 2500
// Siphon Darkness / Shadow Step Disabled
// Suggested by sureal8"#)
//   }
// }