use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

pub fn solve(){
    let file = File::open("data/day02_input.txt").expect("File day02_input.txt could not be found.");
    let reader: BufReader<File> = BufReader::new(file);

    let mut report_matrix: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line: String = line.expect("Failed to read line");
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Failed to parse number"))
            .collect();

        report_matrix.push(numbers);
    }

    println!("------------ Day Two Answers ------------");
    println!("Part One: {}", part_one(&report_matrix));
    println!("Part Two: {}", part_two(&report_matrix));
}

fn part_one(matrix: &Vec<Vec<i32>>) -> i32 {
    let acceptable_range: RangeInclusive<i32> = 1..=3;

    let num:i32 = matrix
        .iter()
        .filter(|x| check_element_safe_distances(x, &acceptable_range))
        .filter(|x| (check_is_ascending(x) ^ check_is_descending(x)))
        .count() as i32;

    return num;
}

fn part_two(matrix: &Vec<Vec<i32>>) -> i32{
    let acceptable_range: RangeInclusive<i32> = 1..=3;

    let unsafe_reports: Vec<Vec<i32>> = matrix
        .iter()
        .filter(|x| !check_element_safe_distances(x, &acceptable_range) || !(check_is_ascending(x) ^ check_is_descending(x)))
        .cloned()
        .collect();

    let mut safe_reports: Vec<Vec<i32>> = matrix
        .iter()
        .filter(|x| (check_is_ascending(x) ^ check_is_descending(x)))
        .filter(|x| check_element_safe_distances(x, &acceptable_range))
        .cloned()
        .collect();
    
    let mut dampened_safe_reports: Vec<Vec<i32>> = unsafe_reports
        .iter()
        .filter(|x| dampener(x, &acceptable_range))
        .cloned()
        .collect();

    safe_reports.append(&mut dampened_safe_reports);
    
    return safe_reports.iter().count() as i32;
}

// HELPERS 

fn check_is_ascending(list: &Vec<i32>) -> bool {
    let mut sorted_list: Vec<i32> = list.clone();
    sorted_list.sort();

    return sorted_list == *list
}

fn check_is_descending(list: &Vec<i32>) -> bool {
    let mut reversed_list: Vec<i32> = list.clone();
    reversed_list.sort();
    reversed_list.reverse();

    return reversed_list == *list
}

fn check_element_safe_distances(list: &Vec<i32>, acceptable_range: &RangeInclusive<i32>) -> bool{
    if list.len() < 2 { return true; }

    for window in list.windows(2){
        let difference: i32 = (window[0] - window[1]).abs();
        if !acceptable_range.contains(&difference){
            return false;
        }
    } 

    return true;
}

fn dampener(list: &Vec<i32>, acceptable_range: &RangeInclusive<i32>) -> bool{
    for (index, _) in list.iter().enumerate(){
        let mut list_without_item: Vec<i32> = list.clone();
        list_without_item.remove(index);

        if (check_is_ascending(&list_without_item) ^ check_is_descending(&list_without_item)) 
            && check_element_safe_distances(&list_without_item, acceptable_range){ return true; }

    }

    return false;
}