use std::fs;
use std::path::Path;

fn main() {
  println!("Let's start with december 2nd riddle");

  let contents = fs::read_to_string(Path::new("./riddle1.txt")).expect("Should have been able to read the file");

  let lines = contents.lines();

  println!("The sum is {}", lines);
}
