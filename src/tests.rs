use rstest::{fixture, rstest};
const INPUT: &str =
  "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789.,?!@#$%^&*()[]{}<>/\'\"-_=+`~";

// rstest provides features to take common context into tests, and set up small cases testing
#[derive(Clone, Debug, Eq, PartialEq)]
struct Wb {
  string: String,
}

// context setup function to be implicitly called by `wb`
#[fixture]
fn string() -> String { return INPUT.to_string(); }

// context setup function to be implicitly called by `test_wb`
#[fixture]
fn wb(string: String) -> Wb {
  let _ = env_logger::builder().filter_level(log::LevelFilter::Debug).is_test(true).try_init();
  Wb { string }
}

#[rstest]
fn test_wb(wb: Wb) {
  log::info!("wb: {wb:?}");
  assert!(false);
}
