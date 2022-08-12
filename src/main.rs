mod commands;

use std::str::FromStr;

use clap::{load_yaml, App};
use commands::Commands;

#[tokio::main]
async fn main() {
  let yaml = load_yaml!("cli-definition.yaml");
  let matches = App::from(yaml).get_matches();
  
  match matches.subcommand_name() {
    Some(subcommand_name) => {
      let args = matches.subcommand_matches(subcommand_name).unwrap();
      match Commands::from_str(subcommand_name).unwrap() {
        Commands::Postgres => {
          commands::postgres::postgres_cli(args).await
        }
      }
    }
    None => panic!("No command provided!")
  }
}
