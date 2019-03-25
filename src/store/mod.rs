use std::error::Error;

pub mod pickledb;

pub trait Store {
  fn set_session_id(&mut self, sid: &str) -> Result<(), Box<Error>>;
  fn get_session_id(&self) -> Result<Option<String>, Box<Error>>;
}
