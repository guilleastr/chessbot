use crate::engine::{
    board::board::{Board, Turn},
    movement::movement::{Movement, ROW_2, ROW_4, ROW_5, ROW_7, SINGLE_BYTE},
};

pub struct Pawn {}
//rook movement

impl Pawn {
    pub fn get_moves(pawn_bits: u64, color: Turn, board: Board) -> u64 {
        let white_bitboard = board.getWhiteBitboard();
        let black_bitboard = board.getBlackBitboard();

        let piece_index = Movement::get_piece_index(pawn_bits);
        let row: i8 = piece_index / 8;
        let column: i8 = piece_index % 8;

        let mut move_bits = 0;
        if matches!(color, Turn::Black) {
            let blocking_piece = pawn_bits >> 8 & (white_bitboard | black_bitboard);
            if blocking_piece == 0 {
                move_bits |= pawn_bits >> 8;
            }

            if row == ROW_7 {
                let start_blocking_piece =
                    (pawn_bits >> 8 | pawn_bits >> 16) & (white_bitboard | black_bitboard);
                if start_blocking_piece == 0 {
                    move_bits |= pawn_bits >> 16;
                }
            }
            move_bits |=
                (pawn_bits >> 9 | pawn_bits >> 7) & (white_bitboard & SINGLE_BYTE << (row - 1) * 8);
        }

        if matches!(color, Turn::White) {
            let blocking_piece = pawn_bits << 8 & (white_bitboard | black_bitboard);
            if blocking_piece == 0 {
                move_bits |= pawn_bits << 8;
            }

            if row == ROW_2 {
                let start_blocking_piece =
                    (pawn_bits << 8 | pawn_bits << 16) & (white_bitboard | black_bitboard);
                if start_blocking_piece == 0 {
                    move_bits |= pawn_bits << 16;
                }
            }
            move_bits |=
                (pawn_bits << 9 | pawn_bits << 7) & (black_bitboard & SINGLE_BYTE << (row + 1) * 8);
        }

        return move_bits;
    }

    pub fn get_moves_enpassant(pawn_bits: u64, unpassant_bits: u64, color: Turn) -> u64 {
        let piece_index = Movement::get_piece_index(pawn_bits);
        let row: i8 = piece_index / 8;
        let pawn_mask = pawn_bits << 1 | pawn_bits >> 1;

        let mut move_bits = 0;
        if matches!(color, Turn::Black) {
            if row != ROW_4 {
                return move_bits;
            }
            if pawn_mask & unpassant_bits != 0 {
                move_bits |= unpassant_bits >> 8;
            }
        }

        if matches!(color, Turn::White) {
            if row != ROW_5 {
                return move_bits;
            }
            if pawn_mask & unpassant_bits != 0 {
                move_bits |= unpassant_bits << 8;
            }
        }
        return move_bits;
    }
}
