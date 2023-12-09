use std::{fs, char};
use std::path::Path;
use regex::{Regex, Match};

fn check_if_character_is_symbol(character: char) -> bool {
  character != '.' && !character.is_numeric()
}


fn check_adjacent_symbol(content: &Vec<&str>, line_index: usize, capture: Match<'_> ) -> bool {
  // check line above
  if line_index > 0 {
    // check diagonal above left
    if capture.start() > 0 {
      if check_if_character_is_symbol(content[line_index-1].chars().nth(capture.start()-1).unwrap()) {
        // println!("diagonal above left");
        return true;
      }
    }

    // check direct above
    for index in capture.start()..capture.end() {
      if check_if_character_is_symbol(content[line_index-1].chars().nth(index).unwrap()) {
        // println!("direct above");
        return true;
      }
    }

    // check diagonal above right
    if capture.end() < content[line_index-1].len(){
      if check_if_character_is_symbol(content[line_index-1].chars().nth(capture.end()).unwrap()) {
        // println!("diagonal above right");
        return true;
      }
    }
  }

  // check left
  if capture.start() > 0 {
    if check_if_character_is_symbol(content[line_index].chars().nth(capture.start()-1).unwrap()) {
      // println!("left");
      return true;
    }
  }

  // check  right
  if capture.end() < content[line_index].len(){
    if check_if_character_is_symbol(content[line_index].chars().nth(capture.end()).unwrap()) {
      // println!("right");
      return true;
    }
  }


  // check line below
  if line_index < content.len() - 1 {
    // check diagonal below left
    if capture.start() > 0 {
      if check_if_character_is_symbol(content[line_index+1].chars().nth(capture.start()-1).unwrap()) {
        // println!("diagonal below left");
        return true;
      }
    }

    // check direct below
    for index in capture.start()..capture.end() {
      if check_if_character_is_symbol(content[line_index+1].chars().nth(index).unwrap()) {
        // println!("direct below");
        return true;
      }
    }

    // check diagonal below right
    if capture.end() < content[line_index+1].len(){
      if check_if_character_is_symbol(content[line_index+1].chars().nth(capture.end()).unwrap()) {
        // println!("diagonal below right");
        return true;
      }
    }
  }

  return false;
} 

fn main() {
  println!("Let's start with december 2nd riddle");

  let contents = fs::read_to_string(Path::new("./riddle.txt")).expect("Should have been able to read the file");
  let lines: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

  let lines_iter = contents.lines();

  let mut sum_stage1:u32 = 0;

  let regex = Regex::new(r"\d+").unwrap();
  for (line_index, line) in lines_iter.enumerate() {
    for capture in regex.captures_iter(line) {
      for capture in capture.iter() {
        if check_adjacent_symbol(&lines, line_index, capture.unwrap()) {
          // println!("Number {:?} is adjacent", capture);
          let number = capture.unwrap().as_str().parse::<u32>().unwrap();
          sum_stage1 += number;
        } else {
          // println!("Number {:?} is not adjacent", capture);
        }
      }
    }
  }

  println!("Sum of adjacent numbers: {}", sum_stage1)

}
