use log::error;
use std::env;
use std::fs;

fn main() {
    let (mut distance, mut depth, mut aim): (i32, i32, i32) = (0, 0, 0);
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        error!("Did not provide sufficient args");
        println!("Usage: input.txt");
        println!("     input.txt");
        return;
    }
    let contents = fs::read_to_string(args[1].clone()).unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();
    if lines.len() == 0 {
        return;
    }

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let direction = parts[0];
        let val = parts[1].parse::<i32>().unwrap();
        match direction {
            "forward" => {
                distance += val;
                depth += val * aim;
            }
            "up" => aim -= val,
            "down" => aim += val,
            _ => return,
        }
    }
    println!("Final Depth {}, and distance {}", depth, distance);
    println!("Sum of both {}", depth * distance);
}
