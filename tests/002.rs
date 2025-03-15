use chessme::{ChessBoard, Player, parse_position};


fn setup_test_game() -> ChessBoard { 
    ChessBoard::new()
}

#[test]
fn test_game_checkmate_qh5_qxe5() {
    // Simulate the moves for the sequence: e4 e5 Qh5 Ke7 Qxe5#
    
    let mut board = setup_test_game();

    let positions = vec![
        ("e2","e4"), // White plays e4
        ("e7","e5"), // Black plays e5
        ("d1","h5"), // White plays Qh5
        ("e8","e7"), // Black plays Ke7
        ("h5","e5"), // White plays Qxe5#
    ];
    
    for (start_str, end_str) in positions {
        let start = parse_position(start_str).expect("Postion can't be reached");
        let end = parse_position(end_str).expect("Postion can't be reached");
        board.move_piece(start, end).unwrap();
    }

    let result = board.result(Player::White); // Checking for white's result after the moves
    assert_eq!(result, "1-0"); // White wins
}
