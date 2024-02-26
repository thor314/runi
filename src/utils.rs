use clap::Parser;

use crate::{cli::MyCli, error::MyError};
/// Set up crate logging and environment variables.
pub(crate) fn setup() -> Result<MyCli, MyError> {
  let my_cli = MyCli::parse();
  env_logger::builder().filter_level(my_cli.log_level()).init();
  Ok(my_cli)
}
