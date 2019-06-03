#[macro_use]
extern crate log;

mod api;
mod model;
mod types;

pub use api::Smite;
pub use model::Model;

pub trait Store {
  type Error;
  fn save_session_id(&mut self, sid: &str) -> Result<(), Self::Error>;
  fn load_session_id(&mut self) -> Result<Option<String>, Self::Error>;
}
