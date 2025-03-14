use std::io::{self, Write};
use std::collections::VecDeque;

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
#[derive(Clone)]
struct ChessBoard {
    board: [[Option<Piece>; 8]; 8],
    moves_history: VecDeque<String>  // Track the moves in PGN format
}

impl ChessBoard {
    fn new() -> Self {
        let moves_history:VecDeque<String> = VecDeque::new();
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

        ChessBoard { board , moves_history}
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

    fn move_if_valid(&mut self, start: (usize, usize), end: (usize, usize), current_player: Player) -> bool {
        let is_valid:bool = self.clone().is_valid_move(start, end, current_player);
        
        let piece = match self.board[start.0][start.1] {
            Some(piece) => piece,
            None => return false,
        };

        if is_valid {
            let fen = self.write_to_fen(current_player);
            println!(">> {fen}");

            self.add_move(piece, end, current_player);

        };

        is_valid
    }

    fn is_valid_move(self, start: (usize, usize), end: (usize, usize), current_player: Player) -> bool {
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

    fn write_to_pgn(&self, white_player: &str, black_player: &str, result: &str) -> String {
        // Construct PGN header
        let mut pgn = format!(
            "[Event \"Chess Game\"]\n[Site \"localhost\"]\n[Date \"2025.03.11\"]\n[Round \"1\"]\n[White \"{}\"]\n[Black \"{}\"]\n[Result \"{}\"]\n\n",
            white_player, black_player, result
        );

        // Add the moves
        let mut move_counter = 1;
        for chunk in self.moves_history.iter() {
            pgn.push_str(&format!("{}. {} ", move_counter, chunk));
            move_counter += 1;
        }

        pgn
    }

    fn add_move(&mut self, piece:Piece,end: (usize, usize), current_player: Player) {
        let letter = match piece {
            Piece::King(current_player) => if current_player == Player::White { "K" } else { "k" },
            Piece::Queen(current_player) => if current_player == Player::White { "Q" } else { "q" },
            Piece::Rook(current_player) => if current_player == Player::White { "R" } else { "r" },
            Piece::Bishop(current_player) => if current_player == Player::White { "B" } else { "b" },
            Piece::Knight(current_player) => if current_player == Player::White { "N" } else { "n" },
            Piece::Pawn(current_player) => if current_player == Player::White { "" } else { "" },
        };
        let ending = match current_player {
            Player::White => " ",
            Player::Black => "\n"
        };
        let column = (b'a' + end.0 as u8) as char;  // Convert the column index to a letter
        let row = end.1.to_string();  // Convert the row index to a string

        let move_notation : String= format!("{letter}{column}{row}{ending}");
        
        self.moves_history.push_back(move_notation);
    }

    fn result(&self, current_player: Player) -> String {
        let current_player_cant_move:bool = self.has_legal_moves(current_player);
        // Check if the current player is in checkmate
        // If in check, but no legal moves, it's a checkmate
        if self.is_check(current_player) && !current_player_cant_move {
            // If it's the white player's turn and they're in check, black wins
            if current_player == Player::White {
                return "0-1".to_string();
            } else {
                return "1-0".to_string();
            }
        }
        // Check if it's a stalemate (no legal moves, and not in check)
        if !current_player_cant_move {
            return "1/2-1/2".to_string();
        }

        // If no conclusion yet, game is still ongoing
        "*".to_string()
    }

    fn is_check(&self, player: Player) -> bool {
        // Find the player's king position
        let king_position = self.find_king_position(player);

        // Check if any opposing piece can attack the king
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[row][col] {
                    match piece {
                        Piece::King(some_player) => if some_player != player { return self.is_attack_possible(piece, (row, col), king_position, player) },
                        Piece::Queen(some_player) => if some_player != player { return self.is_attack_possible(piece, (row, col), king_position, player) },
                        Piece::Rook(some_player) => if some_player != player { return self.is_attack_possible(piece, (row, col), king_position, player) },
                        Piece::Bishop(some_player) => if some_player != player { return self.is_attack_possible(piece, (row, col), king_position, player) },
                        Piece::Knight(some_player) => if some_player != player { return self.is_attack_possible(piece, (row, col), king_position, player) },
                        Piece::Pawn(some_player) => if some_player != player { return self.is_attack_possible(piece, (row, col), king_position, player) }
                    };
                }
            }
        }
        
        return false
    }
    fn find_king_position(&self, player: Player) -> (usize, usize) {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[row][col] {
                    if let Piece::King(piece_player) = piece {
                        if piece_player == player {
                            return (row, col);
                        }
                    }
                }
            }
        }
        panic!("King not found on the board!");
    }
    fn is_attack_possible(&self, piece: Piece, start: (usize, usize), target: (usize, usize), player: Player) -> bool {
        // Implement the movement rules for each piece (Rook, Knight, Bishop, Queen, King, Pawn)
        match piece {
            Piece::King(_) => self.is_valid_king_move(start, target),
            Piece::Queen(_) => self.is_valid_queen_move(start, target),
            Piece::Rook(_) => self.is_valid_rook_move(start, target),
            Piece::Bishop(_) => self.is_valid_bishop_move(start, target),
            Piece::Knight(_) => self.is_valid_knight_move(start, target),
            Piece::Pawn(_) => self.is_valid_pawn_move(start, target, player)
        }
    }

    fn has_legal_moves(&self, player: Player) -> bool {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.board[row][col] {
                    match piece {
                        Piece::King(some_player) => if some_player == player { for target_row in 0..8 {
                            for target_col in 0..8 {
                                if self.clone().is_valid_move((row, col), (target_row, target_col), player) {
                                    return true; // Found a valid move
                                }
                            }
                        } },
                        Piece::Queen(some_player) => if some_player == player { for target_row in 0..8 {
                            for target_col in 0..8 {
                                if self.clone().is_valid_move((row, col), (target_row, target_col), player) {
                                    return true; // Found a valid move
                                }
                            }
                        } },
                        Piece::Rook(some_player) => if some_player == player { for target_row in 0..8 {
                            for target_col in 0..8 {
                                if self.clone().is_valid_move((row, col), (target_row, target_col), player) {
                                    return true; // Found a valid move
                                }
                            }
                        } },
                        Piece::Bishop(some_player) => if some_player == player { for target_row in 0..8 {
                            for target_col in 0..8 {
                                if self.clone().is_valid_move((row, col), (target_row, target_col), player) {
                                    return true; // Found a valid move
                                }
                            }
                        } },
                        Piece::Knight(some_player) => if some_player == player { for target_row in 0..8 {
                            for target_col in 0..8 {
                                if self.clone().is_valid_move((row, col), (target_row, target_col), player) {
                                    return true; // Found a valid move
                                }
                            }
                        } },
                        Piece::Pawn(some_player) => if some_player == player { for target_row in 0..8 {
                            for target_col in 0..8 {
                                if self.clone().is_valid_move((row, col), (target_row, target_col), player) {
                                    return true; // Found a valid move
                                }
                            }
                        } }
                    };
                }
            }
        }
        false
    }
}

fn main() {
    let mut board = ChessBoard::new();
    let mut current_player = Player::White;

    // Example to track moves and write PGN
    let white_player = "Player 1";
    let black_player = "Player 2";

    loop {
        board.print();
        println!("{:?}'s turn", current_player);

        // Read user input
        let mut input = String::new();
        print!("Enter move (e.g., 'e2 e4'): ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        if parts.len() == 1 && parts[0] == "stop" {
            println!("Stopping the game.");
            break;
        }

        if parts.len() != 2 {
            println!("Invalid input. Please enter in format 'e2 e4'.");
            continue;
        }

        let start = parse_position(parts[0]);
        let end = parse_position(parts[1]);

        match (start, end) {
            (Some(start_pos), Some(end_pos)) if board.move_if_valid(start_pos, end_pos, current_player) => {
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

    // Write PGN to string
    let pgn:String = board.write_to_pgn(white_player, black_player, &board.result(current_player));
    println!("\nPGN:\n{}", pgn);
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
