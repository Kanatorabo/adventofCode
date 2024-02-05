use std::fs;
use std::path::Path;
use regex::Regex;

fn main() {
  println!("Let's start with december 2nd riddle");

  let contents = fs::read_to_string(Path::new("./riddle.txt")).expect("Should have been able to read the file");

  let lines_iter = contents.lines();

  let regex: Regex = Regex::new(r"\d+").unwrap();
  let mut sum_stage1: i32 = 0;
  let mut sum_stage2: i32 = 0;
  let mut card_queue: Vec<i32> = Vec::new();
  for (line_index, line) in lines_iter.enumerate() {

    let data = line.split("|").collect::<Vec<&str>>();
    let winning_str = data[0].split(":").collect::<Vec<&str>>();

    let winning_numbers:Vec<i64> = regex.find_iter(winning_str[1]).filter_map(|capture| capture.as_str().parse().ok()).collect();
    let actual_numbers:Vec<i64> = regex.find_iter(data[1]).filter_map(|capture| capture.as_str().parse().ok()).collect();

    let mut card_result:u32 = 0;

    for capture in actual_numbers {
      if winning_numbers.contains(&capture) {
        card_result += 1;
      }
    }

    if card_result > 0 {
      sum_stage1 += 2i32.pow(card_result-1);
    }

    let mut card_count: i32 = 1;
    if card_queue.len() != 0 {
      card_count = card_queue.remove(0) + 1;
    }
    println!("Card {}: {}", line_index, card_count + 1);
    sum_stage2 += card_count;

    for index in 0..card_result {
      if card_queue.len() >= index as usize + 1 {
        card_queue[index as usize] += card_count;
      } else {
        card_queue.push(card_count);
      }
    }
  }

  println!("Sum of points: {}", sum_stage1);
  println!("Sum of cards: {}", sum_stage2);

}
