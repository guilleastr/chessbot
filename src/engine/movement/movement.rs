use crate::engine::board::{
    board::{Board, PieceType, Turn},
    position::position::{CastleOptions, LegalMove},
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

    pub fn enemy_blockers(color: &Turn, white_bitboard: u64, black_bitboard: u64) -> u64 {
        let mut result_move_bits: u64 = 0;
        if matches!(color, Turn::Black) {
            result_move_bits = white_bitboard
        }
        if matches!(color, Turn::White) {
            result_move_bits = black_bitboard;
        }

        return result_move_bits;
    }

    pub fn ally_blockers(color: &Turn, white_bitboard: u64, black_bitboard: u64) -> u64 {
        let mut result_move_bits: u64 = 0;
        if matches!(color, Turn::Black) {
            result_move_bits = black_bitboard;
        }
        if matches!(color, Turn::White) {
            result_move_bits = white_bitboard;
        }

        return result_move_bits;
    }

    pub fn get_oposite_color(color: Turn) -> Turn {
        match color {
            Turn::Black => return Turn::White,
            Turn::White => return Turn::Black,
        }
    }

    pub fn can_castle_king_side(board: Board, playing_as: Turn) -> bool {
        match playing_as {
            Turn::White => {
                let attacts = Movement::extract_all_attacks_for_color(board, Turn::Black);

                return !(board.has_w_king_side_castle || board.w_king_has_moved)
                    && board.w_king & 0x8 > 0
                    && board.w_rooks & 0x1 > 0
                    && board.getOcupancy() & 0x6 == 0
                    && attacts & 0xe == 0;
            }
            Turn::Black => {
                let attacts = Movement::extract_all_attacks_for_color(board, Turn::White);

                return !(board.has_b_king_side_castle || board.b_king_has_moved)
                    && board.b_king & 0x800000000000000 > 0
                    && board.b_rooks & 0x100000000000000 > 0
                    && board.getOcupancy() & 0x600000000000000 == 0
                    && attacts & 0xe00000000000000 == 0;
            }
        }
    }

    pub fn can_castle_queen_side(board: Board, playing_as: Turn) -> bool {
        match playing_as {
            Turn::White => {
                let attacts = Movement::extract_all_attacks_for_color(board, Turn::Black);

                return !(board.has_w_queen_side_castle || board.w_king_has_moved)
                    && board.w_king & 0x8 > 0
                    && board.w_rooks & 0x80 > 0
                    && board.getOcupancy() & 0x70 == 0
                    && attacts & 0x78 == 0;
            }
            Turn::Black => {
                let attacts = Movement::extract_all_attacks_for_color(board, Turn::White);

                return !(board.has_b_queen_side_castle || board.b_king_has_moved)
                    && board.b_king & 0x800000000000000 > 0
                    && board.b_rooks & 0x8000000000000000 > 0
                    && board.getOcupancy() & 0x7000000000000000 == 0
                    && attacts & 0x7800000000000000 == 0;
            }
        }
    }

    //Returns true if can castle
    pub fn can_castle(board: Board, playing_as: Turn, movve: LegalMove) -> bool {
        match playing_as {
            Turn::White => {
                let attacts = Movement::extract_all_attacks_for_color(board, Turn::Black);

                match movve.castle {
                    CastleOptions::KingSide => {
                        return Movement::can_castle_king_side(board, playing_as);
                    }
                    CastleOptions::QueenSide => {
                        return Movement::can_castle_queen_side(board, playing_as);
                    }
                    CastleOptions::None => {
                        return false;
                    }
                }
            }
            Turn::Black => {
                let attacts = Movement::extract_all_attacks_for_color(board, Turn::White);
                match movve.castle {
                    CastleOptions::KingSide => {
                        return Movement::can_castle_king_side(board, playing_as);
                    }
                    CastleOptions::QueenSide => {
                        return Movement::can_castle_queen_side(board, playing_as);
                    }
                    CastleOptions::None => {
                        return false;
                    }
                }
            }
        }
    }

    pub fn check_for_captures_at(check_for: &Turn, board: Board, at_bitboard: u64) -> bool {
        let pawns: u64;
        let knights: u64;
        let bishops: u64;
        let rooks: u64;
        let queen: u64;
        let king: u64;
        match check_for {
            Turn::White => {
                pawns = Movement::extract_pieces_moves_from_bitboard(
                    board.b_pawns,
                    Turn::White,
                    PieceType::WhitePawn,
                    board,
                );
                knights = Movement::extract_pieces_moves_from_bitboard(
                    board.b_knights,
                    Turn::White,
                    PieceType::WhiteKnight,
                    board,
                );
                rooks = Movement::extract_pieces_moves_from_bitboard(
                    board.b_rooks,
                    Turn::White,
                    PieceType::WhiteRook,
                    board,
                );
                bishops = Movement::extract_pieces_moves_from_bitboard(
                    board.b_bishops,
                    Turn::White,
                    PieceType::WhiteBishop,
                    board,
                );
                queen = Movement::extract_pieces_moves_from_bitboard(
                    board.b_queen,
                    Turn::White,
                    PieceType::WhiteQueen,
                    board,
                );
                king = Movement::get_king_moves(board.w_king, Turn::White, board);

                let bitboard_merge =
                    (pawns | knights | rooks | bishops | queen | king) & at_bitboard;

                return bitboard_merge != 0;
            }
            Turn::Black => {
                pawns = Movement::extract_pieces_moves_from_bitboard(
                    board.w_pawns,
                    Turn::White,
                    PieceType::BlackPawn,
                    board,
                );
                knights = Movement::extract_pieces_moves_from_bitboard(
                    board.w_knights,
                    Turn::Black,
                    PieceType::BlackKnight,
                    board,
                );
                rooks = Movement::extract_pieces_moves_from_bitboard(
                    board.w_rooks,
                    Turn::Black,
                    PieceType::BlackRook,
                    board,
                );
                bishops = Movement::extract_pieces_moves_from_bitboard(
                    board.w_bishops,
                    Turn::Black,
                    PieceType::BlackBishop,
                    board,
                );
                queen = Movement::extract_pieces_moves_from_bitboard(
                    board.w_queen,
                    Turn::Black,
                    PieceType::BlackQueen,
                    board,
                );

                king = Movement::get_king_moves(board.w_king, Turn::Black, board);

                let bitboard_merge =
                    (pawns | knights | rooks | bishops | queen | king) & at_bitboard;

                return bitboard_merge != 0;
            }
        }
    }

    pub fn check_for_checkmate(check_for: Turn, board: Board) -> bool {
        match check_for {
            Turn::White => {
                return Movement::check_for_check(check_for, board)
                    && (Movement::extract_all_attacks_for_color(board, Turn::White)
                        & Movement::extract_all_attacks_for_color(board, Turn::Black))
                        == 0
                    && Movement::get_king_moves(board.w_king, check_for, board)
                        & Movement::extract_all_attacks_for_color(board, Turn::Black)
                        == 0;
            }
            Turn::Black => {
                return Movement::check_for_check(check_for, board)
                    && (Movement::extract_all_attacks_for_color(board, Turn::White)
                        & Movement::extract_all_attacks_for_color(board, Turn::Black))
                        == 0
                    && Movement::get_king_moves(board.w_king, check_for, board)
                        & Movement::extract_all_attacks_for_color(board, Turn::White)
                        != 0;
            }
        }
    }

    pub fn get_color_attacks(check_for: Turn, board: Board) -> u64 {
        let pawns: u64;
        let knights: u64;
        let bishops: u64;
        let rooks: u64;
        let queen: u64;
        let king: u64;
        match check_for {
            Turn::Black => {
                pawns = Movement::extract_pieces_moves_from_bitboard(
                    board.b_pawns,
                    Turn::Black,
                    PieceType::BlackPawn,
                    board,
                );
                knights = Movement::extract_pieces_moves_from_bitboard(
                    board.b_knights,
                    Turn::Black,
                    PieceType::BlackKnight,
                    board,
                );
                rooks = Movement::extract_pieces_moves_from_bitboard(
                    board.b_rooks,
                    Turn::Black,
                    PieceType::BlackRook,
                    board,
                );
                bishops = Movement::extract_pieces_moves_from_bitboard(
                    board.b_bishops,
                    Turn::Black,
                    PieceType::BlackBishop,
                    board,
                );
                queen = Movement::extract_pieces_moves_from_bitboard(
                    board.b_queen,
                    Turn::Black,
                    PieceType::BlackQueen,
                    board,
                );
                king = Movement::get_king_moves(board.b_king, Turn::Black, board);

                let bitboard_merge = pawns | knights | rooks | bishops | queen | king;

                return bitboard_merge;
            }
            Turn::White => {
                pawns = Movement::extract_pieces_moves_from_bitboard(
                    board.w_pawns,
                    Turn::White,
                    PieceType::WhitePawn,
                    board,
                );
                knights = Movement::extract_pieces_moves_from_bitboard(
                    board.w_knights,
                    Turn::White,
                    PieceType::WhiteKnight,
                    board,
                );
                rooks = Movement::extract_pieces_moves_from_bitboard(
                    board.w_rooks,
                    Turn::White,
                    PieceType::WhiteRook,
                    board,
                );
                bishops = Movement::extract_pieces_moves_from_bitboard(
                    board.w_bishops,
                    Turn::White,
                    PieceType::WhiteBishop,
                    board,
                );
                queen = Movement::extract_pieces_moves_from_bitboard(
                    board.w_queen,
                    Turn::White,
                    PieceType::WhiteQueen,
                    board,
                );
                king = Movement::get_king_moves(board.b_king, Turn::Black, board);

                let bitboard_merge = pawns | knights | rooks | bishops | queen | king;

                return bitboard_merge;
            }
        }
    }

    pub fn check_for_check(check_for: Turn, board: Board) -> bool {
        let pawns: u64;
        let knights: u64;
        let bishops: u64;
        let rooks: u64;
        let queen: u64;
        if matches!(check_for, Turn::White) {
            pawns = Movement::extract_pieces_moves_from_bitboard(
                board.b_pawns,
                Turn::Black,
                PieceType::BlackPawn,
                board,
            );
            knights = Movement::extract_pieces_moves_from_bitboard(
                board.b_knights,
                Turn::Black,
                PieceType::BlackKnight,
                board,
            );
            rooks = Movement::extract_pieces_moves_from_bitboard(
                board.b_rooks,
                Turn::Black,
                PieceType::BlackRook,
                board,
            );
            bishops = Movement::extract_pieces_moves_from_bitboard(
                board.b_bishops,
                Turn::Black,
                PieceType::BlackBishop,
                board,
            );
            queen = Movement::extract_pieces_moves_from_bitboard(
                board.b_queen,
                Turn::Black,
                PieceType::BlackQueen,
                board,
            );

            let bitboard_merge = (pawns | knights | rooks | bishops | queen) & board.w_king;

            return bitboard_merge != 0;
        }
        if matches!(check_for, Turn::Black) {
            pawns = Movement::extract_pieces_moves_from_bitboard(
                board.w_pawns,
                Turn::White,
                PieceType::WhitePawn,
                board,
            );
            knights = Movement::extract_pieces_moves_from_bitboard(
                board.w_knights,
                Turn::White,
                PieceType::WhiteKnight,
                board,
            );
            rooks = Movement::extract_pieces_moves_from_bitboard(
                board.w_rooks,
                Turn::White,
                PieceType::WhiteRook,
                board,
            );
            bishops = Movement::extract_pieces_moves_from_bitboard(
                board.w_bishops,
                Turn::White,
                PieceType::WhiteBishop,
                board,
            );
            queen = Movement::extract_pieces_moves_from_bitboard(
                board.w_queen,
                Turn::White,
                PieceType::WhiteQueen,
                board,
            );

            let bitboard_merge = (pawns | knights | rooks | bishops | queen) & board.b_king;

            return bitboard_merge != 0;
        }
        return false;
    }

    pub fn check_for_check_at(check_for: &Turn, board: Board, at_bitboard: u64) -> bool {
        let pawns: u64;
        let knights: u64;
        let bishops: u64;
        let rooks: u64;
        let queen: u64;
        if matches!(check_for, Turn::White) {
            pawns = Movement::extract_pieces_moves_from_bitboard(
                board.b_pawns,
                Turn::Black,
                PieceType::BlackPawn,
                board,
            );
            knights = Movement::extract_pieces_moves_from_bitboard(
                board.b_knights,
                Turn::Black,
                PieceType::BlackKnight,
                board,
            );
            rooks = Movement::extract_pieces_moves_from_bitboard(
                board.b_rooks,
                Turn::Black,
                PieceType::BlackRook,
                board,
            );
            bishops = Movement::extract_pieces_moves_from_bitboard(
                board.b_bishops,
                Turn::Black,
                PieceType::BlackBishop,
                board,
            );
            queen = Movement::extract_pieces_moves_from_bitboard(
                board.b_queen,
                Turn::Black,
                PieceType::BlackQueen,
                board,
            );

            let bitboard_merge = (pawns | knights | rooks | bishops | queen) & at_bitboard;

            return bitboard_merge != 0;
        }
        if matches!(check_for, Turn::Black) {
            pawns = Movement::extract_pieces_moves_from_bitboard(
                board.w_pawns,
                Turn::White,
                PieceType::WhitePawn,
                board,
            );
            knights = Movement::extract_pieces_moves_from_bitboard(
                board.w_knights,
                Turn::White,
                PieceType::WhiteKnight,
                board,
            );
            rooks = Movement::extract_pieces_moves_from_bitboard(
                board.w_rooks,
                Turn::White,
                PieceType::WhiteRook,
                board,
            );
            bishops = Movement::extract_pieces_moves_from_bitboard(
                board.w_bishops,
                Turn::White,
                PieceType::WhiteBishop,
                board,
            );
            queen = Movement::extract_pieces_moves_from_bitboard(
                board.w_queen,
                Turn::White,
                PieceType::WhiteQueen,
                board,
            );

            let bitboard_merge = (pawns | knights | rooks | bishops | queen) & at_bitboard;

            return bitboard_merge != 0;
        }
        return false;
    }

    fn extract_piece_moves_from_bitboard(
        piece_bits: u64,
        color: Turn,
        piece_type: PieceType,
        board: Board,
    ) -> u64 {
        match piece_type {
            PieceType::WhiteBishop => {
                return Movement::get_bishop_moves(piece_bits, color, board);
            }
            PieceType::WhiteKnight => return Movement::get_knigth_moves(piece_bits, color, board),
            PieceType::WhiteRook => return Movement::get_rook_moves(piece_bits, color, board),
            PieceType::WhiteQueen => return Movement::get_queen_moves(piece_bits, color, board),
            PieceType::WhitePawn => return Movement::get_pawn_moves(piece_bits, color, board),
            PieceType::WhiteKing => return Movement::get_king_moves(piece_bits, color, board),
            PieceType::BlackBishop => return Movement::get_bishop_moves(piece_bits, color, board),
            PieceType::BlackKnight => return Movement::get_knigth_moves(piece_bits, color, board),
            PieceType::BlackRook => return Movement::get_rook_moves(piece_bits, color, board),
            PieceType::BlackQueen => return Movement::get_queen_moves(piece_bits, color, board),
            PieceType::BlackPawn => return Movement::get_pawn_moves(piece_bits, color, board),
            PieceType::BlackKing => return Movement::get_king_moves(piece_bits, color, board),
            _ => 0,
        }
    }

    pub fn extract_pieces_moves_from_bitboard(
        mut pieces_bits: u64,
        color: Turn,
        piece_type: PieceType,
        board: Board,
    ) -> u64 {
        if pieces_bits == 0 {
            return 0;
        } else {
            let mut move_bits = 0;
            while pieces_bits > 0 {
                let piece_pos = Movement::msb_pos(pieces_bits);
                let piece_bits = (1 as u64) << piece_pos;
                move_bits |= Movement::extract_piece_moves_from_bitboard(
                    piece_bits, color, piece_type, board,
                );
                pieces_bits = pieces_bits & !piece_bits;
            }

            return move_bits;
        }
    }

    pub fn extract_all_attacks_for_color(board: Board, color: Turn) -> u64 {
        match color {
            Turn::White => {
                let mut white_attacks = Movement::extract_pieces_moves_from_bitboard(
                    board.w_pawns,
                    color,
                    PieceType::WhitePawn,
                    board,
                );

                white_attacks |= Movement::extract_pieces_moves_from_bitboard(
                    board.w_rooks,
                    color,
                    PieceType::WhiteRook,
                    board,
                );

                white_attacks |= Movement::extract_pieces_moves_from_bitboard(
                    board.w_bishops,
                    color,
                    PieceType::WhiteBishop,
                    board,
                );
                white_attacks |= Movement::extract_pieces_moves_from_bitboard(
                    board.w_knights,
                    color,
                    PieceType::WhiteKnight,
                    board,
                );
                white_attacks |= Movement::extract_pieces_moves_from_bitboard(
                    board.w_queen,
                    color,
                    PieceType::WhiteQueen,
                    board,
                );

                white_attacks |= Movement::extract_pieces_moves_from_bitboard(
                    board.w_king,
                    color,
                    PieceType::WhiteKing,
                    board,
                );

                return white_attacks;
            }
            Turn::Black => {
                let mut black_attacks = Movement::extract_pieces_moves_from_bitboard(
                    board.b_pawns,
                    color,
                    PieceType::BlackPawn,
                    board,
                );

                black_attacks |= Movement::extract_pieces_moves_from_bitboard(
                    board.b_rooks,
                    color,
                    PieceType::BlackRook,
                    board,
                );

                black_attacks |= Movement::extract_pieces_moves_from_bitboard(
                    board.b_bishops,
                    color,
                    PieceType::BlackBishop,
                    board,
                );
                black_attacks |= Movement::extract_pieces_moves_from_bitboard(
                    board.b_knights,
                    color,
                    PieceType::BlackKnight,
                    board,
                );
                black_attacks |= Movement::extract_pieces_moves_from_bitboard(
                    board.b_queen,
                    color,
                    PieceType::BlackQueen,
                    board,
                );

                black_attacks |= Movement::extract_pieces_moves_from_bitboard(
                    board.b_king,
                    color,
                    PieceType::BlackKing,
                    board,
                );

                return black_attacks;
            }
        }
    }

    pub fn get_knigth_moves(knight_bits: u64, color: Turn, board: Board) -> u64 {
        return Knight::get_moves(knight_bits, color, board);
    }

    pub fn get_pawn_moves(pawn_bits: u64, color: Turn, board: Board) -> u64 {
        return Pawn::get_moves(pawn_bits, color, board);
    }

    pub fn is_enpassant(piece_bits: u64, en_passant_pos: u64) -> bool {
        return (piece_bits << 1 & en_passant_pos != 0) || (piece_bits >> 1 & en_passant_pos != 0);
    }

    pub fn get_pawn_moves_enpassant(pawn_bits: u64, enpassant_bits: u64, color: Turn) -> u64 {
        return Pawn::get_moves_enpassant(pawn_bits, enpassant_bits, color);
    }

    pub fn get_rook_moves(rook_bits: u64, color: Turn, board: Board) -> u64 {
        return Rook::get_moves(rook_bits, color, board);
    }

    pub fn get_bishop_moves(bishop_bits: u64, color: Turn, board: Board) -> u64 {
        return Bishop::get_moves(bishop_bits, color, board);
    }

    pub fn get_queen_moves(queen_bits: u64, color: Turn, board: Board) -> u64 {
        return Queen::get_moves(queen_bits, color, board);
    }

    pub fn get_king_moves(king_bits: u64, color: Turn, board: Board) -> u64 {
        return King::get_moves(king_bits, color, board);
    }
}
