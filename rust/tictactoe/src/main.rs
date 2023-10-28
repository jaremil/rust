use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Occupied(Player),
}

struct Board {
    cells: [[Cell; 3]; 3],
}

impl Board {
    fn new() -> Self {
        Board {
            cells: [[Cell::Empty; 3]; 3],
        }
    }

    fn display(&self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    Cell::Empty => print!("_ "),
                    Cell::Occupied(Player::X) => print!("X "),
                    Cell::Occupied(Player::O) => print!("O "),
                }
            }
            println!();
        }
    }

    fn make_move(&mut self, player: Player, row: usize, col: usize) -> Result<(), &str> {
        if row < 3 && col < 3 && self.cells[row][col] == Cell::Empty {
            self.cells[row][col] = Cell::Occupied(player);
            Ok(())
        } else {
            Err("Invalid move!")
        }
    }

    fn check_winner(&self) -> Option<Player> {
        for i in 0..3 {
            if self.cells[i][0] != Cell::Empty
                && self.cells[i][0] == self.cells[i][1]
                && self.cells[i][0] == self.cells[i][2]
            {
                return Some(match self.cells[i][0] {
                    Cell::Occupied(player) => player,
                    _ => unreachable!(),
                });
            }
            if self.cells[0][i] != Cell::Empty
                && self.cells[0][i] == self.cells[1][i]
                && self.cells[0][i] == self.cells[2][i]
            {
                return Some(match self.cells[0][i] {
                    Cell::Occupied(player) => player,
                    _ => unreachable!(),
                });
            }
        }

        if self.cells[0][0] != Cell::Empty
            && self.cells[0][0] == self.cells[1][1]
            && self.cells[0][0] == self.cells[2][2]
        {
            return Some(match self.cells[0][0] {
                Cell::Occupied(player) => player,
                _ => unreachable!(),
            });
        }

        if self.cells[0][2] != Cell::Empty
            && self.cells[0][2] == self.cells[1][1]
            && self.cells[0][2] == self.cells[2][0]
        {
            return Some(match self.cells[0][2] {
                Cell::Occupied(player) => player,
                _ => unreachable!(),
            });
        }

        None
    }
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::X;

    loop {
        println!("Current Board:");
        board.display();

        println!("Player {:?}, enter your move (row and column, 0-2 separated by space):", current_player);

        let mut input = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let coordinates: Vec<&str> = input.trim().split_whitespace().collect();
        if coordinates.len() != 2 {
            println!("Invalid input. Please enter row and column numbers separated by space.");
            continue;
        }

        let row: usize = match coordinates[0].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid row number.");
                continue;
            }
        };

        let col: usize = match coordinates[1].parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid column number.");
                continue;
            }
        };

        match board.make_move(current_player, row, col) {
            Ok(_) => {
                match board.check_winner() {
                    Some(player) => {
                        println!("Player {:?} wins!", player);
                        break;
                    }
                    None => {
                        // Switch player
                        current_player = match current_player {
                            Player::X => Player::O,
                            Player::O => Player::X,
                        };
                    }
                }
            }
            Err(err) => println!("{}", err),
        }
    }
}