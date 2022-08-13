use crate::utils::{read_data_file, save_data_file};

use core::panic;
use std::process::Command;

use clap::ArgMatches;
use colored::Colorize;

static CONTAINER_ID_FILEPATH: &str = "postgres/.container_id";
static DOCKER_IMAGE: &str = "postgres";

pub async fn postgres_cli<'a>(args: &ArgMatches<'a>) {
  match args.subcommand_name() {
    Some(subcommand_name) => match subcommand_name {
      "start" => start().await,
      "stop" => stop().await,
      _ => {}
    }
    None => {}
  }
}

async fn start() {
  let password = "root";
  let user = "root";
  let port: i16 = 5432;

  let output = Command::new("docker")
    .arg("run")
    .args(["--name", "devi_local-postgres-server"])
    .args(["-e", &format!("POSTGRES_PASSWORD={}", password)])
    .args(["-e", &format!("POSTGRES_USER={}", user)])
    .args(["-v", "devi_postgres-data:/var/lib/postgresql/data"])
    .args(["-d", "--rm", "-p", format!("{0}:{0}", port).as_str()])
    .arg(DOCKER_IMAGE)
    .output()
    .expect("Failed to start the PostgreSQL server Docker container!");

  match output.status.code() {
    Some(code) => {
      let stdout = String::from_utf8(output.stdout).unwrap();
      let stderr = String::from_utf8(output.stderr).unwrap();
      match code {
        0 => {
          let container_id = stdout.trim();
          save_container_id(container_id);
          println!("{}", format!("Docker container ID = {}", container_id.bold()))
        }
        _ => {
          panic!("Docker exited with non-zero status\nstdout:\n{}\nstderr:\n{}", stdout, stderr);
        }
      }
    }
    None => {}
  }

  println!("{}", "PostgreSQL server running".green());
}

async fn stop() {
  let container_id = read_container_id();

  Command::new("docker")
    .arg("container")
    .arg("stop")
    .arg(container_id)
    .output()
    .expect("Failed to stop the PostgreSQL server Docker container");

  save_container_id("");

  println!("{}", "PostgreSQL server stopped".green())
}

fn save_container_id(container_id: &str) {
  save_data_file(CONTAINER_ID_FILEPATH, container_id);
}

fn read_container_id() -> String {
  let container_id = read_data_file(CONTAINER_ID_FILEPATH);
  if container_id == "" {
    panic!("Docker container ID is unknown, can't stop the container")
  }
  container_id
}