const BACKGROUND_COLOR_WHITE: &str = "\x1b[38;5;231;48;5;22m";
const BACKGROUND_COLOR_BLACK: &str = "\x1b[38;5;231;48;5;239m";
const BACKGROUND_COLOR_CLOSE: &str = "\x1b[0m";

pub const WHITE_KING_CHAR: &str = "♚";
pub const WHITE_QUEEN_CHAR: &str = "♛";
pub const WHITE_ROOK_CHAR: &str = "♜";
pub const WHITE_BISHOP_CHAR: &str = "♝";
pub const WHITE_KNIGHT_CHAR: &str = "♞";
pub const WHITE_PAWN_CHAR: &str = "♟︎";

pub const BLACK_KING_CHAR: &str = "\x1b[38;5;234m♚\x1b[0m";
pub const BLACK_QUEEN_CHAR: &str = "\x1b[38;5;234m♛\x1b[0m";
pub const BLACK_ROOK_CHAR: &str = "\x1b[38;5;234m♜\x1b[0m";
pub const BLACK_BISHOP_CHAR: &str = "\x1b[38;5;234m♝\x1b[0m";
pub const BLACK_KNIGHT_CHAR: &str = "\x1b[38;5;234m♞\x1b[0m";
pub const BLACK_PAWN_CHAR: &str = "\x1b[38;5;234m♟︎\x1b[0m";

pub struct Printer {}

impl Printer {
    pub fn print_board(board_name: &str, board: [&str; 64]) {
        println!("\n {}", board_name);
        let mut square_index = 0;
        for index in 0..64 {
            if index % 8 == 0 && index > 0 {
                println!("");
                square_index += 1;
            }

            if index % 8 == 0 {
                print!("{}", 8 - (index / 8))
            }
            if (index + square_index) % 2 == 0 {
                print!("{}", BACKGROUND_COLOR_WHITE)
            } else {
                print!("{}", BACKGROUND_COLOR_BLACK)
            }
            if board[index] == "" {
                print!(" ",);
            } else {
                print!("{}", board[index]);
            }
            print!("{}", BACKGROUND_COLOR_CLOSE);
        }
        println!("\n ABCDEFGH");
    }
}
