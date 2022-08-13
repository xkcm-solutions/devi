use std::path::Path;
use std::fs::{create_dir_all, File, read_to_string};
use std::io::Write;

use home::home_dir;

static DEVI_DIR: &str = ".devi";

pub fn save_data_file(path: &str, data: &str) {
  let devi_dir_path = ensure_devi_dir();

  let filepath = Path::new(&devi_dir_path)
    .join(path);  
  
  create_dir_all(
    filepath
      .parent()
      .expect(&format!("Failed to get the parent of the {} path", filepath.to_str().unwrap()))
      .to_str()
      .unwrap()
  ).expect("");

  let file = File::create(filepath).expect("Failed to create file");
  write!(&file, "{}", data)
    .expect("Failed to save file data");
}

pub fn read_data_file(path: &str) -> String {
  let devi_dir_path = get_devi_dir_path();
  let filepath = Path::new(&devi_dir_path)
    .join(path);

  read_to_string(&filepath)
    .expect(&format!("Failed to read contents of file at {}", filepath.to_str().unwrap()))
}

fn ensure_devi_dir() -> String {
  let devi_dir_path = get_devi_dir_path();

  create_dir_all(&devi_dir_path)
    .expect(&format!("Failed to create {} path", &devi_dir_path));
  
  devi_dir_path
}

fn get_devi_dir_path() -> String {
  let home_dir_path_buf = home_dir()
    .expect("Home directory could not be obtained");
  let home_dir_path = home_dir_path_buf
    .to_str()
    .expect("Failed to parse home directory path");

  Path::new(home_dir_path)
    .join(DEVI_DIR)
    .to_str()
    .expect("Failed to parse devi directory path")
    .to_owned()
}