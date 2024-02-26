#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(non_snake_case)]
#![allow(clippy::clone_on_copy)]

mod cli;
mod error;
#[cfg(test)] mod tests;
mod utils;
mod mappings;
use error::MyError;
use log::info;

fn main() -> Result<(), MyError> {
  let _cli = utils::setup()?;
  _cli.handle();

  info!("hello thor");

  Ok(())
}
