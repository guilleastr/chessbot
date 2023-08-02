use crate::engine::board::{
    self,
    board::{Board, Turn},
};

pub struct Queen {}
//rook movement

impl Queen {
    pub fn get_moves(queen_bits: u64, color: Turn, board: Board) -> u64 {
        let move_bits: u64 = board.pub_get_rook_moves_bitboard(queen_bits, color)
            | board.pub_get_bishop_moves_bitboard(queen_bits, color);
        return move_bits & !queen_bits;
    }
}
