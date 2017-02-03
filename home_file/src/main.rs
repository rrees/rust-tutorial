use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    println!("Hello, world!");

    match env::home_dir() {
      Some(path) => print_dir(path),
      None => println!("Home directory could not be read")
    }
}

fn print_dir(directory_path: PathBuf) {
    println!("{}", directory_path.display());
    if let Ok(entries) = fs::read_dir(directory_path.as_path()) {
        for entry in entries {
            if let Ok(entry) = entry {
                println!("{}", entry.path().display());
            }
        }
    }
}
