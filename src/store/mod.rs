use std::error::Error;

pub mod pickledb;

pub trait Store {
  fn save_session_id(&mut self, sid: &str) -> Result<(), Box<Error>>;
  fn load_session_id(&self) -> Result<Option<String>, Box<Error>>;
}
