mod day01;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run <day>");
        return;
    }

    match args[1].as_str() {
        "1" => day01::solve(),
        // "2" => day02::solve(),
        // Add more cases for other days
        _ => println!("Invalid day specified"),
    }

}
