use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;


pub fn solve(){
    let file = File::open("data/day03_input.txt").expect("File day02_input.txt could not be found.");
    let reader: BufReader<File> = BufReader::new(file);

    let mut corrupted_memory: Vec<String> = Vec::new();

    for line in reader.lines() {
        let str: String = line.expect("Could not read line");
        corrupted_memory.push(str);
    }

    println!("------------ Day Two Answers ------------");
    println!("Part One: {}", part_one(corrupted_memory));
    // println!("Part Two: {}", part_two(&report_matrix));
}

fn part_one(memory: Vec<String>) -> i32{

    let mul_pattern = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let int_pattern = Regex::new(r"[0-9]+").unwrap();

    let mut valid_multiplications: Vec<String> = Vec::new();

    for line in memory{
        let mut current_line_valid_multiplications: Vec<String> = mul_pattern
            .find_iter(&line)
            .map(|m| m.as_str().to_string())
            .collect();
        
        valid_multiplications.append(&mut current_line_valid_multiplications);
    }

    let mul_tuples: Vec<(i32, i32)> = valid_multiplications
        .iter()
        .filter_map(|m| {
            int_pattern.captures(m).and_then(|cap| {
                let first = cap[0].parse::<i32>().ok()?;
                let second = cap[1].parse::<i32>().ok()?;
                Some((first, second))
            })
        }).collect();
        
    return mul_tuples
        .iter()
        .map(|(a,b)| a * b)
        .sum()
}