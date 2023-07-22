use crate::engine::{board::board::Turn, movement::movement::Movement};

pub struct Queen {}
//rook movement

impl Queen {
    pub fn get_moves(
        queen_bits: u64,
        color: Turn,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        let move_bits: u64 =
            Movement::get_rook_moves(queen_bits, color, white_bitboard, black_bitboard)
                | Movement::get_bishop_moves(queen_bits, color, white_bitboard, black_bitboard);
        return move_bits & !queen_bits;
    }
}
