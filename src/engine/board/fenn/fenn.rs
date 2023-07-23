use crate::engine::board::board::{Board, Turn};

const COLUMNS: [&'static str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
pub struct FEN {}

impl FEN {
    pub fn get_board_from_fenn_str(fen_str: &'static str) -> Board {
        let fenn_split = fen_str.split(" ").collect::<Vec<&str>>();
        let board_str = fenn_split[0];
        let player = fenn_split[1];
        let castles = fenn_split[2];
        let en_passasnt_square = fenn_split[3];
        let half_move_clock = fenn_split[4];
        let full_move_clock = fenn_split[5];

        //Turn
        let mut board = Board::new_empty();
        match player {
            "w" => board.set_turn(Turn::White),
            "b" => board.set_turn(Turn::Black),
            _ => {}
        }

        //Castling
        let castles_values = castles.trim().split("").collect::<Vec<&str>>();
        board.has_w_king_side_castle = true;
        board.has_w_queen_side_castle = true;
        board.has_b_king_side_castle = true;
        board.has_b_queen_side_castle = true;
        for castle_value in castles_values {
            match castle_value {
                "K" => board.has_w_king_side_castle = false,
                "Q" => board.has_w_queen_side_castle = false,
                "k" => board.has_b_king_side_castle = false,
                "q" => board.has_b_queen_side_castle = false,
                _ => {}
            }
        }

        //en passant
        let en_passant_values = en_passasnt_square.trim().split("").collect::<Vec<&str>>();
        let mut en_passat_column = -1;

        let mut en_passant_row = -1;

        'enpassant_loop: for e in en_passant_values {
            match e {
                "a" => en_passat_column = 7,
                "b" => en_passat_column = 6,
                "c" => en_passat_column = 5,
                "d" => en_passat_column = 4,
                "e" => en_passat_column = 3,
                "f" => en_passat_column = 2,
                "g" => en_passat_column = 1,
                "h" => en_passat_column = 0,
                "1" => en_passant_row = 0,
                "2" => en_passant_row = 1,
                "3" => en_passant_row = 2,
                "4" => en_passant_row = 3,
                "5" => en_passant_row = 4,
                "6" => en_passant_row = 5,
                "7" => en_passant_row = 6,
                "8" => en_passant_row = 7,
                "-" => {
                    break 'enpassant_loop;
                }
                _ => {}
            }
        }

        let en_passant_pos = en_passat_column + en_passant_row * 8;

        if en_passat_column > -1 && en_passant_row > -1 {
            if en_passant_pos <= 31 {
                board.w_en_passant = 1 << en_passant_pos
            } else {
                board.b_en_passant = 1 << en_passant_pos
            }
        }

        //Moves count
        board.half_move_count = half_move_clock.parse().unwrap_or(0);
        board.full_move_count = full_move_clock.parse().unwrap_or(0);

        let mut column = 0;
        let mut row = 0;

        for rank in board_str.split("/").collect::<Vec<&str>>().iter().rev() {
            for this_rank in rank.split("").collect::<Vec<&str>>().iter().rev() {
                let shifted_column = column;

                let shifted_row = row * 8;
                let mut value_displacement = shifted_column + shifted_row;

                if value_displacement / 8 != row {
                    value_displacement -= 1;
                }

                match *this_rank {
                    "" => {}
                    "1" => column += 1,
                    "2" => column += 2,
                    "3" => column += 3,
                    "4" => column += 4,
                    "5" => column += 5,
                    "6" => column += 6,
                    "7" => column += 7,
                    "8" => {}
                    "r" => {
                        board.b_rooks |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    "n" => {
                        board.b_knights |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    "b" => {
                        board.b_bishops |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    "q" => {
                        board.b_queen |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    "k" => {
                        board.b_king |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    "p" => {
                        board.b_pawns |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    "R" => {
                        board.w_rooks |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    "N" => {
                        board.w_knights |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    "B" => {
                        board.w_bishops |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    "Q" => {
                        board.w_queen |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    "K" => {
                        board.w_king |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    "P" => {
                        board.w_pawns |= (1 as u64) << value_displacement;
                        column += 1;
                    }
                    _ => {}
                }
            }
            column = 0;
            row += 1;
        }
        return board;
    }
}
