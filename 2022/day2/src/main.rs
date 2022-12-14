use std::fs;
use std::{cmp::Ordering, str::FromStr};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

// Turns the characters from the file into Move enums
impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("Not a known move".to_string()),
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            Some(Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissors {
            Some(Ordering::Greater)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

pub fn process_part1(input: &str) -> u32 {
    let result: u32 = input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            match moves[0].partial_cmp(&moves[1]) {
                Some(Ordering::Equal) => 3 + moves[1] as u32,
                Some(Ordering::Less) => 6 + moves[1] as u32,
                Some(Ordering::Greater) => 0 + moves[1] as u32,
                None => panic!("moves should be comparable"),
            }
        })
        .sum();

    return result;
}

pub fn process_part2(input: &str) -> u32 {
    let result = input.lines().map(|line| {
        let moves: Vec<&str> = line.split(" ").collect();
        let opponent_move = moves[0].parse::<Move>().unwrap();
        match moves[1] {
            "X" => {
                let our_move = match opponent_move {
                    Move::Rock => Move::Scissors,
                    Move::Paper => Move::Rock,
                    Move::Scissors => Move::Paper,
                };
                0 + our_move as u32
            }
            "Y" => 3 + opponent_move as u32,
            "Z" => {
                let our_move = match opponent_move {
                    Move::Paper => Move::Scissors,
                    Move::Rock => Move::Paper,
                    Move::Scissors => Move::Rock,
                };
                6 + our_move as u32
            }
            _ => panic!("Error")
        }
    }).sum();

    return result;
}

fn main() {
    println!("Welcome to day 2 of Advent of Code");
    let file = fs::read_to_string("files/input.txt").unwrap();
    println!("Our score is: {}", process_part1(&file));
    println!("Our score for part 2 is: {}", process_part2(&file));
}
