use std::collections::HashMap;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: input.txt" "include_diagi");
        println!("     input.txt");
        return;
    }
    let input = fs::read_to_string(args[1].clone()).expect("Please give a valid file name");
    let lines: Vec<Line> = read_lines(input);
    let mut map: HashMap<[u32; 2], u32> = HashMap::with_capacity(10000);
    fill_hashmap(&mut map, lines, true);
    let part1 = calculate_overlap(&map);
    println!("{} is the number of ares to avoid", part1);
}

#[derive(Default, Debug)]
struct Line {
    end_points: [[i32; 2]; 2],
}

fn read_lines(input: String) -> Vec<Line> {
    let mut ret: Vec<Line> = Vec::new();
    let lines: Vec<&str> = input.split('\n').collect();
    for line in lines {
        let points: Vec<&str> = line.split(" -> ").collect();
        let mut curr_line: Line = Default::default();
        for (i, point) in points.iter().enumerate() {
            let coords: Vec<&str> = point.split(",").collect();
            for (j, coord) in coords.iter().enumerate() {
                curr_line.end_points[i][j] =
                    coord.parse::<i32>().expect("could not convert to u32");
            }
        }
        ret.push(curr_line);
    }
    ret
}

fn fill_hashmap(map: &mut HashMap<[u32; 2], u32>, lines: Vec<Line>, include_diag: bool) {
    for line in lines {
        let mut points: Option<Vec<[u32; 2]>> = Default::default();
        if include_diag {
            points = iterate_more_points(line);
        } else {
            points = iterate_points(line);
        }
        if points == None {
            continue;
        }
        for point in points.unwrap() {
            let curr_val = map.get(&point);
            if curr_val.is_some() {
                map.insert(point, curr_val.unwrap() + 1);
            } else {
                map.insert(point, 1);
            }
        }
    }
}

fn iterate_more_points(line: Line) -> Option<Vec<[u32; 2]>> {
    let mut ret: Vec<[u32; 2]> = Vec::new();
    if line.end_points[0][0] == line.end_points[1][0] {
        //Vertical line
        let mut len = line.end_points[1][1] - line.end_points[0][1];
        while len != 0 {
            ret.push([
                line.end_points[0][0] as u32,
                (line.end_points[0][1] + len) as u32,
            ]);
            if len < 0 {
                len += 1;
            } else {
                len -= 1;
            }
        }
        ret.push([line.end_points[0][0] as u32, line.end_points[0][1] as u32])
    } else if line.end_points[0][1] == line.end_points[1][1] {
        //Horizontal line
        let mut len = line.end_points[1][0] - line.end_points[0][0];
        while len != 0 {
            ret.push([
                (line.end_points[0][0] + len) as u32,
                line.end_points[0][1] as u32,
            ]);
            if len < 0 {
                len += 1;
            } else {
                len -= 1;
            }
        }
        ret.push([line.end_points[0][0] as u32, line.end_points[0][1] as u32])
    } else {
        //weird lines
        let mut len_1 = line.end_points[1][0] - line.end_points[0][0];
        let mut len_2 = line.end_points[1][1] - line.end_points[0][1];
        while len_1 != 0 {
            ret.push([
                (line.end_points[0][0] + len_1) as u32,
                (line.end_points[0][1] + len_2) as u32,
            ]);
            if len_1 < 0 {
                len_1 += 1;
            } else {
                len_1 -= 1;
            }
            if len_2 < 0 {
                len_2 += 1;
            } else {
                len_2 -= 1;
            }
        }
        ret.push([line.end_points[0][0] as u32, line.end_points[0][1] as u32])
    }
    if ret.len() == 0 {
        return None;
    } else {
        Some(ret)
    }
}

fn iterate_points(line: Line) -> Option<Vec<[u32; 2]>> {
    let mut ret: Vec<[u32; 2]> = Vec::new();
    if line.end_points[0][0] == line.end_points[1][0] {
        //Vertical line
        let mut len = line.end_points[1][1] - line.end_points[0][1];
        while len != 0 {
            ret.push([
                line.end_points[0][0] as u32,
                (line.end_points[0][1] + len) as u32,
            ]);
            if len < 0 {
                len += 1;
            } else {
                len -= 1;
            }
        }
        ret.push([line.end_points[0][0] as u32, line.end_points[0][1] as u32])
    } else if line.end_points[0][1] == line.end_points[1][1] {
        //Horizontal line
        let mut len = line.end_points[1][0] - line.end_points[0][0];
        while len != 0 {
            ret.push([
                (line.end_points[0][0] + len) as u32,
                line.end_points[0][1] as u32,
            ]);
            if len < 0 {
                len += 1;
            } else {
                len -= 1;
            }
        }
        ret.push([line.end_points[0][0] as u32, line.end_points[0][1] as u32])
    }
    if ret.len() == 0 {
        return None;
    } else {
        Some(ret)
    }
}

fn calculate_overlap(map: &HashMap<[u32; 2], u32>) -> u32 {
    let mut count = 0;
    for val in map.values().cloned().collect::<Vec<u32>>() {
        if val > 1 {
            count += 1;
        }
    }
    count
}
