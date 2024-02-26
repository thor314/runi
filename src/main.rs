pub mod cli;
mod error;
mod mappings;
mod utils;

use error::MyError;

fn main() -> Result<(), MyError> {
  let my_cli = utils::setup()?;
  my_cli.handle();

  Ok(())
}
