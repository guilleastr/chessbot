use crate::engine::analyzer::analyzer::PlayingAs;
use crate::engine::movement::movement::Movement;

pub struct Queen {}
//rook movement

impl Queen {
    pub fn get_moves(
        queen_bits: u64,
        color: PlayingAs,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        let mut move_bits: u64 =
            Movement::get_rook_moves(queen_bits, color, white_bitboard, black_bitboard)
                | Movement::get_bishop_moves(queen_bits, color, white_bitboard, black_bitboard);
        return move_bits & !queen_bits;
    }
}
