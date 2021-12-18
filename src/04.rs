use std::fs;

fn main() {
    println!("Day 04-01:");
    // bingo();
    println!("Day 04-02:");
    last_winner();
}

#[derive(Clone, Copy, Debug)]
struct Entry {
    value: usize,
    marked: bool,
}

impl Entry {
    pub fn from_string(input: &str) -> Entry {
        Entry {
            value: input
                .parse::<usize>()
                .expect(&format!("Could not parse input {}!", input).to_owned()),
            marked: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Board(Vec<Vec<Entry>>);

impl Board {
    pub fn mark(&self, value: usize) -> Self {
        Board(
            self.0
                .iter()
                .map(|v| {
                    v.iter()
                        .map(|e| {
                            if e.value == value {
                                Entry {
                                    value,
                                    marked: true,
                                }
                            } else {
                                *e
                            }
                        })
                        .collect()
                })
                .collect::<Vec<Vec<Entry>>>(),
        )
    }

    fn check_columns(&self) -> usize {
        self.0
            .iter()
            .map(|v| v.iter().all(|e| e.marked))
            .filter(|&v| v == true)
            .collect::<Vec<bool>>()
            .len()
    }

    fn check_rows(&self) -> usize {
        let mut counter = 0;
        for index in 0..self.0[0].len() {
            if self.0.iter().map(|v| v[index]).all(|e| e.marked == true) {
                counter += 1;
            }
        }
        counter
    }

    pub fn check(&self) -> usize {
        self.check_columns() + self.check_rows()
    }

    pub fn new() -> Board {
        Board(vec![])
    }

    pub fn parse_row(&mut self, row: &str) {
        self.0.push(
            row.trim()
                .split(" ")
                .map(str::trim)
                .filter(|&e| !e.is_empty())
                .map(Entry::from_string)
                .collect::<Vec<Entry>>(),
        )
    }
}

/// Day 04-01
fn bingo() {
    let contents =
        fs::read_to_string("./data/04.txt").expect("Something went wrong reading the file");

    let numbers: Vec<usize> = contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    let board_input: Vec<&str> = contents.lines().skip(2).collect();

    let mut boards: Vec<Board> = vec![];
    let mut current_board = Board::new();
    for row in board_input {
        if row.is_empty() {
            boards.push(current_board);
            current_board = Board::new();
        } else {
            current_board.parse_row(row);
        }
    }
    boards.push(current_board);

    for number in numbers {
        let winner = boards
            .iter()
            .map(|b| b.mark(number))
            .map(|b| (b.check(), b))
            .filter(|(n, _b)| n > &0)
            .reduce(|a, b| if a.0 > b.0 { a } else { b });

        match winner {
            Some((_n, board)) => {
                let sum_of_unmarked: usize = board
                    .0
                    .iter()
                    .flatten()
                    .filter(|e| !e.marked)
                    .map(|e| e.value)
                    .sum();

                println!("{:?}", board);
                println!("Number of bingos: {:?}", board.check_columns());
                println!("Sum of board: {}", sum_of_unmarked);
                println!("Current number: {}", number);
                println!("Result: {}", sum_of_unmarked * number);
                return;
            }
            _ => {}
        };
    }
}

/// Day 04-02
fn last_winner() {
    let contents =
        fs::read_to_string("./data/04.txt").expect("Something went wrong reading the file");

    let numbers: Vec<usize> = contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect();

    let board_input: Vec<&str> = contents.lines().skip(2).collect();

    let mut boards: Vec<Board> = vec![];
    let mut current_board = Board::new();
    for row in board_input {
        if row.is_empty() {
            boards.push(current_board);
            current_board = Board::new();
        } else {
            current_board.parse_row(row);
        }
    }
    boards.push(current_board);

    let mut last_winner: (Board, usize) = (Board::new(), 0);

    for number in numbers {
        let winner = boards
            .iter()
            .map(|b| b.mark(number))
            .map(|b| (b.check(), b))
            .enumerate()
            .filter(|(_index, (bingos, _board))| bingos > &0)
            .map(|(_index, (_bingos, board))| (board, number))
            .last();

        match winner {
            Some(winner) => last_winner = winner,
            _ => {}
        }

        boards = boards
            .iter()
            .map(|b| b.mark(number))
            .map(|b| (b.check(), b))
            .inspect(|(bingos, b)| println!("{}", bingos))
            .filter(|(bingos, _board)| bingos <= &0)
            .map(|(_bingos, board)| board.to_owned())
            .collect();
    }

    let sum_of_unmarked: usize = last_winner
        .0
         .0
        .iter()
        .flatten()
        .filter(|e| !e.marked)
        .map(|e| e.value)
        .sum();

    println!("Number of bingos: {:?}", last_winner.0.check());
    println!("Sum of board: {}", sum_of_unmarked);
    println!("Current number: {}", last_winner.1);
    println!("Result: {}", last_winner.1 * sum_of_unmarked);
}
