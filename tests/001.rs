use chessme::{ChessBoard, Player, parse_position};

fn setup_test_game() -> ChessBoard { 
    ChessBoard::new()
}

#[test]
fn test_game_checkmate_f3_qh4_1() {
    // Simulate the moves for the sequence: f3 e6 g4 Qh4#
    
    let mut board = setup_test_game();

    let positions = vec![
        ("f2","f3"), // White plays f3 
        ("e7","e6"), // Black plays e6
        ("g2","g4"), // Black plays g4
        ("d8","h4"), // White plays h4
    ];
    
    for (start_str, end_str) in positions {
        let start = parse_position(start_str).expect("Postion can't be reached");
        let end = parse_position(end_str).expect("Postion can't be reached");
        board.move_piece(start, end).unwrap();
    }
    
    let result = board.result(Player::Black); // Checking for black's result after the moves
    assert_eq!(result, "0-1"); // Black wins
}

#[test]
fn test_game_checkmate_f3_qh4_2() {
    // Simulate the moves for the sequence: f3 e5 g4 Qh4#
    
    let mut board = setup_test_game();

    let positions = vec![
        ("f2","f3"), // White plays f3 
        ("e7","e5"), // Black plays e5
        ("g2","g4"), // Black plays g4
        ("d8","h4"), // White plays h4
    ];
    
    for (start_str, end_str) in positions {
        let start = parse_position(start_str).expect("Postion can't be reached");
        let end = parse_position(end_str).expect("Postion can't be reached");
        board.move_piece(start, end).unwrap();
    }
    
    let result = board.result(Player::Black); // Checking for black's result after the moves
    assert_eq!(result, "0-1"); // Black wins
}

#[test]
fn test_game_checkmate_f4_qh4_1() {
    // Simulate the moves for the sequence: f4 e6 g4 Qh4#
    
    let mut board = setup_test_game();

    let positions = vec![
        ("f2","f4"), // White plays f4
        ("e7","e6"), // Black plays e6
        ("g2","g4"), // White plays g4
        ("d8","h4"), // Black plays Qh4#
    ];
    
    for (start_str, end_str) in positions {
        let start = parse_position(start_str).expect("Postion can't be reached");
        let end = parse_position(end_str).expect("Postion can't be reached");
        let _ = board.move_piece(start, end);
    }

    let result = board.result(Player::Black); // Checking for black's result after the moves
    assert_eq!(result, "0-1"); // Black wins
}

#[test]
fn test_game_checkmate_f4_qh4_2() {
    // Simulate the moves for the sequence: f4 e5 g4 Qh4#
    
    let mut board = setup_test_game();

    let positions = vec![
        ("f2","f4"), // White plays f4
        ("e7","e5"), // Black plays e5
        ("g2","g4"), // White plays g4
        ("d8","h4"), // Black plays Qh4#
    ];
    
    for (start_str, end_str) in positions {
        let start = parse_position(start_str).expect("Postion can't be reached");
        let end = parse_position(end_str).expect("Postion can't be reached");
        board.move_piece(start, end).unwrap();
    }
    
    let result = board.result(Player::Black); // Checking for black's result after the moves
    assert_eq!(result, "0-1"); // Black wins
}

#[test]
fn test_game_checkmate_g4_qh4_1() {
    // Simulate the moves for the sequence: g4 e6 f3 Qh4#
    
    let mut board = setup_test_game();

    let positions = vec![
        ("g2","g4"), // White plays g4
        ("e7","e6"), // Black plays e6
        ("f2","f3"), // White plays f3
        ("d8","h4"), // Black plays Qh4#
    ];
    
    for (start_str, end_str) in positions {
        let start = parse_position(start_str).expect("Postion can't be reached");
        let end = parse_position(end_str).expect("Postion can't be reached");
        board.move_piece(start, end).unwrap();
    }
    
    let result = board.result(Player::Black); // Checking for black's result after the moves
    assert_eq!(result, "0-1"); // Black wins
}


#[test]
fn test_game_checkmate_g4_qh4_2() {
    // Simulate the moves for the sequence: g4 e6 f4 Qh4#
    
    let mut board = setup_test_game();

    let positions = vec![
        ("g2","g4"), // White plays g4
        ("e7","e6"), // Black plays e6
        ("f2","f4"), // White plays f4
        ("d8","h4"), // Black plays Qh4#
    ];
    
    for (start_str, end_str) in positions {
        let start = parse_position(start_str).expect("Postion can't be reached");
        let end = parse_position(end_str).expect("Postion can't be reached");
        board.move_piece(start, end).unwrap();
    }
    
    let result = board.result(Player::Black); // Checking for black's result after the moves
    assert_eq!(result, "0-1"); // Black wins
}

#[test]
fn test_game_checkmate_g4_qh4_3() {
    // Simulate the moves for the sequence: g4 e5 f3 Qh4#
    
    let mut board = setup_test_game();

    let positions = vec![
        ("g2","g4"), // White plays g4
        ("e7","e5"), // Black plays e5
        ("f2","f3"), // White plays f3
        ("d8","h4"), // Black plays Qh4#
    ];
    
    for (start_str, end_str) in positions {
        let start = parse_position(start_str).expect("Postion can't be reached");
        let end = parse_position(end_str).expect("Postion can't be reached");
        board.move_piece(start, end).unwrap();
    }
    
    let result = board.result(Player::Black); // Checking for black's result after the moves
    assert_eq!(result, "0-1"); // Black wins
}


#[test]
fn test_game_checkmate_g4_qh4_4() {
    // Simulate the moves for the sequence: g4 e5 f4 Qh4#
    
    let mut board = setup_test_game();

    let positions = vec![
        ("g2","g4"), // White plays g4
        ("e7","e5"), // Black plays e5
        ("f2","f4"), // White plays f4
        ("d8","h4"), // Black plays Qh4#
    ];
    
    for (start_str, end_str) in positions {
        let start = parse_position(start_str).expect("Postion can't be reached");
        let end = parse_position(end_str).expect("Postion can't be reached");
        board.move_piece(start, end).unwrap();
    }
    
    let result = board.result(Player::Black); // Checking for black's result after the moves
    assert_eq!(result, "0-1"); // Black wins
}