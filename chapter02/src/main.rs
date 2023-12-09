use std::fs;
use std::path::Path;
use regex::Regex;

struct Cube {
  red: u32,
  green: u32,
  blue: u32,
}

fn main() {
  let cube_count = Cube {
    red: 12,
    green: 13,
    blue: 14,
  };
  println!("Let's start with december 2nd riddle");

  let contents = fs::read_to_string(Path::new("./riddle.txt")).expect("Should have been able to read the file");

  let lines = contents.lines();

  let mut sum_stage1:u32 = 0;
  let mut sum_stage2:u64 = 0;

  for line in lines {
    let regex_for_game = Regex::new(r"Game (\d+)").unwrap();
    let regex_for_cubes = Regex::new(r"(\d+) (red|blue|green)").unwrap();
    let captures = regex_for_game.captures(line).unwrap();

    let mut invalid = false;
    let mut min_cubes = Cube {
      red: 0,
      green: 0,
      blue: 0,
    };

    for (_, [count, color]) in regex_for_cubes.captures_iter(line).map(|c| c.extract()) {
      let count = count.parse::<u32>().unwrap();
      if color == "red" {
        if min_cubes.red < count {
          min_cubes.red = count;
        }
        if count > cube_count.red as u32 {
          invalid = true;
        }
      } else if color == "green" {
        if min_cubes.green < count {
          min_cubes.green = count;
        }
        if count > cube_count.green as u32 {
          invalid = true;
        }
      } else if color == "blue" {
        if min_cubes.blue < count {
          min_cubes.blue = count;
        }
        if count > cube_count.blue as u32 {  
          invalid = true;
        }
      } 
    }
    let power = min_cubes.red as u64 * min_cubes.green as u64 * min_cubes.blue as u64;

    sum_stage2 += power;
    if invalid {
      continue;
    }
    sum_stage1 += captures[1].parse::<u32>().unwrap();
  
  }

  println!("Sum stage1: {}", sum_stage1);
  println!("Sum stage2: {}", sum_stage2);
}
