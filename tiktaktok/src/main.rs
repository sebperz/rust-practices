use std::io;

const ORANGE: &str = "\x1B[38;5;208m";
const BLUE: &str = "\x1B[34m";
const RESET: &str = "\x1B[0m";

fn main() {
    let mut number_of_moves = 0;
    let mut table: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

    loop {
        number_of_moves += 1;
        print!("\x1Bc");
        println!("{}X turn:{}\n", ORANGE, RESET);
        print_table(table);
        let movement = turn(table, 'X');
        table[movement as usize] = 'X';
        if check_if_winner(table) {
            print!("\x1Bc");
            print_table(table);
            println!("{}X Player WINS!{}", ORANGE, RESET);
            break;
        }
        //Full board
        if number_of_moves == 9 {
            print!("\x1Bc");
            print_table(table);
            println!("It's a draw!");
            break;
        }
        number_of_moves += 1;
        print!("\x1Bc");
        println!("{}O turn:{}\n", BLUE, RESET);
        print_table(table);
        let movement = turn(table, 'O');
        table[movement as usize] = 'O';
        if check_if_winner(table) {
            print!("\x1Bc");
            print_table(table);
            println!("{}O Player WINS!{}", BLUE, RESET);
            break;
        }
    }
}

fn print_table(array: [char; 9]) {
    let colored_cell = |c: char| match c {
        'X' => format!("{}{}{}", ORANGE, c, RESET),
        'O' => format!("{}{}{}", BLUE, c, RESET),
        _ => c.to_string(),
    };

    println!(
        "{}|{}|{}\n-----\n{}|{}|{}\n-----\n{}|{}|{}\n",
        colored_cell(array[0]),
        colored_cell(array[1]),
        colored_cell(array[2]),
        colored_cell(array[3]),
        colored_cell(array[4]),
        colored_cell(array[5]),
        colored_cell(array[6]),
        colored_cell(array[7]),
        colored_cell(array[8])
    );
}

fn read_position() -> u8 {
    println!("Type the number to select position:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: u8 = match input.trim().parse::<u8>() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid number!");
            return read_position();
        }
    };

    println!("You select: {}\n", input);
    if input > 9 || input < 1 {
        println!("Pick a number between 1 and 9");
        return read_position();
    }
    input
}

fn check_if_winner(array: [char; 9]) -> bool {
    // Rows
    if array[0] == array[1] && array[1] == array[2] {
        return true;
    }
    if array[3] == array[4] && array[4] == array[5] {
        return true;
    }
    if array[6] == array[7] && array[7] == array[8] {
        return true;
    }
    // Columns
    if array[0] == array[3] && array[3] == array[6] {
        return true;
    }
    if array[1] == array[4] && array[4] == array[7] {
        return true;
    }
    if array[2] == array[5] && array[5] == array[8] {
        return true;
    }
    // Diagonals
    if array[0] == array[4] && array[4] == array[8] {
        return true;
    }
    if array[2] == array[4] && array[4] == array[6] {
        return true;
    }
    false
}

fn turn(array: [char; 9], player: char) -> u8 {
    let player_selection = read_position() - 1;
    if array[player_selection as usize] == 'X' || array[player_selection as usize] == 'O' {
        println!("Invalid Position");
        return turn(array, player);
    }
    player_selection
}
