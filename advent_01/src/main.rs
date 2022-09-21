use log::error;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        error!("Did not provide sufficient args");
        println!("Usage: input.txt");
        return;
    }

    let contents = fs::read_to_string(args[1].clone()).unwrap();
    let lines: Vec<&str> = contents.split_whitespace().collect();
    if lines.len() == 0 {
        return;
    }
    let mut prev = lines[0].parse::<i32>().unwrap();
    let mut total: u32 = 0;
    for line in lines {
        let curr = line.parse::<i32>().unwrap();
        if curr < prev {
            println!("Decreased");
        } else if curr > prev {
            println!("Increased");
            total += 1;
        }
        prev = curr;
    }
    println!("{} number of increases", total);
}
