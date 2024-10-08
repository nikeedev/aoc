// 2015: day 2

use std::{env, fs, ops::Index, process};

fn part1and2(input: Vec<&str>) {
    // part 1:

    let mut total_sum = 0;
    let mut total_ribbon = 0;

    for line in input {
        // println!("line: {}", i + 1);

        let mut line_sum = 0;
        let mut line_ribbon_sum = 0;

        let mut l = String::new();
        let mut w = String::new();
        let mut h = String::new();

        let mut num = 1;

        for char in line.chars() {
            if char == 'x' {
                num += 1;
            } else {
                match num {
                    1 => l.push(char),
                    2 => w.push(char),
                    3 => h.push(char),
                    _ => {}
                }
            }
            // println!("num: {}", num);
            // println!("char: {}", char);
            // println!("l:{}, w:{}, h:{}", l, w, h);
        }
        let l = l.parse::<i32>().unwrap();
        let w = w.parse::<i32>().unwrap();
        let h = h.parse::<i32>().unwrap();

        line_sum += (2 * l * w) + (2 * w * h) + (2 * h * l);

        line_sum += [l * w, w * h, h * l].iter().min().unwrap();

        total_sum += line_sum;

        let mut nums = vec![l, w, h];

        let smallest = *nums.iter().min().unwrap();
        nums.remove(nums.iter().position(|n| *n == smallest).unwrap());
        let second_smallest = *nums.iter().min().unwrap();

        println!("smallest: {}, second_smallest: {}", smallest, second_smallest);
        line_ribbon_sum += smallest * 2;
        line_ribbon_sum += second_smallest * 2;
        line_ribbon_sum += l * w * h;

        // println!("line_ribbon_sum: {}", line_ribbon_sum);

        total_ribbon += line_ribbon_sum;
    }
    println!("Total ribbon: {}", total_ribbon);
    println!("Total sum: {}", total_sum);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file: String = if args.len() >= 2 {
        match fs::read_to_string(args[1].clone()) {
            Ok(x) => x,
            Err(x) => panic!("Error reading file: {}", x),
        }
    } else {
        println!("Usage: <input file>");
        process::exit(1);
    };

    let file: Vec<&str> = file.lines().collect();

    part1and2(file);
    // or part1 and part2 separetly if nessecary
}
