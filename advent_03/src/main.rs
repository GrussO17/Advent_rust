use log::error;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut places: Vec<i32> = vec![0; 12];

    if args.len() != 2 {
        error!("Did not provide sufficient args");
        println!("Usage: input.txt");
        println!("     input.txt");
        return;
    }
    let contents = fs::read_to_string(args[1].clone()).unwrap();
    let lines: Vec<&str> = contents.split('\n').collect();
    let numlines: i32 = lines.len() as i32 / 2;
    if lines.len() == 0 {
        return;
    }
    for line in &lines {
        for (i, bit) in line.as_bytes().iter().enumerate() {
            if *bit == b'1' {
                places[i] += 1;
            }
        }
    }
    let mut gamma: u32 = 0;
    let mut epsilon: u32 = 0;
    for (i, bit) in places.iter().enumerate() {
        let i = i as u32;
        if *bit > numlines {
            gamma += u32::pow(2, places.len() as u32 - i);
        } else {
            epsilon += u32::pow(2, places.len() as u32 - i);
        }
    }
    println!("Gamma {}, Epsilon {}", gamma, epsilon);
    println!("Power Consumption {}", gamma * epsilon);
    //let number = i32::from_str_radix(line, 2).expect("Not a Binary Number");

    let mut currently_left_o2: Vec<&str> = lines.clone();
    let mut count = 0;
    let mut num_ones_o2 = places[0];
    let mut num_zeros_o2 = numlines.clone();
    let mut currently_left_co2: Vec<&str> = lines.clone();
    let mut num_ones_co2 = places[0];
    let mut num_zeros_co2 = numlines.clone();
    let mut co2 = 0;
    let mut o2 = 0;
    loop {
        //Calculate number of 1's left in current index
        num_ones_o2 = 0;
        for line in &currently_left_o2 {
            if line.as_bytes()[count] == b'1' {
                num_ones_o2 += 1;
            }
        }
        num_zeros_o2 = currently_left_o2.len() as i32 - num_ones_o2;
        num_ones_co2 = 0;
        for line in &currently_left_co2 {
            if line.as_bytes()[count] == b'1' {
                num_ones_co2 += 1;
            }
        }
        num_zeros_co2 = currently_left_co2.len() as i32 - num_ones_co2;

        //Keep only the numbers with most popular bits
        let mut next_o2: Vec<&str> = Vec::new();
        for line in &currently_left_o2 {
            if num_ones_o2 >= num_zeros_o2 && line.as_bytes()[count] == b'1' {
                next_o2.push(line);
            } else if num_ones_o2 < num_zeros_o2 && line.as_bytes()[count] == b'0' {
                next_o2.push(line);
            }
        }
        //keep only numbers with least popular bits
        let mut next_co2: Vec<&str> = Vec::new();
        for line in &currently_left_co2 {
            if num_ones_co2 >= num_zeros_co2 && line.as_bytes()[count] == b'0' {
                next_co2.push(line);
            } else if num_ones_co2 < num_zeros_co2 && line.as_bytes()[count] == b'1' {
                next_co2.push(line);
            }
        }
        currently_left_co2 = next_co2;
        currently_left_o2 = next_o2;
        count += 1;
        if currently_left_co2.len() == 1 {
            println!("co2 generator rating {:?}", currently_left_co2);
            co2 = 0;
            for (i, bit) in currently_left_co2[0].as_bytes().iter().enumerate() {
                if *bit == b'1' {
                    co2 += i32::pow(2, currently_left_co2[0].len() as u32 - i as u32 - 1);
                }
            }
            println!("rating: {}", co2);
        }
        if currently_left_o2.len() == 1 {
            println!("o2 generator rating {:?}", currently_left_o2);
            o2 = 0;
            for (i, bit) in currently_left_o2[0].as_bytes().iter().enumerate() {
                if *bit == b'1' {
                    o2 += i32::pow(2, currently_left_o2[0].len() as u32 - i as u32 - 1);
                }
            }
            println!("rating: {}", o2);
        }
        if currently_left_o2.len() <= 1 && currently_left_co2.len() <= 1 {
            break;
        }
    }
    println!("diagnostic rating {}", co2 * o2);
}
