use std::path::Path;
use std::error::Error;

use pickledb::{PickleDb as Db, PickleDbDumpPolicy, SerializationMethod};

pub struct PickleDb {
  db: Db,
}

impl PickleDb {
  pub fn new<P: AsRef<Path>>(dir: P) -> Self {
    let p = dir.as_ref().join("pickle.db");
    let db = match Db::load(
      &p,
      PickleDbDumpPolicy::AutoDump,
      SerializationMethod::Json,
    ) {
      Ok(v) => v,
      Err(_) => Db::new(
        &p,
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
      ),
    };

    PickleDb{
      db,
    }
  }
}

impl super::Store for PickleDb {
  fn set_session_id(&mut self, sid: &str) -> Result<(), Box<Error>> {
    self
      .db
      .set("session_id", &sid)
      .map_err(|e| format!("{}", e))?;

    Ok(())
  }

  fn get_session_id(&self) -> Result<Option<String>, Box<Error>> {
   let v = self.db.get::<String>("session_id");
   Ok(v)
  }
}
