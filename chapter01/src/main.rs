use std::fs;
use std::path::Path;

use either::Either;


fn find_first_occurrence_of_number(line: &str, rev: bool) -> Option<u32> {
  let one: Option<usize>;
  let two: Option<usize>;
  let three: Option<usize>;
  let four: Option<usize>;
  let five: Option<usize>;
  let six: Option<usize>;
  let seven: Option<usize>;
  let eight: Option<usize>;
  let nine: Option<usize>;

  if rev {
    one = line.chars().collect::<String>().rfind("one");
    two = line.chars().collect::<String>().rfind("two");
    three = line.chars().collect::<String>().rfind("three");
    four = line.chars().collect::<String>().rfind("four");
    five = line.chars().collect::<String>().rfind("five");
    six = line.chars().collect::<String>().rfind("six");
    seven = line.chars().collect::<String>().rfind("seven");
    eight = line.chars().collect::<String>().rfind("eight");
    nine = line.chars().collect::<String>().rfind("nine");
  } else {
    one = line.chars().collect::<String>().find("one");
    two = line.chars().collect::<String>().find("two");
    three = line.chars().collect::<String>().find("three");
    four = line.chars().collect::<String>().find("four");
    five = line.chars().collect::<String>().find("five");
    six = line.chars().collect::<String>().find("six");
    seven = line.chars().collect::<String>().find("seven");
    eight = line.chars().collect::<String>().find("eight");
    nine = line.chars().collect::<String>().find("nine");
  }

  let mut current = one;

  if current== None || two != None && (current > two) ^ rev {
    current = two;
  } 
  if current== None || three != None && (current > three) ^ rev {
    current = three;
  } 
  if current== None || four != None && (current > four) ^ rev {
    current = four;
  } 
  if current== None || five != None && (current > five) ^ rev {
    current = five;
  } 
  if current== None || six != None && (current > six) ^ rev {
    current = six;
  } 
  if current== None || seven != None && (current > seven) ^ rev {
    current = seven;
  } 
  if current== None || eight != None && (current > eight) ^ rev {
    current = eight;
  } 
  if current== None || nine != None && (current > nine) ^ rev {
    current = nine;
  } 

  let mut forward_number_index = None;
    let mut forward_number: Option<u32> = None;



    let iter = if rev { 
        Either::Left(line.chars().rev()) 
    }
    else { 
        Either::Right(line.chars()) 
    };

    for character in iter.enumerate() {
      if forward_number_index == None {
        forward_number_index = Some(0);
      } else {
        forward_number_index = Some(forward_number_index.unwrap() + 1);
      }

      if character.1.is_numeric() {
        forward_number = character.1.to_digit(10);
        break;
      }
    }

    let forward_number_index_reversed = line.len() - 1 - forward_number_index.unwrap();

    if current == None && forward_number_index == None {
      return None;
    } else if current == None || (!rev && current > forward_number_index || rev && current < Some(forward_number_index_reversed)) {
      return forward_number;
    } else if current == one {
      return Some(1);
    } else if current == two {
      return Some(2);
    } else if current == three {
      return Some(3);
    } else if current == four {
      return Some(4);
    } else if current == five {
      return Some(5);
    } else if current == six {
      return Some(6);
    } else if current == seven {
      return Some(7);
    } else if current == eight {
      return Some(8);
    } else if current == nine {
      return Some(9);
    } else {
      return None;
    }
}

fn main() {
  println!("Let's start with december 1st riddle");

  let contents = fs::read_to_string(Path::new("./riddle1.txt")).expect("Should have been able to read the file");

  let lines = contents.lines();

  let mut sum = 0;

  for line in lines { 
    let mut number_string = String::from("");

    
    let first_number = find_first_occurrence_of_number(line, false);
    let last_number = find_first_occurrence_of_number(line, true);

    if first_number == None || last_number == None {
      continue;
    }

    number_string.push_str(&first_number.unwrap().to_string());
    number_string.push_str(&last_number.unwrap().to_string());


    sum += number_string.parse::<i32>().unwrap();
  }

  println!("The sum is: {sum}");

}
