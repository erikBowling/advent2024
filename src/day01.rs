use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn solve(){
    let file = File::open("data/day01_input.txt").expect("File day01_input.txt could not be found.");
    let reader: BufReader<File> = BufReader::new(file);

    let mut left_numbers: Vec<i32> = Vec::new();
    let mut right_numbers: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line: String = line.expect("Failed to read line");
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();

        if numbers.len() == 2 {
            left_numbers.push(numbers[0]);
            right_numbers.push(numbers[1]);
        }
    }

    left_numbers.sort();
    right_numbers.sort();

    println!("------------ Day One Answers ------------");
    println!("Part One: {}", calculate_total_difference(&left_numbers, &right_numbers));
    println!("Part Two: {}", calculate_similarity_score(&left_numbers, &right_numbers));
}

fn calculate_total_difference(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    let differences: Vec<i32> = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| (a - b).abs())
        .collect::<Vec<i32>>();

    return differences.iter().sum::<i32>();
}

fn calculate_similarity_score(left_list: &Vec<i32>, right_list: &Vec<i32>) -> i32 {
    // Map elements to it's rate of occurance
    // Sum across those rates
    // Return result
    let mut occurence_rates: HashMap<i32, i32>= HashMap::new();

    // Calculate the number of occurences per number and save it as the value in the above hashmap
    for item in left_list {
        let mode_of_item: i32 = right_list
        .iter()
        .filter(|x| *x == item)
        .count() as i32;

        occurence_rates.insert(*item, mode_of_item);
    }

    let mut accumulator: i32 = 0;

    for (key, value) in occurence_rates.into_iter(){
        accumulator += key * value;
    }

    return accumulator;

}