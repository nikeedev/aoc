use std::{env, fs, process};


fn part1and2(input: &str) {
    let chars = input.chars();

    let mut floor = 0;

    let mut touched = false;

    for (i, j) in chars.enumerate() {
        match j {
            '(' => {
                floor += 1;
            },
            ')' => {
                floor -= 1;
            },
            _ => {} 
        }
        // part2:
        if floor == -1 && !touched {
            touched = true;
            println!("Touched basement at: {}", i + 1);
        }
        //
    }

    println!("The floor is: {}", floor);
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
}