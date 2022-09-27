use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: input.txt");
        println!("     input.txt");
        return;
    }
    let input: String = fs::read_to_string(args[1].clone()).expect("Please give a valid file name");
    let mut positions: Vec<u32> = input
        .split(',')
        .map(|x| x.parse::<u32>().expect("could not convert to int"))
        .collect();
    println!("{:?}", positions);
    positions.sort();
    println!("{:?}", positions);
    let median: u32 = *positions.get(positions.len() / 2).unwrap();
    println!("median is {}", median);
    let total = buy_gas(&positions, median);
    println!("Gas used is {}", total);
    let mut options: Vec<u32> = Vec::new();
    for i in 1..1000 {
        options.push(buy_gas(&positions, i));
    }
    options.sort();
    println!(
        "The cheapest way with the updated cost function is {:?}",
        options[0]
    );
}

fn buy_gas(locations: &Vec<u32>, median: u32) -> u32 {
    let mut sum = 0;
    for loc in locations {
        let diff = loc.abs_diff(median);
        for i in 1..diff + 1 {
            sum += i;
        }
    }
    sum
}
