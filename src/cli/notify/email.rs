use std::error::Error;

use lettre::smtp::authentication::Credentials;
use lettre::{EmailAddress, Envelope, SendableEmail, SmtpClient, Transport};
use serde_derive::Deserialize;
use uuid::Uuid;

use super::fmt;
use smitemotd::Model;

#[derive(Deserialize, Debug)]
pub struct EmailOpts {
  recipients: Vec<EmailAddress>,
  from: EmailAddress,
  subject: Option<String>,
  username: Option<String>,
  password: Option<String>,
  smtp: String,
}

pub struct Email {
  opts: EmailOpts,
  // from: EmailAddress,
  // recipients: Vec<EmailAddress>,
}

impl Email {
  pub fn new(opts: EmailOpts) -> Result<Self, Box<Error>> {
    // let mut recipients = vec![];
    // for rec in &opts.recipients {
    //   recipients.push(EmailAddress::new(rec)?);
    // }

    Ok(Self {
      opts, /*, recipients*/
    })
  }
}

impl super::Notify for Email {
  fn notify(&self, m: &Model) -> Result<(), Box<Error>> {
    let message_id = format!("<{}.smitemotd@localhost>", Uuid::new_v4());

    let email = SendableEmail::new(
      Envelope::new(Some(self.opts.from.clone()), self.opts.recipients.clone())?,
      message_id,
      fmt::format(m, false).into_bytes(),
    );

    let mut mailer = SmtpClient::new_simple(&self.opts.smtp)?;

    if let Some(username) = &self.opts.username {
      let creds = Credentials::new(
        username.to_string(),
        self.opts.password.clone().unwrap_or_else(|| "".to_string()),
      );

      mailer = mailer.credentials(creds);
    }

    let mut mailer = mailer.transport();
    mailer.send(email)?;

    Ok(())
  }
}
