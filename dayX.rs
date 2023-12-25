// Template file

use std::{env, fs, process};


fn part1and2(input: &str) {
    let chars = input.chars();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file: String = if args.len() >= 2 {
        match fs::read_to_string(args[1].clone()) {
            Ok(x) => x,
            Err(x) => panic!("Error reading file: {}", x)
        }
    } else {
        println!("Usage: <input file>");
		process::exit(1);
    };

    part1and2(file.as_str());
    // or part1 and part2 separetly if nessecary
}