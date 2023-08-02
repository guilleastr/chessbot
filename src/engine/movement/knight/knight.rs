use crate::engine::{
    board::board::{Board, Turn},
    movement::movement::Movement,
};

pub struct Knight {}
//rook movement

impl Knight {
    pub fn get_moves(knight_bits: u64, color: Turn, board: Board) -> u64 {
        let white_bitboard = board.getWhiteBitboard();
        let black_bitboard = board.getBlackBitboard();

        let l1 = (knight_bits >> 1) & 0x7f7f7f7f7f7f7f7f;
        let l2 = (knight_bits >> 2) & (0x3f3f3f3f3f3f3f3f);
        let r1 = (knight_bits << 1) & (0xfefefefefefefefe);
        let r2 = (knight_bits << 2) & (0xfcfcfcfcfcfcfcfc);
        let h1 = l1 | r1;
        let h2 = l2 | r2;
        let move_bits = (h1 << 16) | (h1 >> 16) | (h2 << 8) | (h2 >> 8);

        let ally_bloquers = Movement::ally_blockers(&color, white_bitboard, black_bitboard);

        return move_bits & !ally_bloquers;
    }
}
