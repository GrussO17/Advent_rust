use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: input.txt");
        println!("     input.txt");
        return;
    }
    let file: String = fs::read_to_string(args[1].clone()).expect("Please give a valid file name");
    let mut matrix: Vec<Vec<u8>> = Vec::new();
    let mut num_lowest: u32 = 0;
    let mut sum_lowest: u32 = 0;
    let mut lowest_locs: Vec<(u32, u32)> = Vec::new();
    for line in file.split('\n') {
        let mut new_line: Vec<u8> = Vec::new();
        for val in line.chars() {
            new_line.push(val.to_digit(10).expect("Found a non number") as u8);
        }
        matrix.push(new_line);
    }
    for row in 0..matrix.len() {
        for col in 0..matrix[0].len() {
            let val = matrix[row][col];
            if row == 0 {
                if val >= matrix[row + 1][col] {
                    continue;
                }
            } else if row == matrix.len() - 1 {
                if val >= matrix[row - 1][col] {
                    continue;
                }
            } else {
                if val >= matrix[row - 1][col] || val >= matrix[row + 1][col] {
                    continue;
                }
            }
            if col == 0 {
                if val >= matrix[row][col + 1] {
                    continue;
                }
            } else if col == matrix[0].len() - 1 {
                if val >= matrix[row][col - 1] {
                    continue;
                }
            } else {
                if val >= matrix[row][col - 1] || val >= matrix[row][col + 1] {
                    continue;
                }
            }
            num_lowest += 1;
            sum_lowest += val as u32 + 1;
            lowest_locs.push((row as u32, col as u32));
        }
    }

    println!("{} low spots found!", num_lowest);
    println!("{} sum of low spots", sum_lowest);

    //part 2
    let mut basin_sizes = find_basins(lowest_locs, &matrix);
    basin_sizes.sort();
    let sum_of_largest = basin_sizes[basin_sizes.len() - 1]
        * basin_sizes[basin_sizes.len() - 2]
        * basin_sizes[basin_sizes.len() - 3];
    println!("The product of the 3 largest basins is {}", sum_of_largest);
}

fn find_basins(minimums: Vec<(u32, u32)>, matrix: &Vec<Vec<u8>>) -> Vec<u32> {
    let mut sizes: Vec<u32> = Vec::new();
    for (row, col) in minimums {
        sizes.push(propogate_basin((row as i32, col as i32), &matrix));
    }
    sizes
}

fn propogate_basin(start: (i32, i32), matrix: &Vec<Vec<u8>>) -> u32 {
    let mut size = 0;
    let mut visited_points: Vec<(i32, i32)> = vec![start];
    let mut points: Vec<(i32, i32)> = vec![start];
    while points.len() > 0 {
        let (curr_row, curr_col) = points.pop().unwrap();
        //handle boundries
        if curr_row < 0
            || curr_row as usize >= matrix.len()
            || curr_col < 0
            || curr_col as usize >= matrix[0].len()
        {
            continue;
        }
        //handle end of basin, i promise its in range
        if matrix[curr_row as usize][curr_col as usize] == 9 {
            continue;
        }
        //add neighbors, assuming haven't already been there
        if !visited_points.contains(&(curr_row - 1, curr_col)) {
            points.push((curr_row - 1, curr_col));
            visited_points.push((curr_row - 1, curr_col));
        }
        if !visited_points.contains(&(curr_row + 1, curr_col)) {
            points.push((curr_row + 1, curr_col));
            visited_points.push((curr_row + 1, curr_col));
        }
        if !visited_points.contains(&(curr_row, curr_col - 1)) {
            points.push((curr_row, curr_col - 1));
            visited_points.push((curr_row, curr_col - 1));
        }
        if !visited_points.contains(&(curr_row, curr_col + 1)) {
            points.push((curr_row, curr_col + 1));
            visited_points.push((curr_row, curr_col + 1));
        }
        size += 1;
    }
    size
}
