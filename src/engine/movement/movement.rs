use crate::engine::{
    board::board::{Board, PieceType},
    game::analyzer::analyzer::PlayingAs,
};

use super::{
    bishop::bishop::Bishop, king::king::King, knight::knight::Knight, pawn::pawn::Pawn,
    queen::queen::Queen, rook::rook::Rook,
};

pub struct Movement {}

pub const SINGLE_BIT: u64 = 0b00000000000000000000000000000000000000000000000000000000000000001;
pub const SINGLE_BYTE: u64 = 0b00000000000000000000000000000000000000000000000000000000011111111;
pub const FULL_u64: u64 = 0xFFFFFFFFFFFFFFFF;
pub const SINGLE_BYTE_U8: u8 = 0b11111111;
pub const EMPTY_U64: u64 = 0b0000000000000000000000000000000000000000000000000000000000000000;
pub const ROOK_FULL_COLUM_MOVEMENT_DEFINITION: u64 = 0x101010101010101;

pub const ROW_1: i8 = 0;
pub const ROW_2: i8 = 1;
pub const ROW_3: i8 = 2;
pub const ROW_4: i8 = 3;
pub const ROW_5: i8 = 4;
pub const ROW_6: i8 = 5;
pub const ROW_7: i8 = 6;
pub const ROW_8: i8 = 7;

pub const COLUMN_A: i8 = 7;
pub const COLUMN_B: i8 = 6;
pub const COLUMN_C: i8 = 5;
pub const COLUMN_D: i8 = 4;
pub const COLUMN_E: i8 = 3;
pub const COLUMN_F: i8 = 2;
pub const COLUMN_G: i8 = 1;
pub const COLUMN_H: i8 = 0;

pub const NOT_FOUND: i8 = -1;

//rook movement

impl Movement {
    pub fn new() -> Movement {
        return Movement {};
    }

    pub fn get_piece_index(piece_bits: u64) -> i8 {
        if piece_bits == 0 {
            return NOT_FOUND;
        }
        let mut i = 1;
        let mut pos = 0;

        while (i & piece_bits) == 0 {
            i = i << 1;
            pos += 1;
        }
        return pos;
    }

    pub fn pieces_count(mut pieces_bitboard: u64) -> i8 {
        if pieces_bitboard == 0 {
            return 0;
        }

        let mut count = 0;

        while pieces_bitboard != 0 {
            let piece_pos = Movement::msb_pos(pieces_bitboard);
            let piece_bits = (1 as u64) << piece_pos;
            pieces_bitboard = pieces_bitboard & !piece_bits;
            count += 1;
        }

        return count;
    }

    pub fn msb_pos(mut bits: u64) -> i8 {
        let mut cnt: i8 = 0;
        if bits == 0 {
            return NOT_FOUND;
        }
        while bits > 0 {
            cnt = cnt + 1;
            bits = bits >> 1;
        }
        return cnt - 1;
    }

    pub fn lsb_pos(mut bits: u64) -> i8 {
        let mut cnt: i8 = 0;
        if (bits == 0) {
            return NOT_FOUND;
        }
        while bits > 0 {
            cnt = cnt + 1;
            bits = bits << 1;
        }
        return 64 - cnt;
    }

    pub fn enemy_blockers(color: &PlayingAs, white_bitboard: u64, black_bitboard: u64) -> u64 {
        let mut result_move_bits: u64 = 0;
        if matches!(color, PlayingAs::Black) {
            result_move_bits = white_bitboard
        }
        if matches!(color, PlayingAs::White) {
            result_move_bits = black_bitboard;
        }

        return result_move_bits;
    }

    pub fn ally_blockers(color: &PlayingAs, white_bitboard: u64, black_bitboard: u64) -> u64 {
        let mut result_move_bits: u64 = 0;
        if matches!(color, PlayingAs::Black) {
            result_move_bits = black_bitboard;
        }
        if matches!(color, PlayingAs::White) {
            result_move_bits = white_bitboard;
        }

        return result_move_bits;
    }

    pub fn check_for_check(check_for: &PlayingAs, board: Board) -> bool {
        let pawns: u64;
        let knights: u64;
        let bishops: u64;
        let rooks: u64;
        let queen: u64;
        if matches!(check_for, PlayingAs::White) {
            pawns = Movement::extract_pieces_moves_from_bitboard(
                board.b_pawns,
                PlayingAs::Black,
                PieceType::BlackPawn,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            );
            knights = Movement::extract_pieces_moves_from_bitboard(
                board.b_knights,
                PlayingAs::Black,
                PieceType::BlackKnight,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            );
            rooks = Movement::extract_pieces_moves_from_bitboard(
                board.b_rooks,
                PlayingAs::Black,
                PieceType::BlackRook,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            );
            bishops = Movement::extract_pieces_moves_from_bitboard(
                board.b_bishops,
                PlayingAs::Black,
                PieceType::BlackBishop,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            );
            queen = Movement::extract_pieces_moves_from_bitboard(
                board.b_queen,
                PlayingAs::Black,
                PieceType::BlackQueen,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            );

            let bitboard_merge = (pawns | knights | rooks | bishops | queen) & board.w_king;

            return bitboard_merge != 0;
        }
        if matches!(check_for, PlayingAs::Black) {
            pawns = Movement::extract_pieces_moves_from_bitboard(
                board.w_pawns,
                PlayingAs::White,
                PieceType::WhitePawn,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            );
            knights = Movement::extract_pieces_moves_from_bitboard(
                board.w_knights,
                PlayingAs::White,
                PieceType::WhiteKnight,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            );
            rooks = Movement::extract_pieces_moves_from_bitboard(
                board.w_rooks,
                PlayingAs::White,
                PieceType::WhiteRook,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            );
            bishops = Movement::extract_pieces_moves_from_bitboard(
                board.w_bishops,
                PlayingAs::White,
                PieceType::WhiteBishop,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            );
            queen = Movement::extract_pieces_moves_from_bitboard(
                board.w_queen,
                PlayingAs::White,
                PieceType::WhiteQueen,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            );

            let bitboard_merge = (pawns | knights | rooks | bishops | queen) & board.b_king;

            return bitboard_merge != 0;
        }
        return false;
    }

    fn extract_piece_moves_from_bitboard(
        piece_bits: u64,
        color: PlayingAs,
        piece_type: PieceType,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        match piece_type {
            PieceType::WhiteBishop => {
                return Movement::get_bishop_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                );
            }
            PieceType::WhiteKnight => {
                return Movement::get_knigth_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            PieceType::WhiteRook => {
                return Movement::get_rook_moves(piece_bits, color, white_bitboard, black_bitboard)
            }
            PieceType::WhiteQueen => {
                return Movement::get_queen_moves(piece_bits, color, white_bitboard, black_bitboard)
            }
            PieceType::WhitePawn => {
                return Movement::get_pawn_moves(piece_bits, color, white_bitboard, black_bitboard)
            }
            PieceType::WhiteKing => {
                return Movement::get_king_moves(piece_bits, color, white_bitboard, black_bitboard)
            }
            PieceType::BlackBishop => {
                return Movement::get_bishop_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            PieceType::BlackKnight => {
                return Movement::get_knigth_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            PieceType::BlackRook => {
                return Movement::get_rook_moves(piece_bits, color, white_bitboard, black_bitboard)
            }
            PieceType::BlackQueen => {
                return Movement::get_queen_moves(piece_bits, color, white_bitboard, black_bitboard)
            }
            PieceType::BlackPawn => {
                return Movement::get_pawn_moves(piece_bits, color, white_bitboard, black_bitboard)
            }
            PieceType::BlackKing => {
                return Movement::get_king_moves(piece_bits, color, white_bitboard, black_bitboard)
            }
        }
    }

    pub fn extract_pieces_moves_from_bitboard(
        mut pieces_bits: u64,
        color: PlayingAs,
        piece_type: PieceType,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        if pieces_bits == 0 {
            return 0;
        } else {
            let mut move_bits = 0;
            while pieces_bits > 0 {
                let piece_pos = Movement::msb_pos(pieces_bits);
                let piece_bits = (1 as u64) << piece_pos;
                move_bits |= Movement::extract_piece_moves_from_bitboard(
                    piece_bits,
                    color,
                    piece_type,
                    white_bitboard,
                    black_bitboard,
                );
                pieces_bits = pieces_bits & !piece_bits;
            }

            return move_bits;
        }
    }

    pub fn get_knigth_moves(
        knight_bits: u64,
        color: PlayingAs,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        return Knight::get_moves(knight_bits, color, white_bitboard, black_bitboard);
    }

    pub fn get_pawn_moves(
        pawn_bits: u64,
        color: PlayingAs,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        return Pawn::get_moves(pawn_bits, color, white_bitboard, black_bitboard, false);
    }

    pub fn get_rook_moves(
        rook_bits: u64,
        color: PlayingAs,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        return Rook::get_moves(rook_bits, color, white_bitboard, black_bitboard);
    }

    pub fn get_bishop_moves(
        bishop_bits: u64,
        color: PlayingAs,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        return Bishop::get_moves(bishop_bits, color, white_bitboard, black_bitboard);
    }

    pub fn get_queen_moves(
        queen_bits: u64,
        color: PlayingAs,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        return Queen::get_moves(queen_bits, color, white_bitboard, black_bitboard);
    }

    pub fn get_king_moves(
        king_bits: u64,
        color: PlayingAs,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        return King::get_moves(king_bits, color, white_bitboard, black_bitboard);
    }
}