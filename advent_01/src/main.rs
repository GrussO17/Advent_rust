use log::{debug, error};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        error!("Did not provide sufficient args");
        println!("Usage: input.txt part");
        println!("     input.txt 2");
        return;
    }
    let contents = fs::read_to_string(args[1].clone()).unwrap();
    let lines: Vec<&str> = contents.split_whitespace().collect();
    if lines.len() == 0 {
        return;
    }
    match args[2].parse::<i32>().unwrap() {
        1 => part_one(lines),
        2 => part_two(lines),
        _ => return,
    }
}

fn part_one(input: Vec<&str>) {
    println!("Running part 1");
    let mut prev = input[0].parse::<i32>().unwrap();
    let mut total: u32 = 0;
    for line in input {
        let curr = line.parse::<i32>().unwrap();
        if curr < prev {
            debug!("Decreased");
        } else if curr > prev {
            debug!("Increased");
            total += 1;
        }
        prev = curr;
    }
    println!("{} number of increases", total);
}

fn part_two(input: Vec<&str>) {
    println!("Running part 2");
    let mut total: u32 = 0;
    for i in 3..input.len() {
        let prev = input[i - 3].parse::<i32>().unwrap();
        let curr = input[i].parse::<i32>().unwrap();
        if curr > prev {
            debug!("Increased");
            total += 1;
        } else if prev > curr {
            debug!("Decreased");
        }
    }
    println!("{} number of increases", total);
}
