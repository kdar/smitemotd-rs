use std::error::Error;

use chrono::{DateTime, Utc};
use md5::{Digest, Md5};
use reqwest;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::types;
use crate::store;

const BASE_URL: &str = "http://api.smitegame.com/smiteapi.svc";
const INVALID_SESSION: &str = "Invalid session id.";

pub struct Smite {
  dev_id: String,
  auth_key: String,
  session_id: Option<String>,
  store: Box<store::Store>,
}

impl Smite {
  pub fn new(dev_id: &str, auth_key: &str, store: Box<store::Store>) -> Smite {
    Smite {
      dev_id: dev_id.to_string(),
      auth_key: auth_key.to_string(),
      store,
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

  fn api_call<T: DeserializeOwned>(&mut self, method: &str, params: &[&str]) -> Result<T, Box<Error>> {
    self.create_session(false)?;    

    let resp = loop {
      let ts = self.timestamp();
      let sig = self.signature(method, &ts);
      let mut url = format!(
        "{base_url}/{method}json/{dev_id}/{signature}/{session}/{timestamp}",
        base_url = BASE_URL,
        method = method,
        dev_id = self.dev_id,
        signature = sig,
        session = self.session_id.as_ref().unwrap(),
        timestamp = ts,
      );

      if params.len() > 0 {
        url = format!("{}/{}", url, params.join("/"));
      }

      let resp: Value = reqwest::Client::new()
        .get(&url)
        .send()?
        .json()?;

      if let Value::Array(v) = &resp {
        if v.len() == 1 {
          if let Value::Object(m) = &v[0] {
            if m.get("ret_msg") == Some(&Value::String(INVALID_SESSION.to_string())) {
              self.create_session(true)?;
              continue;
            }
          }
        }
      }

      break resp;
    };

    let resp = serde_json::from_value(resp)?;
    Ok(resp)
  }

  pub fn create_session(&mut self, force: bool) -> Result<(), Box<Error>> {
    if !force {
      if let Ok(Some(val)) = self.store.get_session_id() {
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

    let session: types::Session = reqwest::Client::new().get(&url).send()?.json()?;

    self
      .store
      .set_session_id(&session.session_id)
      .map_err(|e| format!("{}", e))?;
    self.session_id = Some(session.session_id);

    Ok(())
  }

  pub fn get_motd(&mut self) -> Result<types::Motds, Box<Error>> {
    let res = self.api_call("getmotd", &[])?;
    Ok(res)
  }

  pub fn get_gods(&mut self) -> Result<types::Gods, Box<Error>> {
    let res = self.api_call("getgods", &["1"])?;
    Ok(res)
  }
}
