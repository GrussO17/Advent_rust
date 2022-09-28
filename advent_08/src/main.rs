use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: input.txt");
        println!("     input.txt");
        return;
    }
    let file: String = fs::read_to_string(args[1].clone()).expect("Please give a valid file name");
    let lines: Vec<&str> = file.split('\n').collect::<Vec<&str>>();
    let mut inputs: Vec<&str> = Vec::new();
    let mut outputs: Vec<&str> = Vec::new();
    for line in lines {
        let temp: Vec<&str> = line.split('|').map(|x| x.trim()).collect();
        inputs.push(temp[0]);
        outputs.push(temp[1]);
    }

    let mut sum = 0;
    for i in 0..inputs.len() {
        sum += process_line(inputs[i], outputs[i]);
    }
    let mut sizes = vec![0, 0, 0, 0, 0, 0, 0, 0];
    for output in &outputs {
        for number in output.split(' ') {
            sizes[number.len()] += 1;
        }
    }
    println!(
        "Total number of 1's, 4's, 7's and 8's --=> {}",
        sizes[7] + sizes[2] + sizes[4] + sizes[3]
    );
    println!("The sum of all outputs is {}", sum);
}

fn unordered_contains(str1: &str, str2: &str) -> bool {
    for i in str2.chars() {
        if !str1.contains(i) {
            return false;
        }
    }
    true
}

fn unordered_equal(str1: &str, str2: &str) -> bool {
    str1.len() == str2.len() && number_matches(str1, str2) == str1.len() as i32
}

fn number_matches(str1: &str, str2: &str) -> i32 {
    let mut sum = 0;
    for i in str2.chars() {
        if str1.contains(i) {
            sum += 1;
        }
    }
    sum
}

fn process_line(line: &str, outputs: &str) -> u32 {
    let mut translator: Vec<&str> = vec!["", "", "", "", "", "", "", "", "", ""];
    let mut segs6: Vec<&str> = Vec::new();
    let mut out_num = 0;
    let mut segs5: Vec<&str> = Vec::new();
    for number in line.split(' ') {
        match number.len() {
            2 => translator[1] = number,
            3 => translator[7] = number,
            4 => translator[4] = number,
            5 => segs5.push(number),
            6 => segs6.push(number),
            7 => translator[8] = number,
            _ => print!("uh oh"),
        }
    }
    for unknown in segs6 {
        if !unordered_contains(unknown, translator[1]) {
            translator[6] = unknown;
        } else if unordered_contains(unknown, translator[4]) {
            translator[9] = unknown;
        } else {
            translator[0] = unknown;
        }
    }
    for unknown in segs5 {
        if unordered_contains(unknown, translator[1]) {
            translator[3] = unknown;
        } else if number_matches(unknown, translator[6]) == 5 {
            translator[5] = unknown;
        } else {
            translator[2] = unknown;
        }
    }
    for (i, number) in outputs.split(' ').enumerate() {
        for (j, display) in translator.iter().enumerate() {
            if unordered_equal(number, display) {
                out_num += j as u32 * (10_u32.pow(3 - i as u32));
            }
        }
    }
    out_num
}
