use std::fs;

fn main() {
    let mut game = populate_bingo("input.txt");
    let mut game1 = game.clone();
    let soln = play(&mut game);
    let soln2 = lose(&mut game1);
    println!("The winner had {}!", soln);
    println!("The loser had {}!", soln2);
}

#[derive(Default, Debug, Clone)]
struct BingoGame {
    called_numbers: Vec<u32>,
    boards: Vec<BingoBoard>,
}

#[derive(Debug, Clone, Copy)]
struct BingoBoard {
    board: [[i32; 5]; 5],
    num_per_row: [u8; 5],
    num_per_col: [u8; 5],
    solved: bool,
}

impl Default for BingoBoard {
    fn default() -> Self {
        BingoBoard {
            board: [[0; 5], [0; 5], [0; 5], [0; 5], [0; 5]],
            num_per_row: [0; 5],
            num_per_col: [0; 5],
            solved: false,
        }
    }
}

impl BingoBoard {
    fn solved(&self) -> bool {
        //check rows and cols
        for i in 0..5 {
            if self.num_per_row[i] == 5 {
                return true;
            }
            if self.num_per_col[i] == 5 {
                return true;
            }
        }
        //check diags
        false
    }

    // the score is the sum of all remaining (not called) numbers * the winning number call,
    // this calculates the sum of the remaining numbers
    fn calculate_score(&self) -> u32 {
        let mut sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col] != -1 {
                    sum += self.board[row][col] as u32;
                }
            }
        }
        println!("{:?}", self);
        sum
    }

    // generic wrapper for setting a location
    fn set_loc(&mut self, row: usize, col: usize, val: i32) {
        self.board[row][col] = val as i32;
    }

    //set the called number to -1 in the board and incrament associated win trackers
    fn number_called(&mut self, val: u32) {
        'outer: for row in 0..5 as usize {
            for col in 0..5 as usize {
                if self.board[row][col] == val as i32 {
                    self.num_per_row[row] += 1;
                    self.num_per_col[col] += 1;
                    self.set_loc(row, col, -1);
                    break 'outer;
                }
            }
        }
        self.solved = self.solved()
    }
}

fn populate_bingo(file_name: &str) -> BingoGame {
    //read from file
    let input = fs::read_to_string(file_name).expect("Please give a valid file name");
    let mut input_boards: Vec<&str> = input.split("\n\n").collect();
    input_boards = input_boards.into_iter().rev().collect();
    let called_numbers = input_boards.pop().unwrap();

    //initialize bing game
    let mut bingo_game = BingoGame {
        called_numbers: Vec::new(),
        boards: Vec::new(),
    };

    //input number parsing
    let called_numbers: Vec<&str> = called_numbers.split(',').collect();
    for num in called_numbers {
        bingo_game
            .called_numbers
            .push(num.parse::<u32>().expect("Could not convert called number"));
    }
    //    bingo_game.called_numbers = bingo_game.called_numbers.into_iter().rev().collect(); // restore oreder

    //bingo board parsing
    for input_board in input_boards {
        let mut new_board: BingoBoard = Default::default();
        let rows: Vec<&str> = input_board.split('\n').collect();
        for (row, row_val) in rows.iter().enumerate() {
            let cols: Vec<&str> = row_val.split_whitespace().collect();
            for (col, square_val) in cols.iter().enumerate() {
                new_board.set_loc(
                    row,
                    col,
                    square_val
                        .parse::<i32>()
                        .expect("Could not convert to number correctly"),
                );
            }
        }
        bingo_game.boards.push(new_board);
    }
    bingo_game
}

// Part 1 solution, find the score of the winning board
fn play(game: &mut BingoGame) -> u32 {
    for num in game.called_numbers.iter_mut() {
        for board in game.boards.iter_mut() {
            board.number_called(*num);
            if board.solved {
                return board.calculate_score() * *num;
            }
        }
    }
    return 0;
}

// Part 2 solution, find the score of the board that is the worst board to pick
fn lose(game: &mut BingoGame) -> u32 {
    let mut boards = game.boards.clone();
    for num in game.called_numbers.iter_mut() {
        let mut next_boards: Vec<BingoBoard> = Vec::new();
        let num_boards: usize = boards.len();
        for board in boards.iter_mut() {
            board.number_called(*num);
            if !board.solved {
                next_boards.push(*board);
            } else if board.solved && num_boards == 1 {
                return board.calculate_score() * *num;
            }
        }
        boards = next_boards;
    }
    return 0;
}
