use crate::engine::{
    board::board::{Board, PieceType},
    movement::movement::{Movement, NOT_FOUND},
};

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
    fn get_piece_str_for_piece_type(piece_type: PieceType) -> &'static str {
        match piece_type {
            PieceType::WhiteQueen => WHITE_QUEEN_CHAR,
            PieceType::WhiteKing => WHITE_KING_CHAR,
            PieceType::WhiteBishop => WHITE_BISHOP_CHAR,
            PieceType::WhiteRook => WHITE_ROOK_CHAR,
            PieceType::WhiteKnight => WHITE_KNIGHT_CHAR,
            PieceType::WhitePawn => WHITE_PAWN_CHAR,

            PieceType::BlackQueen => BLACK_QUEEN_CHAR,
            PieceType::BlackKing => BLACK_KING_CHAR,
            PieceType::BlackBishop => BLACK_BISHOP_CHAR,
            PieceType::BlackRook => BLACK_ROOK_CHAR,
            PieceType::BlackKnight => BLACK_KNIGHT_CHAR,
            PieceType::BlackPawn => BLACK_PAWN_CHAR,
        }
    }
    pub(self) fn get_print_param_array_for_piece(
        piece: PieceType,
        piece_board: u64,
        board_array: [&'static str; 64],
    ) -> [&'static str; 64] {
        let mut board_arr = board_array;
        let mut board_aux = piece_board;
        for _ in 0..64 {
            let pos = Movement::lsb_pos(board_aux);
            if pos != NOT_FOUND {
                board_arr[pos as usize] = Self::get_piece_str_for_piece_type(piece);
                board_aux = if pos == 63 {
                    return board_arr;
                } else {
                    board_aux >> pos + 1 << pos + 1
                };
            }
        }
        return board_arr;
    }

    pub fn print_bitboards(
        board_name: &str,
        b_pawn_board: u64,
        b_knigth_board: u64,
        b_bishop_board: u64,
        b_rook_board: u64,
        b_queen_board: u64,
        b_king_board: u64,

        w_pawn_board: u64,
        w_knigth_board: u64,
        w_bishop_board: u64,
        w_rook_board: u64,
        w_queen_board: u64,
        w_king_board: u64,
    ) {
        let mut board_arr: [&str; 64] = [""; 64];
        //Black pawns
        board_arr =
            Printer::get_print_param_array_for_piece(PieceType::BlackPawn, b_pawn_board, board_arr);
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::BlackKnight,
            b_knigth_board,
            board_arr,
        );
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::BlackBishop,
            b_bishop_board,
            board_arr,
        );
        board_arr =
            Printer::get_print_param_array_for_piece(PieceType::BlackRook, b_rook_board, board_arr);
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::BlackQueen,
            b_queen_board,
            board_arr,
        );
        board_arr =
            Printer::get_print_param_array_for_piece(PieceType::BlackKing, b_king_board, board_arr);
        board_arr =
            Printer::get_print_param_array_for_piece(PieceType::WhitePawn, w_pawn_board, board_arr);
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::WhiteKnight,
            w_knigth_board,
            board_arr,
        );
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::WhiteBishop,
            w_bishop_board,
            board_arr,
        );
        board_arr =
            Printer::get_print_param_array_for_piece(PieceType::WhiteRook, w_rook_board, board_arr);
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::WhiteQueen,
            w_queen_board,
            board_arr,
        );
        board_arr =
            Printer::get_print_param_array_for_piece(PieceType::WhiteKing, w_king_board, board_arr);

        board_arr.reverse();

        Printer::print(board_name, board_arr);
    }

    pub fn print_board(board_name: &str, board: Board) {
        let mut board_arr: [&str; 64] = [""; 64];
        //Black pawns
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::BlackPawn,
            board.b_pawns,
            board_arr,
        );
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::BlackKnight,
            board.b_knights,
            board_arr,
        );
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::BlackBishop,
            board.b_bishops,
            board_arr,
        );
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::BlackRook,
            board.b_rooks,
            board_arr,
        );
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::BlackQueen,
            board.b_queen,
            board_arr,
        );
        board_arr =
            Printer::get_print_param_array_for_piece(PieceType::BlackKing, board.b_king, board_arr);

        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::WhitePawn,
            board.w_pawns,
            board_arr,
        );
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::WhiteKnight,
            board.w_knights,
            board_arr,
        );
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::WhiteBishop,
            board.w_bishops,
            board_arr,
        );
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::WhiteRook,
            board.w_rooks,
            board_arr,
        );
        board_arr = Printer::get_print_param_array_for_piece(
            PieceType::WhiteQueen,
            board.w_queen,
            board_arr,
        );
        board_arr =
            Printer::get_print_param_array_for_piece(PieceType::WhiteKing, board.w_king, board_arr);
        board_arr.reverse();

        Printer::print(board_name, board_arr);
    }

    fn print(board_name: &str, board: [&'static str; 64]) {
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
