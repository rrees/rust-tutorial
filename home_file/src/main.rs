use std::env;
use std::path::PathBuf;

fn main() {
    println!("Hello, world!");

    match env::home_dir() {
      Some(path) => print_dir(path),
      None => println!("Home directory could not be read")
    }
}

fn print_dir(directory_path: PathBuf) {
    println!("{}", directory_path.display())
}
