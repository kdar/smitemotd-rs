use std::error::Error;
use std::io::{self, Write};
use std::cell::RefCell;
use std::fs::File;

use serde_derive::Deserialize;

use crate::model::Model;

#[derive(Deserialize, Debug)]
pub struct StreamOpts {
  stdout: Option<bool>,
  stderr: Option<bool>,
  file: Option<String>,
  color: Option<bool>,
}

pub struct Stream {
  w: RefCell<Box<Write>>,
  opts: StreamOpts,
}

impl Stream {
  pub fn new(opts: StreamOpts) -> Result<Self, Box<Error>> {
    let w = RefCell::new(if let Some(true) = opts.stdout {
      Box::new(io::stdout()) as Box<Write>
    } else if let Some(true) = opts.stderr {
      Box::new(io::stderr()) as Box<Write>
    } else if let Some(v) = &opts.file {
      Box::new(File::open(v)?) as Box<Write>
    } else {
      return Err("need one of the following options: stdout, stderr, file".into());
    });

    Ok(Self {
      opts,
      w,
    })
  }
}

impl super::Notify for Stream {
  fn notify(&self, m: &Model) -> Result<(), Box<Error>> {
    self.w.borrow_mut().write_all(m.to_string().as_bytes())?;

    Ok(())
  }
}
