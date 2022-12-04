#![feature(iter_array_chunks)]

pub fn process_part1(input: &str) -> u32 {
    let mut vec_result: Vec<u32> = Vec::new();

    for line in input.lines() {
        let sack_size = line.len() / 2;
        let (sack_a, sack_b) = line.split_at(sack_size);

        let common_char = sack_a.chars().find(|c| sack_b.contains(*c)).unwrap();
        let nu = get_char_index(&common_char);
        vec_result.push(nu);
    }
    return vec_result.iter().sum();
}

pub fn process_part2(input: &str) -> u32 {
    let mut vec_result: Vec<u32> = Vec::new();
    
    for [line1, line2, line3] in input.lines().array_chunks::<3>() {
        let common_char: char = line1.chars().find(|c: &char| {line2.contains(*c) && line3.contains(*c)}).unwrap();
        let nu = get_char_index(&common_char);
        vec_result.push(nu);
    }

    return vec_result.iter().sum();
}

pub fn get_char_index(c: &char) -> u32 {
    if c.is_lowercase() {
        return *c as u32 - 96;
    } else if !c.is_lowercase() {
        return *c as u32 - (65-27);
    } else {
        panic!("Could not convert char to index");
    }
}

fn main() {
    println!("Welcome to day 3 of Advent of Code\n");
    let file = std::fs::read_to_string("files/input.txt").unwrap();
    println!("Result for Part 1: {}", process_part1(&file));
    println!("Result for Part 2: {}", process_part2(&file));
}
