mod day01;
mod day02;
mod day03;
mod day04;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <day>");
        return;
    }

    match args[1].as_str() {
        "1" => day01::solve(),
        "2" => day02::solve(),
        "3" => day03::solve(),
        "4" => day04::solve(),
        _ => println!("Invalid day specified"),
    }

}
