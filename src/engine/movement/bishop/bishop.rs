use crate::engine::{
    board::board::{Board, Turn},
    movement::movement::{FULL_u64, Movement, EMPTY_U64},
};

pub struct Bishop {}
//rook movement

impl Bishop {
    pub fn get_moves(bishop_bits: u64, color: Turn, board: Board) -> u64 {
        let piece_index = Movement::get_piece_index(bishop_bits);
        let column = piece_index % 8;
        let row = piece_index / 8;

        let mut move_bits: u64 = EMPTY_U64;

        let white_bitboard = board.getWhiteBitboard();
        let black_bitboard = board.getBlackBitboard();

        let enemy_blockers = Movement::enemy_blockers(&color, white_bitboard, black_bitboard);
        let ally_bloquers = Movement::ally_blockers(&color, white_bitboard, black_bitboard);

        //Board::print_board_moves_with_text("Rotated", W_BISHOP_UTF, 0x8041221400142241);

        //Board::print_board_moves_with_text("Rotated", W_BISHOP_UTF, 0x8041221400142241 >> 9);

        //UP LEFT
        if piece_index != 63 {
            let mut diagonal: u64 = 0;
            let mut displacement_count = 1;
            for row_bishop in 0..8 {
                if row_bishop > row {
                    let displacement = (row_bishop * 8) + (column + displacement_count);
                    if displacement % 8 < column || displacement > 64 {
                        break;
                    }

                    diagonal |= (1 as u64) << displacement;
                    displacement_count += 1;
                }
            }
            let enemy_diagonal = diagonal & enemy_blockers;

            if enemy_diagonal != 0 {
                let mut lsb = Movement::lsb_pos(enemy_diagonal);

                if lsb == 63 {
                    lsb = 63;
                } else {
                    lsb += 1
                }
                let mask = FULL_u64 << lsb;
                diagonal = !mask & diagonal;
            }

            let ally_diagonal = diagonal & ally_bloquers;
            if ally_diagonal != 0 {
                let lsb = Movement::lsb_pos(ally_diagonal);
                let mask = FULL_u64 << lsb;
                diagonal = !mask & diagonal;
            }
            move_bits |= diagonal;
        }

        //DOWN RIGHT
        if piece_index != 0 {
            let mut diagonal: u64 = 0;
            let base_displacement = column - row;
            for row_bishop in 0..8 {
                if row_bishop < row {
                    let displacement = (row_bishop) * 8 + row_bishop + base_displacement;
                    if displacement % 8 > column || displacement < 0 {
                        continue;
                    }

                    diagonal |= (1 as u64) << displacement;
                }
            }
            let enemy_diagonal = diagonal & enemy_blockers;

            if enemy_diagonal != 0 {
                let msb = Movement::msb_pos(enemy_diagonal);
                let msb_pos = if 63 - msb + 1 > 63 { 63 } else { 63 - msb + 1 };
                let mask = FULL_u64 >> msb_pos;
                diagonal = !mask & diagonal;
            }

            let ally_diagonal = diagonal & ally_bloquers;
            if ally_diagonal != 0 {
                let msb = Movement::msb_pos(ally_diagonal);
                let mask = FULL_u64 >> 63 - msb;
                diagonal = !mask & diagonal;
            }
            move_bits |= diagonal;
        }

        //UP RIGTH
        if piece_index != 56 {
            let mut diagonal: u64 = 0;
            let mut displacement_count = 1;
            for row_bishop in 0..8 {
                if row_bishop > row {
                    let displacement = (row_bishop) * 8 + column - displacement_count;
                    if displacement % 8 > column || displacement < 0 {
                        continue;
                    }

                    diagonal |= (1 as u64) << displacement;
                    displacement_count += 1;
                }
            }
            let enemy_diagonal = diagonal & enemy_blockers;

            if enemy_diagonal != 0 {
                let lsb = Movement::lsb_pos(enemy_diagonal);
                let mask = FULL_u64 << lsb + 1;
                diagonal = !mask & diagonal;
            }

            let ally_diagonal = diagonal & ally_bloquers;
            if ally_diagonal != 0 {
                let lsb = Movement::lsb_pos(ally_diagonal);
                let mask = FULL_u64 << lsb;
                diagonal = !mask & diagonal;
            }
            move_bits |= diagonal;
        }

        //DOWN LEFT
        if piece_index != 7 {
            let mut diagonal: u64 = 0;
            for row_bishop in 0..8 {
                if row_bishop < row {
                    let displacement = (row_bishop) * 8 + column + (row - row_bishop);
                    if displacement % 8 < column || displacement > 64 {
                        continue;
                    }

                    diagonal |= (1 as u64) << displacement;
                }
            }

            let enemy_diagonal = diagonal & enemy_blockers;

            if enemy_diagonal != 0 {
                let msb = Movement::msb_pos(enemy_diagonal);
                let mask = FULL_u64 >> 63 - msb + 1;

                diagonal = !mask & diagonal;
            }

            let ally_diagonal = diagonal & ally_bloquers;
            if ally_diagonal != 0 {
                let msb = Movement::msb_pos(ally_diagonal);
                let mask = FULL_u64 >> 63 - msb;

                diagonal = !mask & diagonal;
            }

            move_bits |= diagonal;
        }

        return move_bits & !bishop_bits;
    }
}
