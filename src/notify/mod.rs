use std::error::Error;

pub mod slack;

pub trait Notify {
  fn notify(&self, m: crate::motd::Motd) -> Result<(), Box<Error>>;
}
