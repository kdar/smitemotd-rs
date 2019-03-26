use std::error::Error;

pub mod slack;
pub mod pushed;
pub mod pushbullet;
pub mod stream;
pub mod email;
mod fmt;

pub trait Notify {
  fn notify(&self, m: &crate::model::Model) -> Result<(), Box<Error>>;
}
