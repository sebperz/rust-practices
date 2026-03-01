use std::{io, iter::Enumerate};

const WHITE_PIECES: [char; 6] = ['♔', '♕', '♖', '♗', '♘', '♙'];
const BLACK_PIECES: [char; 6] = ['♚', '♛', '♜', '♝', '♞', '♟'];
const WHITE_BG: &str = "\x1B[48;2;146;131;116m";
const BLACK_BG: &str = "\x1B[48;2;60;56;54m";
const WHITE_PIECE_COLOR: &str = "\x1B[38;5;231m";
const BLACK_PIECE_COLOR: &str = "\x1B[38;5;16m";
const GREEN: &str = "\x1B[48;2;104;157;106m";
const RESET: &str = "\x1B[0m";
fn main() {
    let mut board: [[char; 8]; 8] = [
        [' ', ' ', ' ', '♚', ' ', ' ', ' ', ' '], // a1-h1
        [' ', ' ', ' ', '♜', ' ', ' ', ' ', ' '], // a2-h2
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', '♖', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', '♜', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '], // a7-h7
        [' ', ' ', ' ', '♚', ' ', ' ', ' ', ' '], // a8-h8
    ];
    let mut white_posible_movements: [[usize; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0], // a1-h1
        [0, 0, 0, 0, 0, 0, 0, 0], // a2-h2
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0], // a7-h7
        [0, 0, 0, 0, 0, 0, 0, 0], // a8-h8
    ];
    let mut black_posible_movements: [[usize; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 0], // a1-h1
        [0, 0, 0, 0, 0, 0, 0, 0], // a2-h2
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0], // a7-h7
        [0, 0, 0, 0, 0, 0, 0, 0], // a8-h8
    ];

    let mut white_pieces_pinned: [[bool; 8]; 8] = [[false; 8]; 8];
    let mut black_pieces_pinned: [[bool; 8]; 8] = [
        [false, false, false, false, false, false, false, false], // a1-h1
        [false, false, false, false, false, false, false, false], // a2-h2
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false],
        [false, false, false, false, false, false, false, false], // a7-h7
        [false, false, false, false, false, false, false, false], // a8-h8
    ];
    // print_board_pieces(&board);
    print_chess_board(board);
    rook_movement(
        (3, 3),
        &board,
        &WHITE_PIECES,
        &BLACK_PIECES,
        &mut white_posible_movements,
        &mut black_pieces_pinned,
    );
    println!("Posible movements:");
    print_posible_movements(white_posible_movements);
    println!("pings");
    print_pings(black_pieces_pinned);

    // loop {}
}

fn print_chess_board(board: [[char; 8]; 8]) {
    // ANSI escape codes for background colors (48 = background, 38 = foreground)

    // Unicode chess pieces are black by default, white pieces need to be distinguished
    // Black pieces: ♜♞♝♛♚♟ (filled)
    // White pieces: ♖♘♗♕♔♙ (outline style in Unicode, but render differently per font)

    println!("   a  b  c  d  e  f  g  h");

    for row in (0..8).rev() {
        print!("{} ", row + 1);

        for col in 0..8 {
            // Determine square color: (row + col) % 2 == 0 is traditionally a light square
            let is_white_square = (row + col) % 2 == 0;
            let bg = if is_white_square { WHITE_BG } else { BLACK_BG };

            let piece = board[row][col];
            let piece_str = if piece == ' ' {
                "   ".to_string()
            } else {
                // Determine piece color based on piece character
                let is_black_piece = BLACK_PIECES.contains(&piece);
                let piece_color = if is_black_piece {
                    BLACK_PIECE_COLOR
                } else {
                    WHITE_PIECE_COLOR
                };
                format!("{}{:^3}", piece_color, piece)
            };

            // Print: background color + piece color + piece + reset (but keep bg for space)
            // Actually, we need to reset after the space to avoid bleeding
            print!("{}{}{}", bg, piece_str, RESET);
        }

        println!(" {}", row + 1); // New line after each rank
    }
    println!("   a  b  c  d  e  f  g  h");
}

fn read_input() -> (char, char) {
    println!("Type the position of the piece to select it:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut input = input.trim().chars();
    let letter = input.next();
    let number = input.next();

    if input.next().is_some() {
        println!("Invalid input, type exactly 2 characters");
        return read_input();
    }

    let letter = match letter {
        Some(c) => c,
        None => {
            println!("First Character have to be a letter between a and h");
            return read_input();
        }
    };
    let number = match number {
        Some(c) => c,
        None => {
            println!("Second Character have to be a number between 1 and 8");
            return read_input();
        }
    };

    if !('a'..='h').contains(&letter) {
        println!("First Character have to be a letter between a and h");
        return read_input();
    }
    if !('1'..='8').contains(&number) {
        println!("Second Character have to be a number between 1 and 8");
        return read_input();
    }

    (letter, number)
}

fn position_char_to_usize(position: (char, char)) -> (usize, usize) {
    let (letter, number) = position;
    let col = letter as usize - 'a' as usize;
    let row = number as usize - '1' as usize;
    (col, row)
}

fn position_usize_to_char(position: (usize, usize)) -> (char, char) {
    let (col, row) = position;
    let letter = (col as u8 + b'a') as char;
    let number = (row as u8 + b'1') as char;
    (letter, number)
}

fn print_board_pieces(board: &[[char; 8]; 8]) {
    let piece_info: (&str, &str);
    for row in 0..8 {
        for col in 0..8 {
            let piece = board[row][col];
            let (letter, number) = position_usize_to_char((col, row));
            match piece {
                '♔' => println!("White King at {}{}", letter, number),
                '♕' => println!("White Queen at {}{}", letter, number),
                '♖' => println!("White Rook at {}{}", letter, number),
                '♗' => println!("White Bishop at {}{}", letter, number),
                '♘' => println!("White Knight at {}{}", letter, number),
                '♙' => println!("White Pawn at {}{}", letter, number),
                '♚' => println!("Black King at {}{}", letter, number),
                '♛' => println!("Black Queen at {}{}", letter, number),
                '♜' => println!("Black Rook at {}{}", letter, number),
                '♝' => println!("Black Bishop at {}{}", letter, number),
                '♞' => println!("Black Knight at {}{}", letter, number),
                '♟' => println!("Black Pawn at {}{}", letter, number),
                _ => continue,
            }
        }
    }
}

fn white_pawn_possible_movements(
    position: (usize, usize),
    board: &[[char; 8]; 8],
    possible_movements: &mut [[usize; 8]; 8],
) {
    let (col, row) = position;
    if row < 7 && board[row + 1][col] == ' ' {
        possible_movements[row + 1][col] = 1;
        if row == 1 && board[row + 2][col] == ' ' {
            possible_movements[row + 2][col] = 1;
        }
    }
    if row < 7 && col > 0 {
        let target = board[row + 1][col - 1];
        if BLACK_PIECES.contains(&target) {
            possible_movements[row + 1][col - 1] = 1;
        }
    }
    if row < 7 && col < 7 {
        let target = board[row + 1][col + 1];
        if BLACK_PIECES.contains(&target) {
            possible_movements[row + 1][col + 1] = 1;
        }
    }
}

fn black_pawn_possible_movements(
    position: (usize, usize),
    board: &[[char; 8]; 8],
    possible_movements: &mut [[usize; 8]; 8],
) {
    let (col, row) = position;
    if row > 0 && board[row - 1][col] == ' ' {
        possible_movements[row - 1][col] = 1;
        if row == 6 && board[row - 2][col] == ' ' {
            possible_movements[row - 2][col] = 1;
        }
    }
    if row > 0 && col > 0 {
        let target = board[row - 1][col - 1];
        if WHITE_PIECES.contains(&target) {
            possible_movements[row - 1][col - 1] = 1;
        }
    }
    if row > 0 && col < 7 {
        let target = board[row - 1][col + 1];
        if WHITE_PIECES.contains(&target) {
            possible_movements[row - 1][col + 1] = 1;
        }
    }
}

fn rook_movement(
    position: (usize, usize),
    board: &[[char; 8]; 8],
    ally_pieces: &[char; 6],
    enemy_pieces: &[char; 6],
    possible_movements: &mut [[usize; 8]; 8],
    enemy_piece_pinned: &mut [[bool; 8]; 8],
) {
    let (col, row) = position;

    for iter_col in col + 1..=7 {
        let target = board[row][iter_col];
        if target == ' ' {
            possible_movements[row][iter_col] = 1;
        } else if ally_pieces.contains(&target) {
            break;
        } else {
            possible_movements[row][iter_col] = 1;

            //Check for pins: if next piece is Black King
            for jter_col in iter_col + 1..=7 {
                if board[row][jter_col] == enemy_pieces[0] {
                    enemy_piece_pinned[row][col..jter_col].fill(true);
                } else if board[row][jter_col] == ' ' {
                    continue;
                } else {
                    break;
                }
            }
            break;
        }
    }
    //Para los ciclos de col a 0 seria: for i in (0..col).rev() {aja}
    for iter_col in (0..col).rev() {
        let target = board[row][iter_col];
        if target == ' ' {
            possible_movements[row][iter_col] = 1;
        } else if ally_pieces.contains(&target) {
            break;
        } else {
            possible_movements[row][iter_col] = 1;
            //Check for pins: if next piece is Black King
            for jter_col in (0..iter_col).rev() {
                if board[row][jter_col] == enemy_pieces[0] {
                    enemy_piece_pinned[row][jter_col + 1..=col].fill(true);
                } else if board[row][jter_col] == ' ' {
                    continue;
                } else {
                    break;
                }
            }
            break;
        }
    }

    for iter_row in row + 1..=7 {
        let target = board[iter_row][col];
        if target == ' ' {
            possible_movements[iter_row][col] = 1;
        } else if ally_pieces.contains(&target) {
            break;
        } else {
            possible_movements[iter_row][col] = 1;
            //Check for pins: if next piece is Black King
            for jter_row in iter_row + 1..=7 {
                if board[jter_row][col] == enemy_pieces[0] {
                    for r in row..jter_row {
                        enemy_piece_pinned[r][col] = true;
                    }
                } else if board[jter_row][col] == ' ' {
                    continue;
                } else {
                    break;
                }
            }
            break;
        }
    }

    for iter_row in (0..row).rev() {
        let target = board[iter_row][col];
        if target == ' ' {
            possible_movements[iter_row][col] = 1;
        } else if ally_pieces.contains(&target) {
            break;
        } else {
            possible_movements[iter_row][col] = 1;
            //Check for pins: if next piece is Black King
            for jter_row in (0..iter_row).rev() {
                if board[jter_row][col] == enemy_pieces[0] {
                    for r in jter_row + 1..=row {
                        enemy_piece_pinned[r][col] = true;
                    }
                } else if board[jter_row][col] == ' ' {
                    continue;
                } else {
                    break;
                }
            }
            break;
        }
    }
}
fn print_posible_movements(board: [[usize; 8]; 8]) {
    // ANSI escape codes for background colors (48 = background, 38 = foreground)

    // Unicode chess pieces are black by default, white pieces need to be distinguished
    // Black pieces: ♜♞♝♛♚♟ (filled)
    // White pieces: ♖♘♗♕♔♙ (outline style in Unicode, but render differently per font)

    println!("   a  b  c  d  e  f  g  h");

    for row in (0..8).rev() {
        print!("{} ", row + 1);

        for col in 0..8 {
            // Determine square color: (row + col) % 2 == 0 is traditionally a light square
            let is_white_square = (row + col) % 2 == 0;
            let bg = if board[row][col] == 1 {
                GREEN
            } else if is_white_square {
                WHITE_BG
            } else {
                BLACK_BG
            };

            // Print: background color + piece color + piece + reset (but keep bg for space)
            // Actually, we need to reset after the space to avoid bleeding
            print!("{}   {}", bg, RESET);
        }

        println!(" {}", row + 1); // New line after each rank
    }
    println!("   a  b  c  d  e  f  g  h");
}
fn print_pings(board: [[bool; 8]; 8]) {
    // ANSI escape codes for background colors (48 = background, 38 = foreground)

    // Unicode chess pieces are black by default, white pieces need to be distinguished
    // Black pieces: ♜♞♝♛♚♟ (filled)
    // White pieces: ♖♘♗♕♔♙ (outline style in Unicode, but render differently per font)

    println!("   a  b  c  d  e  f  g  h");

    for row in (0..8).rev() {
        print!("{} ", row + 1);

        for col in 0..8 {
            // Determine square color: (row + col) % 2 == 0 is traditionally a light square
            let is_white_square = (row + col) % 2 == 0;
            let bg = if board[row][col] == true {
                GREEN
            } else if is_white_square {
                WHITE_BG
            } else {
                BLACK_BG
            };

            // Print: background color + piece color + piece + reset (but keep bg for space)
            // Actually, we need to reset after the space to avoid bleeding
            print!("{}   {}", bg, RESET);
        }

        println!(" {}", row + 1); // New line after each rank
    }
    println!("   a  b  c  d  e  f  g  h");
}
