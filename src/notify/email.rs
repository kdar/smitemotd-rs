// use lettre::smtp::authentication::Credentials;
// use lettre::{EmailAddress, Envelope, SendableEmail, SmtpClient, Transport};

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