#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod cli;
mod error;
mod mappings;
#[cfg(test)] mod tests;
mod utils;

use error::MyError;
use log::info;

fn main() -> Result<(), MyError> {
  let my_cli = utils::setup()?;
  my_cli.handle();

  Ok(())
}
