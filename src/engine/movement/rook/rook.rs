use crate::engine::{
    game::analyzer::analyzer::PlayingAs,
    movement::movement::{
        FULL_u64, Movement, COLUMN_H, EMPTY_U64, ROOK_FULL_COLUM_MOVEMENT_DEFINITION, ROW_1,
        SINGLE_BYTE_U8,
    },
};

pub struct Rook {}
//rook movement

impl Rook {
    pub fn get_moves(
        rook_bits: u64,
        color: PlayingAs,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> u64 {
        let piece_index = Movement::get_piece_index(rook_bits);
        let column = piece_index % 8;
        let row = piece_index / 8;

        let mut move_bits: u64 = EMPTY_U64;

        let enemy_blockers = Movement::enemy_blockers(&color, white_bitboard, black_bitboard);
        let ally_bloquers =
            Movement::ally_blockers(&color, white_bitboard, black_bitboard) & !rook_bits;
        //UP MOVES

        {
            let mut up_colum = ROOK_FULL_COLUM_MOVEMENT_DEFINITION << piece_index;

            let enemy_up_colum = up_colum & enemy_blockers;

            if enemy_up_colum != 0 {
                let lsb = Movement::lsb_pos(enemy_up_colum);
                let mask = FULL_u64 >> 63 - (lsb);
                up_colum = mask & up_colum;
            }

            let ally_up_colum = up_colum & ally_bloquers;
            if ally_up_colum != 0 {
                let lsb = Movement::lsb_pos(ally_up_colum);
                let mask = FULL_u64 >> 63 - (lsb - 1);
                up_colum = mask & up_colum;
            }
            move_bits |= up_colum;
        }

        //DOWN MOVES
        if row > ROW_1 {
            let mut down_colum = ROOK_FULL_COLUM_MOVEMENT_DEFINITION >> 63 - piece_index + 1;

            let enemy_down_colum = down_colum & enemy_blockers;

            if enemy_down_colum != 0 {
                let msb = Movement::msb_pos(enemy_down_colum);
                let mask = FULL_u64 << msb;
                down_colum = mask & down_colum;
            }

            let ally_down_colum = down_colum & ally_bloquers;
            if ally_down_colum != 0 {
                let msb = Movement::msb_pos(ally_down_colum);
                let mask = FULL_u64 << msb + 1;
                down_colum = mask & down_colum;
            }
            move_bits |= down_colum;
        }

        //LEFT MOVES
        {
            let mut left_row = ((SINGLE_BYTE_U8 << column) as u64) << row * 8;

            let enemy_left_row = left_row & enemy_blockers;
            if enemy_left_row != 0 {
                let lsb = Movement::lsb_pos(enemy_left_row);
                let mask = FULL_u64 >> 63 - (lsb);
                left_row = mask & left_row;
            }

            let ally_down_colum = left_row & ally_bloquers;
            if ally_down_colum != 0 {
                let lsb = Movement::lsb_pos(ally_down_colum);
                let mask = FULL_u64 >> 63 - (lsb - 1);
                left_row = mask & left_row;
            }
            move_bits |= left_row;
        }

        //RIGHT MOVES
        if column > COLUMN_H {
            let mut right_row = ((SINGLE_BYTE_U8 >> 8 - column) as u64) << row * 8;

            let enemy_left_row = right_row & enemy_blockers;
            if enemy_left_row != 0 {
                let msb = Movement::msb_pos(enemy_left_row);
                let mask = FULL_u64 << msb;
                right_row = mask & right_row;
            }

            let ally_down_colum = right_row & ally_bloquers;
            if ally_down_colum != 0 {
                let msb = Movement::msb_pos(ally_down_colum);
                let mask = FULL_u64 << msb + 1;
                right_row = mask & right_row;
            }

            move_bits |= right_row;
        }
        return move_bits & !rook_bits;
    }
}
