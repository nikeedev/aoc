// 2015: day 4
use std::{env, fs, process};

use crypto::md5::Md5;
use crypto::digest::Digest;

fn part1and2(secret_code: &str) {
    let mut hasher = Md5::new();

    let key = secret_code.as_bytes();
    for i in 0..u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());
        
        let mut output = [0; 16]; // An MD5 is 16 bytes
        hasher.result(&mut output);

        let first_five = output[0] as i32 + output[1] as i32 + output[2] as i32;
        if first_five == 0 {
            println!("{}", i);
            break;
        }
        hasher.reset();
    }
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