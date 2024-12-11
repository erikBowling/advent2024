use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve(){
    let file = File::open("data/day04_input.txt").expect("File day04_input.txt could not be found.");
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let str: String = line.expect("Could not read line");
        println!("{}", str);
    }

    println!("------------ Day Five Answers ------------");
    println!("Part One: {}", part_one());
    //println!("Part Two: {}", part_two(&word_search));
}

fn part_one() -> i32{

    return 0;
}