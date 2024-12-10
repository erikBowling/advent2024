use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;


pub fn solve(){
    let file = File::open("data/day03_input.txt").expect("File day03_input.txt could not be found.");
    let reader: BufReader<File> = BufReader::new(file);

    let mut corrupted_memory: Vec<String> = Vec::new();

    for line in reader.lines() {
        let str: String = line.expect("Could not read line");
        corrupted_memory.push(str);
    }

    println!("------------ Day Three Answers ------------");
    println!("Part One: {}", part_one(&corrupted_memory));
    println!("Part Two: {}", part_two(&corrupted_memory));
}

fn part_one(memory: &Vec<String>) -> i32{

    let mul_pattern = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let mut valid_multiplications: Vec<String> = Vec::new();

    for line in memory{
        let mut current_line_valid_multiplications: Vec<String> = mul_pattern
            .find_iter(line)
            .map(|m| m.as_str().to_string())
            .collect();
        
        valid_multiplications.append(&mut current_line_valid_multiplications);
    }

    let mut mul_tuples: Vec<(i32, i32)> = Vec::new();
    for str_mul in valid_multiplications{
        mul_tuples.push(create_int_tuple(&str_mul));
    }

    return mul_tuples
        .iter()
        .map(|(a, b)| a * b)
        .sum();
}

fn part_two(memory: &Vec<String>) -> i32{
    let match_patterns = Regex::new(r"don't\(\)|do\(\)|mul\([0-9]+,[0-9]+\)").unwrap();
    let mut all_matches: Vec<String> = Vec::new();
    for line in memory{
        let mut match_pattern_results: Vec<String> = match_patterns
            .find_iter(&line)
            .map(|m| m.as_str().to_string())
            .collect();

        all_matches.append(&mut match_pattern_results);
    }

    let mut allowed = true;
    let mut mul_tuples: Vec<(i32, i32)> = Vec::new();

    for str in all_matches{
        if str == "do()"{
            allowed = true;
        }
        else if str == "don't()"{
            allowed = false;
        }
        else{
            if allowed {
                mul_tuples.push(create_int_tuple(&str));
            }else{
                continue;
            }
        }
    
    }

    return mul_tuples
        .iter()
        .map(|(a, b)| a * b)
        .sum();
}

// HELPERS

fn create_int_tuple(str_mul: &String)-> (i32, i32){
    let int_pattern = Regex::new(r"[0-9]+").unwrap();
    let current_mul: Vec<i32> = int_pattern
        .find_iter(&str_mul)
        .map(|m| m.as_str().parse::<i32>().expect("Failed to parse integer"))
        .collect();

    return (*current_mul.first().unwrap(), *current_mul.last().unwrap());
}