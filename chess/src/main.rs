use std::io;

fn main() {
    let mut white_movements: String;
    let mut black_movements: String;
    let board: [[char; 8]; 8] = [
        ['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'],
        ['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ['♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙'],
        ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'],
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
