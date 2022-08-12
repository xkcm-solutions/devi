use clap::ArgMatches;

use core::panic;
use std::process::Command;
use std::fs::{File, read_to_string};
use std::io::Write;

use colored::Colorize;

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
  let docker_image = "postgres";

  let output = Command::new("docker")
    .arg("run")
    .args(["--name", "devi_local-postgres-server"])
    .args(["-e", format!("POSTGRES_PASSWORD={}", password).as_str()])
    .args(["-e", format!("-e POSTGRES_USER={}", user).as_str()])
    .args(["-v", "devi_postgres-data:/var/lib/postgresql/data"])
    .args(["-d", "--rm", "-p", format!("{0}:{0}", port).as_str()])
    .arg(docker_image)
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
  let mut container_id_file = File::create("./.container_id").unwrap();
  write!(&mut container_id_file, "{}", container_id).unwrap();
}

fn read_container_id() -> String {
  let container_id = read_to_string("./.container_id").unwrap();
  if container_id == "" {
    panic!("Docker container ID is unknown, can't stop the container")
  }
  container_id
}