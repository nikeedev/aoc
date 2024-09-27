// 2015: day 3

use std::{env, fs, process, vec};


fn part1and2(input: &str) {
    let chars = input.chars();

    let mut x = 0;
    let mut y = 0;

    let mut presents: Vec<Vec<i32>> = vec![vec![0; 50]; 50];

    for ch in chars {
        match ch { 
            'v' => y += 1,
            '^' => y -= 1,
            '>' => x += 1,
            '<' => x -= 1,

            _ => {}
        }
        presents[x][y] += 1;
    }
    println!("x: {} y: {}", x, y);
    println!("presents: {:?}", presents);

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