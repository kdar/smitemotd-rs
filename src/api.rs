use std::error::Error;

use chrono::{DateTime, Utc};
use md5::{Digest, Md5};
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use reqwest;

use crate::models::*;

const BASE_URL: &str = "http://api.smitegame.com/smiteapi.svc";

pub struct Smite {
  dev_id: String,
  auth_key: String,
  db: PickleDb,
  session_id: Option<String>,
}

impl Smite {
  pub fn new(dev_id: &str, auth_key: &str) -> Smite {
    let db = match PickleDb::load(
      "smitemotd.db",
      PickleDbDumpPolicy::AutoDump,
      SerializationMethod::Json,
    ) {
      Ok(v) => v,
      Err(_) => PickleDb::new(
        "smitemotd.db",
        PickleDbDumpPolicy::AutoDump,
        SerializationMethod::Json,
      ),
    };

    Smite {
      dev_id: dev_id.to_string(),
      auth_key: auth_key.to_string(),
      db: db,
      session_id: None,
    }
  }

  fn timestamp(&self) -> String {
    let dt: DateTime<Utc> = Utc::now();
    dt.format("%Y%m%d%H%M%S").to_string()
  }

  fn signature(&self, method: &str, timestamp: &str) -> String {
    let mut hasher = Md5::new();
    hasher.input(&self.dev_id);
    hasher.input(method);
    hasher.input(&self.auth_key);
    hasher.input(timestamp);
    let result = hasher.result();

    format!("{:x}", result)
  }

  pub fn create_session(&mut self, force: bool) -> Result<(), Box<Error>> {
    if !force {
      if let Some(val) = self.db.get::<String>("session_id") {
        self.session_id = Some(val);
        return Ok(());
      }
    }

    let method = "createsession";
    let ts = self.timestamp();
    let sig = self.signature(method, &ts);
    let url = format!(
      "{base_url}/{method}json/{dev_id}/{signature}/{timestamp}", 
      base_url = BASE_URL,
      method = method,
      dev_id = self.dev_id,
      signature = sig,
      timestamp = ts,
    );

    let session: Session = reqwest::Client::new()
      .get(&url)
      .send()?
      .json()?;

    self.db.set("session_id", &session.session_id).map_err(|e| format!("{}", e))?;
    self.session_id = Some(session.session_id);

    Ok(())
  }

  pub fn get_motd(&mut self) -> Result<Motd, Box<Error>> {
    self.create_session(false)?;

    let motd: Motd = loop {
      let method = "getmotd";
      let ts = self.timestamp();
      let sig = self.signature(method, &ts);
      let url = format!(
        "{base_url}/{method}json/{dev_id}/{signature}/{session}/{timestamp}", 
        base_url = BASE_URL,
        method = method,
        dev_id = self.dev_id,
        signature = sig,
        session = self.session_id.as_ref().unwrap(),
        timestamp = ts,
      );

      let motd: Motd = reqwest::Client::new()
        .get(&url)
        .send()?
        .json()?;
      
      if motd.len() == 1 {
        if let Some(ret_msg) = &motd[0].ret_msg {
          if ret_msg == "Invalid session id." {
            self.create_session(true)?;
            continue;
          }
        }
      }

      break motd;
    };

    Ok(motd)
  }
}