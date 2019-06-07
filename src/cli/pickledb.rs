use std::error::Error;
use std::path::Path;

use pickledb::{PickleDb as Db, PickleDbDumpPolicy, SerializationMethod};

use smitemotd::Store;

pub struct PickleDb {
  db: Db,
}

impl PickleDb {
  pub fn new<P: AsRef<Path>>(dir: P) -> Self {
    let p = dir.as_ref().join("pickle.db");
    let db = match Db::load(&p, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json) {
      Ok(v) => v,
      Err(_) => Db::new(&p, PickleDbDumpPolicy::AutoDump, SerializationMethod::Json),
    };

    PickleDb { db }
  }
}

impl Store for PickleDb {
  type Error = Box<Error>;

  fn save_session_id(&mut self, sid: &str) -> Result<(), Self::Error> {
    self
      .db
      .set("session_id", &sid)
      .map_err(|e| format!("{}", e))?;

    Ok(())
  }

  fn load_session_id(&mut self) -> Result<Option<String>, Self::Error> {
    let v = self.db.get::<String>("session_id");
    Ok(v)
  }
}
