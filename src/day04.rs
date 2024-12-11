use std::fs::File;
use std::io::{BufRead, BufReader};

 // The input is a 140x140 matrix of chars
const NUM_ROW: usize = 140;
const NUM_COL: usize = 140;
const XMAS: &str = "XMAS";
const MAS: &str = "MAS";

struct DirMap {
    right: (isize, isize),
    left: (isize, isize),
    up: (isize, isize),
    down: (isize, isize),
    up_right: (isize, isize),
    up_left: (isize, isize),
    down_left: (isize, isize),
    down_right: (isize, isize)
}

const DIRECTIONS: DirMap = DirMap {
    right: (1, 0),
    left: (-1, 0),
    up: (0, 1),
    down: (0, -1),
    up_right: (1, 1),
    up_left: (-1, 1),
    down_left: (-1, -1),
    down_right: (1, -1),
};

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
    println!("Part Two: {}", part_two(&word_search));
}

fn part_one(puzzle: &[[char; NUM_COL]; NUM_ROW]) -> i32{
    let mut num_of_searches = 0;
    for row in 0..NUM_ROW{
        for col in 0..NUM_COL{
            num_of_searches += check_location_any(puzzle, row, col);
        }
    }    
    return num_of_searches;
}

fn part_two(puzzle: &[[char; NUM_COL]; NUM_ROW]) -> i32{
    let mut num_of_searches = 0;
    for row in 0..NUM_ROW{
        for col in 0..NUM_COL{
            num_of_searches += check_location_x(puzzle, row, col);
        }
    }    
    return num_of_searches;
}

// HELPERS

fn check_location_any(puzzle: &[[char; NUM_COL]; NUM_ROW], row: usize, col: usize) -> i32{
    if puzzle[row][col] != 'X' { return 0; }

    let mut accumulator = 0;
    let directions: [(isize, isize); 8] = [ 
        DIRECTIONS.right,
        DIRECTIONS.left,
        DIRECTIONS.up,
        DIRECTIONS.down,
        DIRECTIONS.up_right,
        DIRECTIONS.up_left,
        DIRECTIONS.down_right,
        DIRECTIONS.down_left
    ];

    for direction in directions.iter(){
        if check_direction(puzzle, row, col, *direction, XMAS.to_string()) { 
            accumulator += 1;
        }
    }
    return accumulator;
}

fn check_location_x(puzzle: &[[char; NUM_COL]; NUM_ROW], row: usize, col: usize) -> i32{
    if puzzle[row][col] != 'A' { return 0; }

    let mut num_mas: i32 = 0;
    let directions: [(isize, isize); 4] = [ 
        DIRECTIONS.up_right,
        DIRECTIONS.up_left,
        DIRECTIONS.down_right,
        DIRECTIONS.down_left
    ];

    for direction in directions.iter(){
        let mut current_row: usize = row;
        let mut current_col: usize = col;
        match (current_row.checked_add_signed(direction.0), current_col.checked_add_signed(direction.1)) {
            (Some(new_row), Some(new_col)) => {
                current_row = new_row;
                current_col = new_col;
            },
            _ => continue,
        }
        if current_row >= NUM_ROW || current_col >= NUM_COL { continue; }
        
        if puzzle[current_row][current_col] == 'M'{
            if check_direction(puzzle, current_row, current_col, opposite(direction), MAS.to_string()){
                num_mas += 1;
            }
        }
    }

    if num_mas >= 2 { return 1 }
    
    return 0;
}

fn check_direction(puzzle: &[[char; NUM_COL]; NUM_ROW], mut row: usize, mut col: usize, direction: (isize, isize), check_string: String) -> bool{
    for c in check_string.chars(){
        if row >= NUM_ROW || col >= NUM_COL { return false; }
        if puzzle[row][col] != c{
            return false;
        }
        if c == 'S' { return true; }
        
        match (row.checked_add_signed(direction.0), col.checked_add_signed(direction.1)) {
            (Some(new_row), Some(new_col)) => {
                row = new_row;
                col = new_col;
            },
            _ => return false // Out of bounds
        }
    }

    return true;
}

fn opposite(direction: &(isize, isize)) -> (isize, isize){
    return (direction.0 * -1, direction.1 * -1);
}

