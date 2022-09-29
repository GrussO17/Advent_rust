use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: input.txt");
        println!("     input.txt");
        return;
    }
    let file: String = fs::read_to_string(args[1].clone()).expect("Please give a valid file name");
    let mut score = 0;
    let mut scores_2 = Vec::new();
    for line in file.split('\n') {
        let ret = syntax_check(line);
        match ret {
            Ok(v) => scores_2.push(sum_score(v)),
            Err(e) => match e {
                ')' => score += 3,
                ']' => score += 57,
                '}' => score += 1197,
                '>' => score += 25137,
                _ => (),
            },
        }
    }
    scores_2.sort();
    println!("The score of this syntax is {}", score);
    println!(
        "The score of the autocomplete is {}",
        scores_2[scores_2.len() / 2]
    );
}

fn syntax_check(line: &str) -> Result<Vec<char>, char> {
    //For determining if the character is an opener or closer, also the
    //lists are in the same order, if it opens with opening[0] is must
    //close with closing[0]
    let opening = "<({[";
    let closing = ">)}]";

    //List of currently 'open' groups
    let mut open: Vec<char> = Vec::new();

    for char in line.chars() {
        //See if the result is a closer or not, if it is not set its location to out of bounds
        let result = closing.find(char).unwrap_or(4);

        // if it is an opener, just add it to the list of open chars
        if opening.contains(char) {
            open.push(char);
        } else {
            // The result is a closer or bad character

            //open size check
            if open.len() == 0 {
                return Err(char);
            }
            //find the location of the most recently opened bracket, else make it out of bounds
            let last_loc = opening.find(open.pop().unwrap()).unwrap_or(5);

            //compare locations, if not the same the syntax is bad
            if last_loc == result {
                continue;
            } else {
                return Err(char);
            }
        }
    }
    if open.len() > 0 {
        //still values left, incomplete
        let mut ret: Vec<char> = Vec::new();
        while open.len() > 0 {
            let open_loc = opening.find(open.pop().unwrap()).unwrap();
            ret.push(closing.chars().nth(open_loc).unwrap());
        }
        return Ok(ret);
    }
    Ok(vec![])
}

fn sum_score(input: Vec<char>) -> u64 {
    let mut ret = 0;
    for val in input {
        match val {
            ')' => ret = ret * 5 + 1,
            ']' => ret = ret * 5 + 2,
            '}' => ret = ret * 5 + 3,
            '>' => ret = ret * 5 + 4,
            _ => (),
        }
    }
    ret
}
