use std::io::{self, Write};
use chessme::{ChessBoard, Player, parse_position}; // Re-export game structs to be accessible


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
                current_player = board.next_player(current_player);
            }
            _ => println!("Invalid move. Try again."),
        }
    }

    // Write PGN to string
    let result = board.result(current_player);
    let pgn:String = board.write_to_pgn(white_player, black_player, &result);
    println!("\nPGN:\n{}", pgn);
}

