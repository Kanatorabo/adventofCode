use std::{fs, char};
use std::path::Path;
use std::collections::HashMap;
use regex::{Regex, Match};

fn check_if_character_is_symbol(character: char) -> bool {
  character != '.' && !character.is_numeric()
}

fn check_if_character_is_asterisk(character: char) -> bool {
  character == '*'
}

fn add_number_to_asterisks(x:usize, y:usize, capture: Match<'_>, asterisks_number: &mut HashMap<String, Vec<i32>>) {
  let key: String = format!("{}:{}", x, y);
  let number = capture.as_str().parse::<i32>().unwrap();

  if !asterisks_number.contains_key(&key) {
    asterisks_number.insert(key, vec![number]);
  } else {
    let mut v = asterisks_number.get(&key).unwrap().to_vec();
    v.push(number);
    asterisks_number.insert(key, v) ;
  }

}

fn check_adjacent_symbol(content: &Vec<&str>, line_index: usize, capture: Match<'_>, asterisks_number: &mut HashMap<String, Vec<i32>>) -> bool {
  let mut result = false;
  // check line above
  if line_index > 0 {
    // check diagonal above left
    if capture.start() > 0 {
      if check_if_character_is_symbol(content[line_index-1].chars().nth(capture.start()-1).unwrap()) {
        if check_if_character_is_asterisk(content[line_index-1].chars().nth(capture.start()-1).unwrap()) {
          add_number_to_asterisks(line_index-1, capture.start()-1, capture, asterisks_number);
          // println!("diagonal above left {:?}", asterisks_number)
        }
        // println!("diagonal above left");
        result = true;
      }
    }

    // check direct above
    for index in capture.start()..capture.end() {
      if check_if_character_is_symbol(content[line_index-1].chars().nth(index).unwrap()) {
        if check_if_character_is_asterisk(content[line_index-1].chars().nth(index).unwrap()) {
          add_number_to_asterisks(line_index-1, index, capture, asterisks_number);
          // println!("diagonal above left {:?}", asterisks_number)
        }
        // println!("direct above");
        result = true;
      }
    }

    // check diagonal above right
    if capture.end() < content[line_index-1].len(){
      if check_if_character_is_symbol(content[line_index-1].chars().nth(capture.end()).unwrap()) {
        if check_if_character_is_asterisk(content[line_index-1].chars().nth(capture.end()).unwrap()) {
          add_number_to_asterisks(line_index-1, capture.end(), capture, asterisks_number);
          // println!("diagonal above left {:?}", asterisks_number)
        }
        // println!("diagonal above right");
        result = true;
      }
    }
  }

  // check left
  if capture.start() > 0 {
    if check_if_character_is_symbol(content[line_index].chars().nth(capture.start()-1).unwrap()) {
      if check_if_character_is_asterisk(content[line_index].chars().nth(capture.start()-1).unwrap()) {
        add_number_to_asterisks(line_index, capture.start()-1, capture, asterisks_number);
        // println!("diagonal above left {:?}", asterisks_number)
      }
      // println!("left");
      result = true;
    }
  }

  // check  right
  if capture.end() < content[line_index].len(){
    if check_if_character_is_symbol(content[line_index].chars().nth(capture.end()).unwrap()) {
      if check_if_character_is_asterisk(content[line_index].chars().nth(capture.end()).unwrap()) {
        add_number_to_asterisks(line_index, capture.end(), capture, asterisks_number);
        // println!("diagonal above left {:?}", asterisks_number)
      }
      // println!("right");
      result = true;
    }
  }


  // check line below
  if line_index < content.len() - 1 {
    // check diagonal below left
    if capture.start() > 0 {
      if check_if_character_is_symbol(content[line_index+1].chars().nth(capture.start()-1).unwrap()) {
        if check_if_character_is_asterisk(content[line_index+1].chars().nth(capture.start()-1).unwrap()) {
          add_number_to_asterisks(line_index+1, capture.start()-1, capture, asterisks_number);
          // println!("diagonal above left {:?}", asterisks_number)
        }
        // println!("diagonal below left");
        return true;
      }
    }

    // check direct below
    for index in capture.start()..capture.end() {
      if check_if_character_is_symbol(content[line_index+1].chars().nth(index).unwrap()) {
        if check_if_character_is_asterisk(content[line_index+1].chars().nth(index).unwrap()) {
          add_number_to_asterisks(line_index+1, index, capture, asterisks_number);
          // println!("diagonal above left {:?}", asterisks_number)
        }
        // println!("direct below");
        return true;
      }
    }

    // check diagonal below right
    if capture.end() < content[line_index+1].len(){
      if check_if_character_is_symbol(content[line_index+1].chars().nth(capture.end()).unwrap()) {
        if check_if_character_is_asterisk(content[line_index+1].chars().nth(capture.end()).unwrap()) {
          add_number_to_asterisks(line_index+1, capture.end(), capture, asterisks_number);
          // println!("diagonal above left {:?}", asterisks_number)
        }
        // println!("diagonal below right");
        result = true;
      }
    }
  }

  return result;
} 


fn main() {
  println!("Let's start with december 2nd riddle");

  let contents = fs::read_to_string(Path::new("./riddle.txt")).expect("Should have been able to read the file");
  let lines: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

  let lines_iter = contents.lines();

  let mut sum_stage1:u32 = 0;

  let regex: Regex = Regex::new(r"\d+").unwrap();
  let mut asterisks_number: HashMap<String, Vec<i32>> = HashMap::new();
  for (line_index, line) in lines_iter.enumerate() {
    for capture in regex.captures_iter(line) {
      for capture in capture.iter() {
        if check_adjacent_symbol(&lines, line_index, capture.unwrap(), &mut asterisks_number) {
          // println!("Number {:?} is adjacent", capture);
          let number = capture.unwrap().as_str().parse::<u32>().unwrap();
          sum_stage1 += number;
        } else {
          // println!("Number {:?} is not adjacent", capture);
        }
      }
    }
  }

  let mut sum_stage2: i32 = 0;
  for (_key, value) in asterisks_number.iter() {
    if value.len() == 2 {
      let sum: i32 = value[0] * value[1];
      sum_stage2 = sum_stage2 + sum;
    }
  }

  println!("Sum of adjacent numbers: {}", sum_stage1);
  println!("Sum of multiplied numbers on adjacent *: {}", sum_stage2)

}
