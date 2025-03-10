use std::io::{self, Write};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    White,
    Black,
}
// Eq, Hash
#[derive(Copy, Clone, Debug, PartialEq)]
enum Piece {
    King(Player),
    Queen(Player),
    Rook(Player),
    Bishop(Player),
    Knight(Player),
    Pawn(Player),
}

struct ChessBoard {
    board: [[Option<Piece>; 8]; 8],
}

impl ChessBoard {
    fn new() -> Self {
        let mut board = [[None; 8]; 8];

        // Place white pieces
        board[0][0] = Some(Piece::Rook(Player::White));
        board[0][1] = Some(Piece::Knight(Player::White));
        board[0][2] = Some(Piece::Bishop(Player::White));
        board[0][3] = Some(Piece::Queen(Player::White));
        board[0][4] = Some(Piece::King(Player::White));
        board[0][5] = Some(Piece::Bishop(Player::White));
        board[0][6] = Some(Piece::Knight(Player::White));
        board[0][7] = Some(Piece::Rook(Player::White));
        for i in 0..8 {
            board[1][i] = Some(Piece::Pawn(Player::White));
        }

        // Place black pieces
        board[7][0] = Some(Piece::Rook(Player::Black));
        board[7][1] = Some(Piece::Knight(Player::Black));
        board[7][2] = Some(Piece::Bishop(Player::Black));
        board[7][3] = Some(Piece::Queen(Player::Black));
        board[7][4] = Some(Piece::King(Player::Black));
        board[7][5] = Some(Piece::Bishop(Player::Black));
        board[7][6] = Some(Piece::Knight(Player::Black));
        board[7][7] = Some(Piece::Rook(Player::Black));
        for i in 0..8 {
            board[6][i] = Some(Piece::Pawn(Player::Black));
        }

        ChessBoard { board }
    }

    fn print(&self) {
        for row in self.board.iter().rev() {
            for cell in row.iter() {
                match cell {
                    Some(piece) => {
                        let symbol = match piece {
                            Piece::King(player) => if *player == Player::White { "K" } else { "k" },
                            Piece::Queen(player) => if *player == Player::White { "Q" } else { "q" },
                            Piece::Rook(player) => if *player == Player::White { "R" } else { "r" },
                            Piece::Bishop(player) => if *player == Player::White { "B" } else { "b" },
                            Piece::Knight(player) => if *player == Player::White { "N" } else { "n" },
                            Piece::Pawn(player) => if *player == Player::White { "P" } else { "p" },
                        };
                        print!("{} ", symbol);
                    },
                    None => print!(". "),
                }
            }
            println!();
        }
    }

    fn move_piece(&mut self, start: (usize, usize), end: (usize, usize)) -> Result<(), String> {
        let start_piece = self.board[start.0][start.1].ok_or("No piece at start position")?;
        self.board[end.0][end.1] = Some(start_piece);
        self.board[start.0][start.1] = None;
        Ok(())
    }

    fn is_valid_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        // Simple validation logic (just check if the move is on the board)
        start.0 < 8 && start.1 < 8 && end.0 < 8 && end.1 < 8
    }
}

fn main() {
    let mut board = ChessBoard::new();
    let mut current_player = Player::White;

    loop {
        board.print();
        println!("{:?}'s turn", current_player);

        // Read user input
        let mut input = String::new();
        print!("Enter move (e.g., 'e2 e4'): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            println!("Invalid input. Please enter in format 'e2 e4'.");
            continue;
        }

        let start = parse_position(parts[0]);
        let end = parse_position(parts[1]);

        match (start, end) {
            (Some(start_pos), Some(end_pos)) if board.is_valid_move(start_pos, end_pos) => {
                if let Err(err) = board.move_piece(start_pos, end_pos) {
                    println!("Error: {}", err);
                    continue;
                }
                current_player = match current_player {
                    Player::White => Player::Black,
                    Player::Black => Player::White,
                };
            }
            _ => println!("Invalid move. Try again."),
        }
    }
}

fn parse_position(position: &str) -> Option<(usize, usize)> {
    let chars: Vec<char> = position.chars().collect();
    if chars.len() != 2 {
        return None;
    }

    let column = chars[0] as usize - 'a' as usize;
    let row = chars[1] as usize - '1' as usize;

    if column < 8 && row < 8 {
        Some((row, column))
    } else {
        None
    }
}
