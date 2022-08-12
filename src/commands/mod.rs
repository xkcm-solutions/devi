pub mod postgres;

use std::str::FromStr;

pub enum Commands {
  Postgres
}

impl FromStr for Commands {
  type Err = ();
  fn from_str(s: &str) -> Result<Commands, Self::Err> {
    match s {
      "postgres" => Ok(Commands::Postgres),
      _ => Err(())
    }
  }
}