use std::fs;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Welcome to Day 1 of advent of code!");

    let file_name = "input/input.txt";

    let mut current_sum = 0;
    let mut elfs: Vec<i32> = Vec::new();

    let file = fs::File::open(file_name).expect("Cannot open file");

    let file = BufReader::new(file);
    for line in file.lines().filter_map(|result| result.ok()) {
        match line.parse::<i32>() {
            Ok(number) => {
                current_sum += number;
            }
            Err(_) => {
                elfs.push(current_sum);
                current_sum = 0;
            }
        }
    }
    elfs.push(current_sum);
    elfs.sort_by(|a,b| b.cmp(a));

    println!("Total calories from top 3 elfs is: {}",elfs.iter().take(3).sum::<i32>());
}
