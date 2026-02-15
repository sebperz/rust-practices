fn main() {
    const LIGHT_SQUARE: &str = "\x1b[48;5;180m";
    const DARK_SQUARE: &str = "\x1b[48;5;94m";
    const RESET: &str = "\x1b[0m";
    const WHITE_PIECE: &str = "\x1b[38;5;255m";
    const BLACK_PIECE: &str = "\x1b[38;5;0m";
    const SQUARE_WIDTH: usize = 4;

    let board: [[Option<char>; 8]; 8] = [
        ['♖', '♘', '♗', '♕', '♔', '♗', '♘', '♖'],
        ['♙', '♙', '♙', '♙', '♙', '♙', '♙', '♙'],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        ['♟', '♟', '♟', '♟', '♟', '♟', '♟', '♟'],
        ['♜', '♞', '♝', '♛', '♚', '♝', '♞', '♜'],
    ]
    .map(|row| row.map(|c| if c == ' ' { None } else { Some(c) }));

    print!("   ");
    for c in 'a'..='h' {
        print!("  {} ", c);
    }
    println!();

    for row in (0..8).rev() {
        print!(" {} ", row + 1);

        for col in 0..8 {
            let is_light = (row + col) % 2 == 0;
            let bg = if is_light { LIGHT_SQUARE } else { DARK_SQUARE };
            let piece = board[row][col];
            let piece_fg = if piece.map(|p| p as u32).unwrap_or(0) > 0x265A {
                WHITE_PIECE
            } else {
                BLACK_PIECE
            };
            let content = piece.unwrap_or(' ');
            print!(
                "{}{}{:^width$}{}",
                bg,
                piece_fg,
                content,
                RESET,
                width = SQUARE_WIDTH
            );
        }
        println!(" {} ", row + 1);

        print!("   ");
        for col in 0..8 {
            let is_light = (row + col) % 2 == 0;
            let bg = if is_light { LIGHT_SQUARE } else { DARK_SQUARE };
            let piece = board[row][col];
            let piece_fg = if piece.map(|p| p as u32).unwrap_or(0) > 0x265A {
                WHITE_PIECE
            } else {
                BLACK_PIECE
            };
            let content = piece.unwrap_or(' ');
            print!(
                "{}{}{:^width$}{}",
                bg,
                piece_fg,
                content,
                RESET,
                width = SQUARE_WIDTH
            );
        }
        println!();
    }

    print!("   ");
    for c in 'a'..='h' {
        print!("  {} ", c);
    }
    println!();
}
