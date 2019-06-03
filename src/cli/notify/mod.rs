use std::error::Error;

pub mod email;
mod fmt;
pub mod pushbullet;
pub mod pushed;
pub mod slack;
pub mod stream;

pub trait Notify {
  fn notify(&self, m: &smitemotd::Model) -> Result<(), Box<Error>>;
}
