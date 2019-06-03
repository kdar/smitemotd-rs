use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

use serde_derive::Deserialize;

use super::fmt;
use smitemotd::Model;

#[derive(Deserialize, Debug, Default)]
pub struct StreamOpts {
  pub stdout: Option<bool>,
  pub stderr: Option<bool>,
  pub file: Option<String>,
  pub color: Option<bool>,
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
      Box::new(File::create(v)?) as Box<Write>
    } else {
      return Err("need one of the following options: stdout, stderr, file".into());
    });

    Ok(Self { opts, w })
  }
}

impl super::Notify for Stream {
  fn notify(&self, m: &Model) -> Result<(), Box<Error>> {
    let mut w = self.w.borrow_mut();
    if let Some(true) = self.opts.color {
      let s = fmt::format(m, self.opts.color.unwrap_or(false));
      w.write_all(s.as_bytes())?;
    // w.write(b"\n")?;
    } else {
      w.write_all(fmt::format(&m, false).as_bytes())?;
      // w.write(b"\n")?;
    }

    Ok(())
  }
}
