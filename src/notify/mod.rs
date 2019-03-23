use std::error::Error;

pub mod slack;
pub mod pushed;

pub trait Notify {
  fn notify(&self, m: crate::model::Model) -> Result<(), Box<Error>>;
}
