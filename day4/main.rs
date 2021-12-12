use std::fs;
fn main() {
    part_1();
    part_2();
}

struct Board {
    state: Vec<Vec<(u32, bool)>>,
    won: bool,
}

impl Board {
    fn calculate_result(&self) -> u32 {
        let mut sum = 0;
        for row_index in 0..self.state.len() {
            for cell_index in 0..self.state[row_index].len() {
                if !self.state[row_index][cell_index].1 {
                    sum += self.state[row_index][cell_index].0;
                }
            }
        }

        return sum;
    }

    fn print(&self) {
        println!("board:");
        for row in &self.state {
            for cell in row {
                eprint!("; cell = ({:#?}, {:#?})", cell.0, cell.1);
            }
            println!();
        }
    }
    fn is_win(&self, row_index: usize, cell_index: usize) -> bool {
        let mut vertical_win = true;
        let mut horizontal_win = true;
        for i in 0..self.state.len() {
            if !self.state[i][cell_index].1 {
                vertical_win = false;
                break;
            }
        }

        for i in 0..self.state[row_index].len() {
            if !self.state[row_index][i].1 {
                horizontal_win = false;
                break;
            }
        }
        return vertical_win || horizontal_win;
    }

    fn pick(&mut self, number: u32) -> Option<u32> {
        for row_index in 0..self.state.len() {
            for cell_index in 0..self.state[row_index].len() {
                if self.state[row_index][cell_index].0 == number {
                    self.state[row_index][cell_index] = (self.state[row_index][cell_index].0, true);

                    if self.is_win(row_index, cell_index) {
                        self.won = true;
                        return Some(number * self.calculate_result());
                    } else {
                        return None;
                    }
                }
            }
        }

        return None;
    }

    fn add_cell(&mut self, board_row: usize, num: u32) {
        self.state[board_row].push((num, false))
    }

    fn new<'a>() -> Board {
        Board {
            state: vec![vec![], vec![], vec![], vec![], vec![]],
            won: false,
        }
    }
}

fn parse_boards(file: &str) -> (Vec<u32>, Vec<Board>) {
    let contents = fs::read_to_string(file).unwrap();
    let mut lines = contents.lines();
    let bingo: Vec<u32> = lines
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut boards: Vec<Board> = Vec::new();
    let mut current_board: Board = Board::new();
    let mut board_row = 0;
    lines.next().unwrap();
    for line in lines {
        if line.is_empty() {
            boards.push(current_board);
            current_board = Board::new();
            board_row = 0;
        } else {
            let numbers: Vec<u32> = line
                .split(" ")
                .map(|x| x.trim())
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .collect();
            for num in numbers {
                current_board.add_cell(board_row, num);
            }
            board_row += 1;
        }
    }
    boards.push(current_board);
    (bingo, boards)
}

fn part_1() -> u32 {
    let (bingo, mut boards) = parse_boards("./input2.txt");

    let mut win = false;
    for num in bingo {
        for i in 0..boards.len() {
            match boards[i].pick(num) {
                Some(result) => {
                    println!("result = {:#?}", result);
                    win = true;
                    break;
                }
                _ => {}
            }
        }
        if win {
            break;
        }
    }
    for board in &boards {
        board.print();
    }

    return 0;
}

fn part_2() {
    let (bingo, mut boards) = parse_boards("./input2.txt");

    for num in bingo {
        boards = boards.into_iter().filter(|b| !b.won).collect();
        for i in 0..boards.len() {
            if !boards[i].won {
                match boards[i].pick(num) {
                    Some(result) => {
                        if boards.len() == 1 {
                            eprintln!("result part 2= {:#?}", result);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
