use core::num;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

 // The input is a 140x140 matrix of chars
const NUM_ROW: usize = 140;
const NUM_COL: usize = 140;
const XMAS: &str = "XMAS";
enum DirMap {
    Right,
    Left,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

impl DirMap {
    fn directions(&self) -> (isize, isize) {
        match self {
            DirMap::Right => (1, 0),
            DirMap::Left => (-1, 0),
            DirMap::Up => (0, 1),
            DirMap::Down => (0, -1),
            DirMap::UpLeft => (-1, 1),
            DirMap::UpRight => (1, 1),
            DirMap::DownLeft => (-1, -1),
            DirMap::DownRight => (1, -1),
        }
    }
}

pub fn solve(){
    let file = File::open("data/day04_input.txt").expect("File day04_input.txt could not be found.");
    let reader: BufReader<File> = BufReader::new(file);
    let mut word_search: [[char; NUM_COL]; NUM_ROW] = [[' '; NUM_COL]; NUM_ROW];

    // Initialize array
    for (row, line) in reader.lines().enumerate() {
        let str: String = line.expect("Could not read line");
        for (col, c) in str.chars().enumerate(){
            word_search[row][col] = c;
        }
    }

    println!("------------ Day Four Answers ------------");
    println!("Part One: {}", part_one(&word_search));
    //println!("Part Two: {}", part_two(&word_search));
}

fn part_one(puzzle: &[[char; 140]; 140]) -> i32{
    let mut num_of_searches = 0;
    for row in 0..NUM_ROW{
        for col in 0..NUM_COL{
            if check_location(*puzzle, &row, &col){
                num_of_searches += 1;
            }
        }
    }    
    return num_of_searches;
}

fn check_location(puzzle:[[char; 140]; 140], row: &usize, col: &usize) -> bool{
    if check_direction(puzzle, row, col, DirMap::Right.directions()){ return true };
    if check_direction(puzzle, row, col, DirMap::Left.directions()){ return true };
    if check_direction(puzzle, row, col, DirMap::Up.directions()){ return true };
    if check_direction(puzzle, row, col, DirMap::Down.directions()){ return true };
    if check_direction(puzzle, row, col, DirMap::UpLeft.directions()){ return true };
    if check_direction(puzzle, row, col, DirMap::UpRight.directions()){ return true };
    if check_direction(puzzle, row, col, DirMap::DownLeft.directions()){ return true };
    if check_direction(puzzle, row, col, DirMap::DownRight.directions()){ return true };
    return false;
}

fn check_direction(puzzle:[[char; 140]; 140], row: &usize, col: &usize, direction: (isize, isize)) -> bool{
    
    for c in XMAS.to_string().chars(){
        if puzzle[*row][*col] != c{
            return false
        }

        // Need to look into subtracting usize types saturated, checked, wrapping.
        *row += direction.0;
        *col += direction.1;
    }
    

    return true;
}

