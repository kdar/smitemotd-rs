use std::error::Error;

pub mod slack;

pub trait Notify {
  fn notify(&self, m: crate::model::Model) -> Result<(), Box<Error>>;
}
