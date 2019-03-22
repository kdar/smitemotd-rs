use std::error::Error;

use denv;

fn main() -> Result<(), Box<Error>> {
  for (k, v) in denv::build_env(".env")? {
    println!("cargo:rustc-env={}={}", k, v);
  }

  Ok(())
}