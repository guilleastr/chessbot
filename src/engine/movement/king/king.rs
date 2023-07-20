use crate::engine::{
    game::analyzer::analyzer::PlayingAs,
    movement::movement::{Movement, ROW_1, ROW_8},
};

pub struct King {}
//rook movement

impl King {
    pub fn get_moves(
        king_bits: u64,
        color: PlayingAs,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        let piece_index = Movement::get_piece_index(king_bits);
        let column = piece_index % 8;
        let row = piece_index / 8;

        let base_row: u64 = 0x7;
        let mut down_moves: u64 = 0;
        let mut up_moves: u64 = 0;

        let mut shift_base_row_amount = column - 1;
        if shift_base_row_amount < 0 {
            shift_base_row_amount = 0;
        }
        if row > ROW_1 {
            down_moves = (base_row << shift_base_row_amount) << (row - 1) * 8;
        }

        if row > ROW_8 {
            up_moves = (base_row << shift_base_row_amount) << (row + 1) * 8;
        }
        let mut same_row_moves = base_row << column - 1;
        if row > 0 {
            same_row_moves = same_row_moves << row * 8;
        }
        let move_bits = down_moves | up_moves | same_row_moves;

        let ally_bloquers = Movement::ally_blockers(&color, white_bitboard, black_bitboard);

        return move_bits & !ally_bloquers & !king_bits;
    }
}
