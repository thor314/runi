//! https://docs.rs/clap/latest/clap/

use clap::{ArgAction, CommandFactory, Parser, Subcommand};
use log::{trace, LevelFilter};

use crate::mappings::*;
// The subcommand handler.
// If no subcommand is provided, the handler will short to the logic for the default command.
// This struct probably doesn't need to change, make changes to `Subcommands` and the individual
// subcommands instead.
#[derive(Parser, Debug)]
#[command(name = "runi")]
#[command(bin_name = "runi")]
#[clap(about = "a CLI tool to generate unicode fonts")]
#[command(author, version)]
#[command(propagate_version = true)]
pub struct MyCli {
  #[command(subcommand)]
  subcommands:   Subcommands,
  /// Set the verbosity. Use -v for DEBUG, -vv for TRACE. None for INFO.
  #[arg(long = "verbose", short = 'v', action = ArgAction::Count)]
  pub verbosity: u8,
  /// Generate shell completions, using doc strings for subcommand hints
  #[arg(short = 'g', long = "generate", value_enum)]
  generator:     Option<clap_complete::Shell>,
}

impl MyCli {
  pub fn handle(&self) {
    if let Some(generator) = self.generator {
      let mut cmd = Self::command();
      eprintln!("Generating completion file for {generator:?}...");
      Self::print_completions(generator, &mut cmd);
      return;
    }

    self.subcommands.handle();
  }

  fn print_completions<G: clap_complete::Generator>(gen: G, cmd: &mut clap::Command) {
    clap_complete::generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
  }

  /// in decreasing order of priority:
  /// if verbosity is specified from command line, e.g. `-v` or `-vv`, use that
  /// if a `RUST_LOG` env var is set, use that
  /// else, use INFO
  pub fn log_level(&self) -> LevelFilter {
    if self.verbosity > 0 {
      match self.verbosity {
        1 => LevelFilter::Debug,
        _ => LevelFilter::Trace,
      }
    } else if let Ok(s) = std::env::var("RUST_LOG") {
      s.parse().expect("RUST_LOG environment invalid value")
    } else {
      LevelFilter::Info
    }
  }
}

/// CLI parser with subcommands
/// The subcommands for this CLI.
/// Add subcommands as demonstrated.
#[derive(Debug, Subcommand)]
enum Subcommands {
  Superscript { s: String },
  Subscript { s: String },
  Script { s: String },
  ScriptBold { s: String },
  Fullwidth { s: String },
  Gothic { s: String },
  GothicBold { s: String },
  Sans { s: String },
  SansItalic { s: String },
  Monospace { s: String },
  SansBold { s: String },
  SansBoldItalic { s: String },
  SerifBold { s: String },
  SmallCaps { s: String },
  Circled { s: String },
  CircledNegative { s: String },
  Squared { s: String },
  SquaredNegative { s: String },
  DoubleStruck { s: String },
  Inverted { s: String },
  Reversed { s: String },
  FauxCyrillic { s: String },
}

impl Subcommands {
  /// delegate handling to each subcommand
  pub fn handle(&self) -> String {
    trace!("handling subcommands...");
    let s: String = match self {
      Subcommands::Superscript { s } =>
        s.chars().filter_map(|c| SUPERSCRIPT.get(&c).copied()).collect::<String>(),
      Subcommands::Subscript { s } =>
        s.chars().filter_map(|c| SUBSCRIPT.get(&c).copied()).collect::<String>(),
      Subcommands::Script { s } =>
        s.chars().filter_map(|c| SCRIPT.get(&c).copied()).collect::<String>(),
      Subcommands::ScriptBold { s } =>
        s.chars().filter_map(|c| SCRIPT_BOLD.get(&c).copied()).collect::<String>(),
      Subcommands::Fullwidth { s } =>
        s.chars().filter_map(|c| FULLWIDTH.get(&c).copied()).collect::<String>(),
      Subcommands::Gothic { s } =>
        s.chars().filter_map(|c| GOTHIC.get(&c).copied()).collect::<String>(),
      Subcommands::GothicBold { s } =>
        s.chars().filter_map(|c| GOTHIC_BOLD.get(&c).copied()).collect::<String>(),
      Subcommands::Sans { s } =>
        s.chars().filter_map(|c| SANS.get(&c).copied()).collect::<String>(),
      Subcommands::SansItalic { s } =>
        s.chars().filter_map(|c| SANS_ITALIC.get(&c).copied()).collect::<String>(),
      Subcommands::Monospace { s } =>
        s.chars().filter_map(|c| MONOSPACE.get(&c).copied()).collect::<String>(),
      Subcommands::SansBold { s } =>
        s.chars().filter_map(|c| SANS_BOLD.get(&c).copied()).collect::<String>(),
      Subcommands::SansBoldItalic { s } =>
        s.chars().filter_map(|c| SANS_BOLD_ITALIC.get(&c).copied()).collect::<String>(),
      Subcommands::SerifBold { s } =>
        s.chars().filter_map(|c| SERIF_BOLD.get(&c).copied()).collect::<String>(),
      Subcommands::SmallCaps { s } =>
        s.chars().filter_map(|c| SMALL_CAPS.get(&c).copied()).collect::<String>(),
      Subcommands::Circled { s } =>
        s.chars().filter_map(|c| CIRCLED.get(&c).copied()).collect::<String>(),
      Subcommands::CircledNegative { s } =>
        s.chars().filter_map(|c| CIRCLED_NEGATIVE.get(&c).copied()).collect::<String>(),
      Subcommands::Squared { s } =>
        s.chars().filter_map(|c| SQUARED.get(&c).copied()).collect::<String>(),
      Subcommands::SquaredNegative { s } =>
        s.chars().filter_map(|c| SQUARED_NEGATIVE.get(&c).copied()).collect::<String>(),
      Subcommands::DoubleStruck { s } =>
        s.chars().filter_map(|c| DOUBLE_STRUCK.get(&c).copied()).collect::<String>(),
      Subcommands::Inverted { s } =>
        s.chars().filter_map(|c| INVERTED.get(&c).copied()).collect::<String>(),
      Subcommands::Reversed { s } =>
        s.chars().filter_map(|c| REVERSED.get(&c).copied()).collect::<String>(),
      Subcommands::FauxCyrillic { s } =>
        s.chars().filter_map(|c| FAUX_CYRILLIC.get(&c).copied()).collect::<String>(),
    };

    println!("{}", s);
    s
  }
}

/// delegate handling to each subcommand
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_handle() {
    let input = "test"; // Example input
    let expected = "ᵗᵉˢᵗ";

    let command = Subcommands::Superscript { s: input.into() };
    let result = command.handle(); // Ensure `handle` returns the transformed string instead of printing it

    assert_eq!(result, expected, "Superscript transformation failed");
  }
}
