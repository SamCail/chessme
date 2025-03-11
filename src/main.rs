use std::io::{self, Write};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Player {
    White,
    Black,
}

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
        print!("{}[2J", 27 as char);
        for row in self.board.iter().rev() {
            for cell in row.iter() {
                match cell {
                    Some(piece) => {
                        let symbol = match piece {
                            Piece::King(player) => if *player == Player::White { "♔" } else { "♚" },
                            Piece::Queen(player) => if *player == Player::White { "♕" } else { "♛" },
                            Piece::Rook(player) => if *player == Player::White { "♖" } else { "♜" },
                            Piece::Bishop(player) => if *player == Player::White { "♗" } else { "♝" },
                            Piece::Knight(player) => if *player == Player::White { "♘" } else { "♞" },
                            Piece::Pawn(player) => if *player == Player::White { "♙" } else { "♟" },
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

    fn is_valid_move(&self, start: (usize, usize), end: (usize, usize), current_player: Player) -> bool {
        let piece = match self.board[start.0][start.1] {
            Some(piece) => piece,
            None => return false,
        };

        // Make sure the piece belongs to the current player
        if self.is_opponent(piece, current_player) {
            return false;
        }

        let is_valid: bool = match piece {
            Piece::King(_) => self.is_valid_king_move(start, end),
            Piece::Queen(_) => self.is_valid_queen_move(start, end),
            Piece::Rook(_) => self.is_valid_rook_move(start, end),
            Piece::Bishop(_) => self.is_valid_bishop_move(start, end),
            Piece::Knight(_) => self.is_valid_knight_move(start, end),
            Piece::Pawn(player) => self.is_valid_pawn_move(start, end, player),
        };

        let fen = self.write_to_fen(current_player);
        println!(">> {fen}");
        
        is_valid
    }

    fn is_valid_king_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let dx = (end.0 as isize - start.0 as isize).abs();
        let dy = (end.1 as isize - start.1 as isize).abs();
        dx <= 1 && dy <= 1
    }

    fn is_valid_queen_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        self.is_valid_rook_move(start, end) || self.is_valid_bishop_move(start, end)
    }

    fn is_valid_rook_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        if start.0 != end.0 && start.1 != end.1 {
            return false;
        }

        let (r1, c1) = start;
        let (r2, c2) = end;

        // Check if there are pieces in the way
        if r1 == r2 {
            let range = if c1 < c2 { c1 + 1..c2 } else { c2 + 1..c1 };
            for col in range {
                if self.board[r1][col].is_some() {
                    return false;
                }
            }
        } else {
            let range = if r1 < r2 { r1 + 1..r2 } else { r2 + 1..r1 };
            for row in range {
                if self.board[row][c1].is_some() {
                    return false;
                }
            }
        }

        true
    }

    fn is_valid_bishop_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let dx = (end.0 as isize - start.0 as isize).abs();
        let dy = (end.1 as isize - start.1 as isize).abs();

        if dx != dy {
            return false;
        }

        let (r1, c1) = start;
        let (r2, c2) = end;

        let row_step = if r2 > r1 { 1 } else { -1 };
        let col_step = if c2 > c1 { 1 } else { -1 };

        let mut row = r1 as isize + row_step;
        let mut col = c1 as isize + col_step;

        while row != r2 as isize && col != c2 as isize {
            if self.board[row as usize][col as usize].is_some() {
                return false;
            }
            row += row_step;
            col += col_step;
        }

        true
    }

    fn is_valid_knight_move(&self, start: (usize, usize), end: (usize, usize)) -> bool {
        let dx = (end.0 as isize - start.0 as isize).abs();
        let dy = (end.1 as isize - start.1 as isize).abs();
        (dx == 2 && dy == 1) || (dx == 1 && dy == 2)
    }

    fn is_valid_pawn_move(&self, start: (usize, usize), end: (usize, usize), player: Player) -> bool {
        let direction:isize = match player {
            Player::White => 1,
            Player::Black => -1,
        };

        let (r1, c1) = start;
        let (r2, c2) = end;
        
        if c1 == c2 {
            // Pawn moves straight
            if r2 == r1.wrapping_add_signed(direction) && self.board[r2][c2].is_none() {
                return true;
            }

            // Pawn can move two squares from its initial position
            if r1 == 1 && player == Player::White && r2 == r1.wrapping_add_signed(2 * direction) && self.board[r2][c2].is_none() {
                return self.board[r1.wrapping_add_signed(direction)][c1].is_none();
            }
            if r1 == 6 && player == Player::Black && r2 == r1.wrapping_add_signed(2 * direction) && self.board[r2][c2].is_none() {
                return self.board[r1.wrapping_add_signed(direction)][c1].is_none();
            }
        }

        // Pawn captures diagonally
        if (r2 == r1.wrapping_add_signed(direction)) && (c2 == c1 + 1 || c2 == c1 - 1) {
            if let Some(piece) = self.board[r2][c2] {
                return self.is_opponent(piece, player);
            }
        }

        false
    }

    fn is_opponent(&self, piece:Piece,  player: Player) -> bool {
        match piece {
            Piece::King(p) => p != player,
            Piece::Queen(p) => p != player,
            Piece::Rook(p) => p != player,
            Piece::Bishop(p) => p != player,
            Piece::Knight(p) => p != player,
            Piece::Pawn(p) => p != player,
        }
    }

    fn write_to_fen(&self, current_player: Player) -> String {
        let mut fen = String::new();

        // Piece Placement
        for row in self.board.iter().rev() {
            let mut empty_count = 0;
            for cell in row.iter() {
                match cell {
                    Some(piece) => {
                        if empty_count > 0 {
                            fen.push_str(&empty_count.to_string());
                            empty_count = 0;
                        }
                        let symbol = match piece {
                            Piece::King(player) => if *player == Player::White { "K" } else { "k" },
                            Piece::Queen(player) => if *player == Player::White { "Q" } else { "q" },
                            Piece::Rook(player) => if *player == Player::White { "R" } else { "r" },
                            Piece::Bishop(player) => if *player == Player::White { "B" } else { "b" },
                            Piece::Knight(player) => if *player == Player::White { "N" } else { "n" },
                            Piece::Pawn(player) => if *player == Player::White { "P" } else { "p" },
                        };
                        fen.push_str(symbol);
                    },
                    None => {
                        empty_count += 1;
                    },
                }
            }
            if empty_count > 0 {
                fen.push_str(&empty_count.to_string());
            }
            fen.push('/');
        }
        fen.pop(); // Remove the last '/'.

        // Active color
        fen.push(' ');
        fen.push(if current_player == Player::White { 'w' } else { 'b' });

        // TODO: Add Castling, En Passant, Halfmove Clock, and Fullmove Number

        fen
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
            (Some(start_pos), Some(end_pos)) if board.is_valid_move(start_pos, end_pos, current_player) => {
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
