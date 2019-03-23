use std::error::Error;
use std::io::Write;
use std::cell::RefCell;

use crate::model::Model;

pub struct Writer<T: Write> {
  w: RefCell<T>,
}

impl<T: Write> Writer<T> {
  pub fn new(w: T) -> Self {
    Self {
      w: RefCell::new(w),
    }
  }
}

impl<T: Write> super::Notify for Writer<T> {
  fn notify(&self, m: Model) -> Result<(), Box<Error>> {
    self.w.borrow_mut().write_all(m.to_string().as_bytes())?;

    Ok(())
  }
}
