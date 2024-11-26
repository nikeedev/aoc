// 2015: day 3

use std::{env, fs, process, vec};


fn part1and2(input: &str) {
    let chars = input.chars();

    let mut xs = 250;
    let mut ys = 250;
    let mut xr = 250; 
    let mut yr = 250;

    let mut presents: Vec<Vec<i32>> = vec![vec![0; 500]; 500];

    presents[xs][ys] += 2;

    let mut robo_turn = false;

    for ch in chars {
        match ch { 
            'v' => {
                if robo_turn {
                    yr += 1;
                } else {
                    ys += 1;
                }
            },
            '^' => {
                if robo_turn {
                    yr -= 1;
                } else {
                    ys -= 1;
                }
            },
            '>' => {
                if robo_turn {
                    xr += 1;
                } else {
                    xs += 1;
                }
            },
            '<' => {
                if robo_turn {
                    xr -= 1;
                } else {
                    xs -= 1;
                }
            },

            _ => {}
        }

        if robo_turn {
            presents[xr][yr] += 1;
        } else {
            presents[xs][ys] += 1;
        }

        println!("char: {} - {} - x: {} y: {}", ch, if robo_turn {"robot"} else {"santa"}, if robo_turn { xr } else { xs }, if robo_turn { yr } else { ys });
        robo_turn = !robo_turn;
    }
    // println!("x: {} y: {}", x, y);
    // println!("presents: {:?}", presents);
    println!("Houses with miniumum one present: {}", presents.iter().flatten().filter(|&x| *x > 0).count());
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