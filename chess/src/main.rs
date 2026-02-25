use std::io;

const WHITE_PIECES: [char; 6] = ['♔', '♕', '♖', '♗', '♘', '♙'];
const BLACK_PIECES: [char; 6] = ['♚', '♛', '♜', '♝', '♞', '♟'];

fn main() {
    let mut white_movements: String;
    let mut black_movements: String;
    let mut is_white_turn: bool = true;

    let mut board: [[char; 8]; 8] = [
        ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'], // a1-h1
        ['♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙'], // a2-h2
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'], // a7-h7
        ['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'], // a8-h8
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
    // loop {}
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

fn input_to_usize(position: (char, char)) -> (usize, usize) {
    let (letter, number) = position;
    let col = letter as usize - 'a' as usize;
    let row = number as usize - '1' as usize;
    (col, row)
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

fn white_bishop_movement(
    position: (usize, usize),
    board: &[[char; 8]; 8],
    possible_movements: &mut [[usize; 8]; 8],
) {
}

fn black_bishop_movement(
    position: (usize, usize),
    board: &[[char; 8]; 8],
    possible_movements: &mut [[usize; 8]; 8],
) {
    let (col, row) = position;
}

fn white_knight_movement(
    position: (usize, usize),
    board: &[[char; 8]; 8],
    possible_movements: &mut [[usize; 8]; 8],
) {
}

fn black_knight_movement(
    position: (usize, usize),
    board: &[[char; 8]; 8],
    possible_movements: &mut [[usize; 8]; 8],
) {
    let (col, row) = position;
}

fn white_rook_movement(
    position: (usize, usize),
    board: &[[char; 8]; 8],
    possible_movements: &mut [[usize; 8]; 8],
) {
    let (col, row) = position;
    // la torre esta en board row col, tengo que hace 4 for desde position hasta
    // limite del tablero o ficha
    for iter_col in col + 1..=7 {
        let target = board[row][iter_col];
        if target == ' ' {
            possible_movements[row][iter_col] = 1;
        } else if WHITE_PIECES.contains(&target) {
            break;
        } else {
            possible_movements[row][iter_col] = 1;
            // Check if next piece is Black King
            break;
        }
    }
    for iter_col in col - 1..=0 {
        let target = board[row][iter_col];
        if target == ' ' {
            possible_movements[row][iter_col] = 1;
        } else if WHITE_PIECES.contains(&target) {
            break;
        } else {
            possible_movements[row][iter_col] = 1;
            // Check if next piece is Black King
            break;
        }
    }
    for iter_row in row + 1..=7 {
        let target = board[iter_row][col];
        if target == ' ' {
            possible_movements[iter_row][col] = 1;
        } else if WHITE_PIECES.contains(&target) {
            break;
        } else {
            possible_movements[iter_row][col] = 1;
            // Check if next piece is Black King
            break;
        }
    }
    for iter_row in row + 1..=7 {
        let target = board[iter_row][col];
        if target == ' ' {
            possible_movements[iter_row][col] = 1;
        } else if WHITE_PIECES.contains(&target) {
            break;
        } else {
            possible_movements[iter_row][col] = 1;
            // Check if next piece is Black King
            break;
        }
    }
}

fn black_rook_movement(
    position: (usize, usize),
    board: &[[char; 8]; 8],
    possible_movements: &mut [[usize; 8]; 8],
) {
    let (col, row) = position;
    let directions: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for (dc, dr) in directions {
        let mut c = col as i32 + dc;
        let mut r = row as i32 + dr;

        while c >= 0 && c < 8 && r >= 0 && r < 8 {
            let target = board[r as usize][c as usize];

            if target == ' ' {
                possible_movements[r as usize][c as usize] = 1;
            } else if WHITE_PIECES.contains(&target) {
                possible_movements[r as usize][c as usize] = 1;
                break;
            } else {
                break;
            }

            c += dc;
            r += dr;
        }
    }
}

fn white_queen_movement(
    position: (usize, usize),
    board: &[[char; 8]; 8],
    possible_movements: &mut [[usize; 8]; 8],
) {
    let (col, row) = position;
    let directions: [(i32, i32); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for (dc, dr) in directions {
        let mut c = col as i32 + dc;
        let mut r = row as i32 + dr;

        while c >= 0 && c < 8 && r >= 0 && r < 8 {
            let target = board[r as usize][c as usize];

            if target == ' ' {
                possible_movements[r as usize][c as usize] = 1;
            } else if BLACK_PIECES.contains(&target) {
                possible_movements[r as usize][c as usize] = 1;
                break;
            } else {
                break;
            }

            c += dc;
            r += dr;
        }
    }
}

fn black_queen_movement(
    position: (usize, usize),
    board: &[[char; 8]; 8],
    possible_movements: &mut [[usize; 8]; 8],
) {
    let (col, row) = position;
    let directions: [(i32, i32); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    for (dc, dr) in directions {
        let mut c = col as i32 + dc;
        let mut r = row as i32 + dr;

        while c >= 0 && c < 8 && r >= 0 && r < 8 {
            let target = board[r as usize][c as usize];

            if target == ' ' {
                possible_movements[r as usize][c as usize] = 1;
            } else if WHITE_PIECES.contains(&target) {
                possible_movements[r as usize][c as usize] = 1;
                break;
            } else {
                break;
            }

            c += dc;
            r += dr;
        }
    }
}
