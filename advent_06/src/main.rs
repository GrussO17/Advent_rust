use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: input.txt");
        println!("     input.txt");
        return;
    }
    let mut input: String =
        fs::read_to_string(args[1].clone()).expect("Please give a valid file name");
    let lines: Vec<&str> = input.split(',').collect();
    let mut numbers: Vec<i32> = Vec::new();
    for str_num in lines {
        let num = str_num
            .parse::<i32>()
            .expect("Could not parse input to int");
        numbers.push(num);
    }
    better_simulator(&mut numbers, 256);
}

fn better_simulator(numbers: &mut Vec<i32>, rounds: i32) {
    let mut fish_array: [u64; 9] = [0; 9];
    for num in numbers {
        fish_array[*num as usize] += 1;
    }
    for _i in 0..rounds {
        let mut num_reset = 0;
        for index in 0..9 {
            if index == 0 {
                num_reset = fish_array[0];
            } else {
                fish_array[index - 1] = fish_array[index];
            }
        }
        fish_array[8] = num_reset;
        fish_array[6] += num_reset;
    }
    let total: u64 = fish_array.iter().sum();
    println!("There are now {} fishies", total);
}

fn simulate(numbers: &mut Vec<i32>, rounds: i32) {
    for _i in 0..rounds {
        let mut next_vector: Vec<i32> = Vec::new();
        for j in 0..numbers.len() {
            let num = numbers.pop().unwrap();
            if num == 0 {
                next_vector.push(8);
                next_vector.push(6);
            } else {
                next_vector.push(num - 1);
            }
        }
        *numbers = next_vector;
    }
    println!("There are now {} fishies", numbers.len());
}
