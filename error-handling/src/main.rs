#![allow(dead_code)]
#![allow(unused_variables)]

use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
  let file_path = "./hello-world.txt";
  // let result = read_file(file_path);
  // let result = read_file_shorter(file_path);
  let result = read_file_shortest(file_path);

  match result {
    Ok(content) => println!("Content of {} is {}", file_path, content),
    Err(err) => panic!("Error reading file {}: {}", file_path, err),
  };
}

fn read_file(file_path: &str) -> Result<String, io::Error> {
  let result = File::open(file_path);

  let mut file = match result {
    Ok(file) => file,
    Err(err) => match err.kind() {
      ErrorKind::NotFound => match File::create(file_path) {
        Ok(file) => file,
        Err(e) => return Err(e),
      },
      other => panic!("Error creating file {}: {}", file_path, other),
    },
  };

  let mut content = String::new();

  match file.read_to_string(&mut content) {
    Ok(_) => Ok(content),
    Err(err) => Err(err),
  }
}

fn read_file_shorter(file_path: &str) -> Result<String, io::Error> {
  let mut file = File::open(file_path)?;

  let mut content = String::new();

  file.read_to_string(&mut content)?;

  Ok(content)
}

fn read_file_shortest(file_path: &str) -> Result<String, io::Error> {
  let mut content = String::new();

  File::open(file_path)?.read_to_string(&mut content)?;

  Ok(content)
}
