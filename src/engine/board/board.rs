use crate::engine::{
    game::analyzer::analyzer::PlayingAs,
    movement::movement::{Movement, NOT_FOUND},
    printer::printer::{
        Printer, BLACK_BISHOP_CHAR, BLACK_KING_CHAR, BLACK_KNIGHT_CHAR, BLACK_PAWN_CHAR,
        BLACK_QUEEN_CHAR, BLACK_ROOK_CHAR, WHITE_BISHOP_CHAR, WHITE_KING_CHAR, WHITE_KNIGHT_CHAR,
        WHITE_PAWN_CHAR, WHITE_QUEEN_CHAR, WHITE_ROOK_CHAR,
    },
};

use super::position::position::Move;

#[derive(Clone, Copy)]
pub enum PieceType {
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,

    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
}

const C_B_ROOKS: u64 = 0b1000000100000000000000000000100000000000000000000000000000000000;
const C_B_KNIGHTS: u64 = 0b0100001000000000000000000000000000000000000000000000000000000000;
const C_B_BISHOPS: u64 = 0b0010010000000000000000000000000000000000000000000000000000000000;
const C_B_QUEEN: u64 = 0b0001000000000000000000000000000000000000000000000000000000000000;
const C_B_KING: u64 = 0b0000100000000000000000000000000000000000000000000000000000000000;
const C_B_PAWNS: u64 = 0b0000000011111111000000000000000000000000000000000000000000000000;

const C_W_ROOKS: u64 = 0b0000000000000000000000000000000000000000000000000000000010000001;
const C_W_KNIGHTS: u64 = 0b0000000000000000000000000000000000000000000000000000000001000010;
const C_W_BISHOPS: u64 = 0b0000000000000000000000000000000000000000000000000000000000100100;
const C_W_QUEEN: u64 = 0b0000000000000000000000000000000000000000000000000000000000010000;
const C_W_KING: u64 = 0b0000000000000000000000000000000000000000000000000000000000001000;
const C_W_PAWNS: u64 = 0b0000000000000000000000000000000000000000000000001111111100000000;

#[derive(Copy, Clone)]
pub struct Board {
    pub w_rooks: u64,
    pub w_knights: u64,
    pub w_bishops: u64,
    pub w_queen: u64,
    pub w_king: u64,
    pub w_pawns: u64,

    pub b_rooks: u64,
    pub b_knights: u64,
    pub b_bishops: u64,
    pub b_queen: u64,
    pub b_king: u64,
    pub b_pawns: u64,
}

impl Board {
    pub fn new() -> Board {
        return Board {
            w_rooks: C_W_ROOKS,
            w_knights: C_W_KNIGHTS,
            w_bishops: C_W_BISHOPS,
            w_queen: C_W_QUEEN,
            w_king: C_W_KING,
            w_pawns: C_W_PAWNS,

            b_rooks: C_B_ROOKS,
            b_knights: C_B_KNIGHTS,
            b_bishops: C_B_BISHOPS,
            b_queen: C_B_QUEEN,
            b_king: C_B_KING,
            b_pawns: C_B_PAWNS,
        };
    }

    pub fn new_empty() -> Board {
        return Board {
            w_rooks: 0,
            w_knights: 0,
            w_bishops: 0,
            w_queen: 0,
            w_king: 0,
            w_pawns: 0,

            b_rooks: 0,
            b_knights: 0,
            b_bishops: 0,
            b_queen: 0,
            b_king: 0,
            b_pawns: 0,
        };
    }

    pub fn new_from_values(
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
    ) -> Board {
        return Board {
            w_rooks: w_rook_board,
            w_knights: w_knigth_board,
            w_bishops: w_bishop_board,
            w_queen: w_queen_board,
            w_king: w_king_board,
            w_pawns: w_pawn_board,

            b_rooks: b_rook_board,
            b_knights: b_knigth_board,
            b_bishops: b_bishop_board,
            b_queen: b_queen_board,
            b_king: b_king_board,
            b_pawns: b_pawn_board,
        };
    }

    pub fn getOcupancy(&self) -> u64 {
        return (self.w_rooks
            | self.w_knights
            | self.w_bishops
            | self.w_queen
            | self.w_king
            | self.w_pawns
            | self.b_rooks
            | self.b_knights
            | self.b_bishops
            | self.b_queen
            | self.b_king
            | self.b_pawns);
    }

    pub fn getNotOcupancy(&self) -> u64 {
        return !self.getOcupancy();
    }
    pub fn getWhiteBitboard(&self) -> u64 {
        return (self.w_rooks
            | self.w_knights
            | self.w_bishops
            | self.w_queen
            | self.w_king
            | self.w_pawns);
    }

    pub fn getBlackBitboard(&self) -> u64 {
        return (self.b_rooks
            | self.b_knights
            | self.b_bishops
            | self.b_queen
            | self.b_king
            | self.b_pawns);
    }

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
        let mut board__arr = board_array;
        let mut board_aux = piece_board;
        for _ in 0..64 {
            let pos = Movement::lsb_pos(board_aux);
            if (pos != NOT_FOUND) {
                board__arr[pos as usize] = Self::get_piece_str_for_piece_type(piece);
                board_aux = if pos == 63 {
                    return board__arr;
                } else {
                    board_aux >> pos + 1 << pos + 1
                };
            }
        }
        return board__arr;
    }

    fn try_move(&mut self, piece_board: u64, destin_board: u64, piece_type: PieceType) -> bool {
        let mut board_copy = self.to_owned();

        match piece_type {
            PieceType::WhiteQueen => {
                board_copy.w_queen = board_copy.w_queen & !piece_board;
                board_copy.w_queen = board_copy.w_queen | destin_board;
                if Movement::check_for_check(&PlayingAs::White, board_copy) {
                    return false;
                }
                self.w_queen = self.w_queen & !piece_board;
                self.w_queen = self.w_queen | destin_board;
                return true;
            }
            PieceType::WhiteKing => {
                board_copy.w_king = board_copy.w_king & !piece_board;
                board_copy.w_king = board_copy.w_king | destin_board;
                if Movement::check_for_check(&PlayingAs::White, board_copy) {
                    return false;
                }
                self.w_king = self.w_king & !piece_board;
                self.w_king = self.w_king | destin_board;
                return true;
            }
            PieceType::WhiteBishop => {
                board_copy.w_bishops = board_copy.w_bishops & !piece_board;
                board_copy.w_bishops = board_copy.w_bishops | destin_board;
                if Movement::check_for_check(&PlayingAs::White, board_copy) {
                    return false;
                }
                self.w_bishops = self.w_bishops & !piece_board;
                self.w_bishops = self.w_bishops | destin_board;
                return true;
            }
            PieceType::WhiteRook => {
                board_copy.w_rooks = board_copy.w_rooks & !piece_board;
                board_copy.w_rooks = board_copy.w_rooks | destin_board;
                if Movement::check_for_check(&PlayingAs::White, board_copy) {
                    return false;
                }
                self.w_rooks = self.w_rooks & !piece_board;
                self.w_rooks = self.w_rooks | destin_board;
                return true;
            }
            PieceType::WhiteKnight => {
                board_copy.w_knights = board_copy.w_knights & !piece_board;
                board_copy.w_knights = board_copy.w_knights | destin_board;
                if Movement::check_for_check(&PlayingAs::White, board_copy) {
                    return false;
                }
                self.w_knights = self.w_knights & !piece_board;
                self.w_knights = self.w_knights | destin_board;
                return true;
            }
            PieceType::WhitePawn => {
                board_copy.w_pawns = board_copy.w_pawns & !piece_board;
                board_copy.w_pawns = board_copy.w_pawns | destin_board;
                if Movement::check_for_check(&PlayingAs::White, board_copy) {
                    return false;
                }
                self.w_pawns = self.w_pawns & !piece_board;
                self.w_pawns = self.w_pawns | destin_board;
                return true;
            }

            PieceType::BlackQueen => {
                board_copy.b_queen = board_copy.b_queen & !piece_board;
                board_copy.b_queen = board_copy.b_queen | destin_board;

                if Movement::check_for_check(&PlayingAs::Black, board_copy) {
                    return false;
                }

                self.b_queen = self.b_queen & !piece_board;
                self.b_queen = self.b_queen | destin_board;
                return true;
            }
            PieceType::BlackKing => {
                board_copy.b_king = board_copy.b_king & !piece_board;
                board_copy.b_king = board_copy.b_king | destin_board;

                if Movement::check_for_check(&PlayingAs::Black, board_copy) {
                    return false;
                }

                self.b_king = self.b_king & !piece_board;
                self.b_king = self.b_king | destin_board;
                return true;
            }
            PieceType::BlackBishop => {
                board_copy.b_bishops = board_copy.b_bishops & !piece_board;
                board_copy.b_bishops = board_copy.b_bishops | destin_board;

                if Movement::check_for_check(&PlayingAs::Black, board_copy) {
                    return false;
                }

                self.b_bishops = self.b_bishops & !piece_board;
                self.b_bishops = self.b_bishops | destin_board;
                return true;
            }
            PieceType::BlackRook => {
                board_copy.b_rooks = board_copy.b_rooks & !piece_board;
                board_copy.b_rooks = board_copy.b_rooks | destin_board;

                if Movement::check_for_check(&PlayingAs::Black, board_copy) {
                    return false;
                }

                self.b_rooks = self.b_rooks & !piece_board;
                self.b_rooks = self.b_rooks | destin_board;
                return true;
            }
            PieceType::BlackKnight => {
                board_copy.b_knights = board_copy.b_knights & !piece_board;
                board_copy.b_knights = board_copy.b_knights | destin_board;

                if Movement::check_for_check(&PlayingAs::Black, board_copy) {
                    return false;
                }

                self.b_knights = self.b_knights & !piece_board;
                self.b_knights = self.b_knights | destin_board;
                return true;
            }
            PieceType::BlackPawn => {
                board_copy.b_pawns = board_copy.b_pawns & !piece_board;
                board_copy.b_pawns = board_copy.b_pawns | destin_board;

                if Movement::check_for_check(&PlayingAs::Black, board_copy) {
                    return false;
                }

                self.b_pawns = self.b_pawns & !piece_board;
                self.b_pawns = self.b_pawns | destin_board;
                return true;
            }
        }
    }

    pub fn perform_move(&mut self, movve: Move, playing_as: PlayingAs) -> bool {
        let mut piece_bitboard: u64 = (1 as u64) << movve.from.colum;
        if movve.from.row != 0 {
            piece_bitboard = piece_bitboard << movve.from.row * 8
        }

        let mut destin_bitboard: u64 = (1 as u64) << movve.to.colum;
        if movve.to.row != 0 {
            destin_bitboard = destin_bitboard << movve.to.row * 8
        }

        let white_bitboard = self.getWhiteBitboard();
        let black_bitboard = self.getBlackBitboard();

        match playing_as {
            PlayingAs::Black => {
                //Piece is Rook
                if self.b_rooks & piece_bitboard > 0
                    && Movement::get_rook_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::BlackRook);
                }

                //Piece is Knight
                if self.b_knights & piece_bitboard > 0
                    && Movement::get_knigth_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::BlackKnight);
                }

                //Piece is Bishop
                if self.b_bishops & piece_bitboard > 0
                    && Movement::get_bishop_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::BlackBishop);
                }

                //Piece is King
                if self.b_king & piece_bitboard > 0
                    && Movement::get_king_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::BlackKing);
                }
                //Piece is Queen
                if self.b_queen & piece_bitboard > 0
                    && Movement::get_queen_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::BlackQueen);
                }
                //Piece is Pawn
                if self.b_pawns & piece_bitboard > 0
                    && Movement::get_pawn_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::BlackPawn);
                }
            }
            PlayingAs::White => {
                //Piece is Rook
                if self.w_rooks & piece_bitboard > 0
                    && Movement::get_rook_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::WhiteRook);
                }

                //Piece is Knight
                if self.w_knights & piece_bitboard > 0
                    && Movement::get_knigth_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::WhiteKnight);
                }

                //Piece is Bishop
                if self.w_bishops & piece_bitboard > 0
                    && Movement::get_bishop_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::WhiteBishop);
                }

                //Piece is King
                if self.w_king & piece_bitboard > 0
                    && Movement::get_king_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::WhiteKing);
                }
                //Piece is Queen
                if self.w_queen & piece_bitboard > 0
                    && Movement::get_queen_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::WhiteQueen);
                }
                //Piece is Pawn
                if self.w_pawns & piece_bitboard > 0
                    && Movement::get_pawn_moves(
                        piece_bitboard,
                        playing_as,
                        white_bitboard,
                        black_bitboard,
                    ) & destin_bitboard
                        > 0
                {
                    return self.try_move(piece_bitboard, destin_bitboard, PieceType::WhitePawn);
                }
            }
        }
        return false;
    }

    pub fn print_board(
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
        let mut board__arr: [&str; 64] = [""; 64];
        //Black pawns
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::BlackPawn, b_pawn_board, board__arr);
        board__arr = Board::get_print_param_array_for_piece(
            PieceType::BlackKnight,
            b_knigth_board,
            board__arr,
        );
        board__arr = Board::get_print_param_array_for_piece(
            PieceType::BlackBishop,
            b_bishop_board,
            board__arr,
        );
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::BlackRook, b_rook_board, board__arr);
        board__arr = Board::get_print_param_array_for_piece(
            PieceType::BlackQueen,
            b_queen_board,
            board__arr,
        );
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::BlackKing, b_king_board, board__arr);
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::WhitePawn, w_pawn_board, board__arr);
        board__arr = Board::get_print_param_array_for_piece(
            PieceType::WhiteKnight,
            w_knigth_board,
            board__arr,
        );
        board__arr = Board::get_print_param_array_for_piece(
            PieceType::WhiteBishop,
            w_bishop_board,
            board__arr,
        );
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::WhiteRook, w_rook_board, board__arr);
        board__arr = Board::get_print_param_array_for_piece(
            PieceType::WhiteQueen,
            w_queen_board,
            board__arr,
        );
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::WhiteKing, w_king_board, board__arr);

        board__arr.reverse();
        Printer::print_board(board_name, board__arr);
    }

    pub fn print_board_moves(piece: PieceType, movement_bits: u64) {
        let mut board__arr: [&str; 64] = [""; 64];

        board__arr = Board::get_print_param_array_for_piece(piece, movement_bits, board__arr);
        board__arr.reverse();

        Printer::print_board("Moves", board__arr);
    }

    pub fn print_board_moves_with_text(message: &str, piece: PieceType, movement_bits: u64) {
        let mut board__arr: [&str; 64] = [""; 64];

        board__arr = Board::get_print_param_array_for_piece(piece, movement_bits, board__arr);
        board__arr.reverse();

        Printer::print_board(message, board__arr);
    }

    pub fn print_board_self(&self, board_name: &str) {
        let mut board__arr: [&str; 64] = [""; 64];
        //Black pawns
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::BlackPawn, self.b_pawns, board__arr);
        board__arr = Board::get_print_param_array_for_piece(
            PieceType::BlackKnight,
            self.b_knights,
            board__arr,
        );
        board__arr = Board::get_print_param_array_for_piece(
            PieceType::BlackBishop,
            self.b_bishops,
            board__arr,
        );
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::BlackRook, self.b_rooks, board__arr);
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::BlackQueen, self.b_queen, board__arr);
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::BlackKing, self.b_king, board__arr);

        board__arr =
            Board::get_print_param_array_for_piece(PieceType::WhitePawn, self.w_pawns, board__arr);
        board__arr = Board::get_print_param_array_for_piece(
            PieceType::WhiteKnight,
            self.w_knights,
            board__arr,
        );
        board__arr = Board::get_print_param_array_for_piece(
            PieceType::WhiteBishop,
            self.w_bishops,
            board__arr,
        );
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::WhiteRook, self.w_rooks, board__arr);
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::WhiteQueen, self.w_queen, board__arr);
        board__arr =
            Board::get_print_param_array_for_piece(PieceType::WhiteKing, self.w_king, board__arr);
        board__arr.reverse();

        Printer::print_board(board_name, board__arr);
    }
}
