use std::{f32::consts::E, io::Read};

use crate::engine::{
    board::{
        board::{Board, Turn},
        position::position::{CastleOptions, LegalMove, Position},
    },
    game::analyzer,
    movement::{
        bishop::bishop::Bishop, king::king::King, knight::knight::Knight, movement::Movement,
        pawn::pawn::Pawn, queen::queen::Queen, rook::rook::Rook,
    },
};

#[derive(Clone, Copy)]
pub enum AnalyzerPieceType {
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
    WhitePawnEnPassant,

    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
    BlackPawnEnPassant,
}

pub struct AnalyzerMove {
    pub from: Position,
    pub to: Position,
    pub piece: AnalyzerPieceType,
    pub castle: CastleOptions,
}

impl AnalyzerMove {
    pub fn new(
        from_row: i8,
        from_column: i8,
        to_row: i8,
        to_colum: i8,
        piece: AnalyzerPieceType,
    ) -> AnalyzerMove {
        return AnalyzerMove {
            from: (Position {
                colum: from_column,
                row: from_row,
            }),
            to: Position {
                colum: to_colum,
                row: to_row,
            },
            piece: piece,
            castle: CastleOptions::None,
        };
    }

    pub fn to_legal_move(analyzer_move: AnalyzerMove) -> LegalMove {
        return LegalMove {
            from: analyzer_move.from,
            to: analyzer_move.to,
            castle: analyzer_move.castle,
        };
    }

    pub fn king_side_castle(color: Turn) -> AnalyzerMove {
        match color {
            Turn::White => {
                return AnalyzerMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    piece: AnalyzerPieceType::WhiteKing,
                    castle: CastleOptions::KingSide,
                };
            }
            Turn::Black => {
                return AnalyzerMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    piece: AnalyzerPieceType::WhiteKing,
                    castle: CastleOptions::KingSide,
                };
            }
        }
    }

    pub fn queen_side_castle(color: Turn) -> AnalyzerMove {
        match color {
            Turn::White => {
                return AnalyzerMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    piece: AnalyzerPieceType::WhiteKing,
                    castle: CastleOptions::QueenSide,
                };
            }
            Turn::Black => {
                return AnalyzerMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    piece: AnalyzerPieceType::WhiteKing,
                    castle: CastleOptions::QueenSide,
                };
            }
        }
    }
}

pub struct AnalyzerMovement {}

impl AnalyzerMovement {
    fn translate_analizer_moves(analyzer_moves: Vec<AnalyzerMove>) -> Vec<LegalMove> {
        let mut legal_moves = Vec::new();
        for analyzer_move in analyzer_moves {
            legal_moves.push(AnalyzerMove::to_legal_move(analyzer_move));
        }
        return legal_moves;
    }

    fn is_move_legal(legal_move: LegalMove, board: Board, playing_as: Turn) -> bool {
        let mut board_check = board.to_owned();
        board_check.do_move(legal_move, playing_as);
        return !Movement::check_for_check(playing_as, board_check);
    }

    fn legalize_moves(moves: Vec<AnalyzerMove>, board: Board, playing_as: Turn) -> Vec<LegalMove> {
        let mut legal_moves: Vec<LegalMove> = Vec::new();

        for movve in moves {
            let legal_move = AnalyzerMove::to_legal_move(movve);
            let mut boad_copy = board.to_owned();
            if AnalyzerMovement::is_move_legal(legal_move, board, playing_as) {
                legal_moves.push(legal_move);
            }
        }

        return legal_moves;
    }

    pub fn get_moves(board: Board, color: Turn) -> Vec<LegalMove> {
        return AnalyzerMovement::legalize_moves(
            AnalyzerMovement::extract_all_moves_for_color(board, color),
            board,
            color,
        );
    }

    pub fn extract_all_moves_for_color(board: Board, color: Turn) -> Vec<AnalyzerMove> {
        let mut moves = Vec::new();
        let mut castling = Vec::new();
        if Movement::can_castle_king_side(board, color) {
            castling.push(AnalyzerMove::king_side_castle(color));
        }

        if Movement::can_castle_queen_side(board, color) {
            castling.push(AnalyzerMove::queen_side_castle(color));
        }

        moves.extend(castling);
        match color {
            Turn::White => {
                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.w_pawns,
                    color,
                    AnalyzerPieceType::WhitePawn,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));
                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.w_pawns,
                    color,
                    AnalyzerPieceType::WhitePawn,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));

                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.w_rooks,
                    color,
                    AnalyzerPieceType::WhiteRook,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));

                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.w_bishops,
                    color,
                    AnalyzerPieceType::WhiteBishop,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));
                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.w_knights,
                    color,
                    AnalyzerPieceType::WhiteKnight,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));
                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.w_queen,
                    color,
                    AnalyzerPieceType::WhiteQueen,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));

                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.w_king,
                    color,
                    AnalyzerPieceType::WhiteKing,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));
            }
            Turn::Black => {
                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.b_pawns,
                    color,
                    AnalyzerPieceType::BlackPawn,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));

                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.b_rooks,
                    color,
                    AnalyzerPieceType::BlackRook,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));

                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.b_bishops,
                    color,
                    AnalyzerPieceType::BlackBishop,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));
                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.b_knights,
                    color,
                    AnalyzerPieceType::BlackKnight,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));
                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.b_queen,
                    color,
                    AnalyzerPieceType::BlackQueen,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));

                moves.extend(AnalyzerMovement::extract_pieces_moves_from_bitboard(
                    board.b_king,
                    color,
                    AnalyzerPieceType::BlackKing,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                ));
            }
        }
        return moves;
    }

    fn extract_pieces_moves_from_bitboard(
        mut pieces_bits: u64,
        color: Turn,
        piece_type: AnalyzerPieceType,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> Vec<AnalyzerMove> {
        if pieces_bits == 0 {
            return Vec::new();
        } else {
            let mut legal_moves: Vec<AnalyzerMove> = Vec::new();

            while pieces_bits > 0 {
                let piece_pos = Movement::msb_pos(pieces_bits);
                let piece_bits = (1 as u64) << piece_pos;
                let result = AnalyzerMovement::extract_piece_moves_from_bitboard(
                    piece_bits,
                    color,
                    piece_type,
                    white_bitboard,
                    black_bitboard,
                );
                for elem in result {
                    legal_moves.push(elem)
                }
                pieces_bits = pieces_bits & !piece_bits;
            }

            return legal_moves;
        }
    }

    fn extract_piece_moves_from_bitboard(
        piece_bits: u64,
        color: Turn,
        piece_type: AnalyzerPieceType,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> Vec<AnalyzerMove> {
        match piece_type {
            AnalyzerPieceType::WhiteBishop => {
                return AnalyzerMovement::get_bishop_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                );
            }
            AnalyzerPieceType::WhiteKnight => {
                return AnalyzerMovement::get_knigth_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            AnalyzerPieceType::WhiteRook => {
                return AnalyzerMovement::get_rook_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            AnalyzerPieceType::WhiteQueen => {
                return AnalyzerMovement::get_queen_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            AnalyzerPieceType::WhitePawn => {
                return AnalyzerMovement::get_pawn_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            AnalyzerPieceType::WhiteKing => {
                return AnalyzerMovement::get_king_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            AnalyzerPieceType::BlackBishop => {
                return AnalyzerMovement::get_bishop_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            AnalyzerPieceType::BlackKnight => {
                return AnalyzerMovement::get_knigth_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            AnalyzerPieceType::BlackRook => {
                return AnalyzerMovement::get_rook_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            AnalyzerPieceType::BlackQueen => {
                return AnalyzerMovement::get_queen_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            AnalyzerPieceType::BlackPawn => {
                return AnalyzerMovement::get_pawn_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            AnalyzerPieceType::BlackKing => {
                return AnalyzerMovement::get_king_moves(
                    piece_bits,
                    color,
                    white_bitboard,
                    black_bitboard,
                )
            }
            _ => return Vec::new(),
        }
    }

    fn translate_piece_moves_from_bitboard(
        piece_bits: u64,
        mut piece_move_bits: u64,
        color: Turn,
        white_piece: AnalyzerPieceType,
        black_piece: AnalyzerPieceType,
    ) -> Vec<AnalyzerMove> {
        let piece_index = Movement::get_piece_index(piece_bits);
        let from_column: i8 = piece_index % 8;
        let from_row: i8 = piece_index / 8;

        let mut legal_moves: Vec<AnalyzerMove> = Vec::new();

        while piece_move_bits > 0 {
            let piece_pos = Movement::msb_pos(piece_move_bits);
            let piece_bits = (1 as u64) << piece_pos;

            let to_pos_index = Movement::get_piece_index(piece_bits);
            let to_column = to_pos_index % 8;
            let to_row = to_pos_index / 8;

            match color {
                Turn::Black => legal_moves.push(AnalyzerMove::new(
                    from_row,
                    from_column,
                    to_row,
                    to_column,
                    black_piece,
                )),
                Turn::White => legal_moves.push(AnalyzerMove::new(
                    from_row,
                    from_column,
                    to_row,
                    to_column,
                    white_piece,
                )),
            }

            piece_move_bits = piece_move_bits & !piece_bits;
        }

        return legal_moves;
    }

    fn get_knigth_moves(
        knight_bits: u64,
        color: Turn,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> Vec<AnalyzerMove> {
        let bitboard_moves = Knight::get_moves(knight_bits, color, white_bitboard, black_bitboard);

        return AnalyzerMovement::translate_piece_moves_from_bitboard(
            knight_bits,
            bitboard_moves,
            color,
            AnalyzerPieceType::WhiteKnight,
            AnalyzerPieceType::BlackKnight,
        );
    }

    fn get_pawn_moves(
        pawn_bits: u64,
        color: Turn,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> Vec<AnalyzerMove> {
        let bitboard_moves = Pawn::get_moves(pawn_bits, color, white_bitboard, black_bitboard);
        return AnalyzerMovement::translate_piece_moves_from_bitboard(
            pawn_bits,
            bitboard_moves,
            color,
            AnalyzerPieceType::WhitePawn,
            AnalyzerPieceType::BlackPawn,
        );
    }

    fn is_enpassant(piece_bits: u64, en_passant_pos: u64) -> bool {
        return (piece_bits << 1 & en_passant_pos != 0) || (piece_bits >> 1 & en_passant_pos != 0);
    }

    fn get_pawn_moves_enpassant(
        pawn_bits: u64,
        enpassant_bits: u64,
        color: Turn,
    ) -> Vec<AnalyzerMove> {
        let bitboard_moves = Pawn::get_moves_enpassant(pawn_bits, enpassant_bits, color);
        return AnalyzerMovement::translate_piece_moves_from_bitboard(
            pawn_bits,
            bitboard_moves,
            color,
            AnalyzerPieceType::WhitePawnEnPassant,
            AnalyzerPieceType::BlackPawnEnPassant,
        );
    }

    fn get_rook_moves(
        rook_bits: u64,
        color: Turn,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> Vec<AnalyzerMove> {
        let bitboard_moves = Rook::get_moves(rook_bits, color, white_bitboard, black_bitboard);

        return AnalyzerMovement::translate_piece_moves_from_bitboard(
            rook_bits,
            bitboard_moves,
            color,
            AnalyzerPieceType::WhiteRook,
            AnalyzerPieceType::BlackRook,
        );
    }

    fn get_bishop_moves(
        bishop_bits: u64,
        color: Turn,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> Vec<AnalyzerMove> {
        let bitboard_moves = Bishop::get_moves(bishop_bits, color, white_bitboard, black_bitboard);

        return AnalyzerMovement::translate_piece_moves_from_bitboard(
            bishop_bits,
            bitboard_moves,
            color,
            AnalyzerPieceType::WhiteBishop,
            AnalyzerPieceType::BlackBishop,
        );
    }

    fn get_queen_moves(
        queen_bits: u64,
        color: Turn,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> Vec<AnalyzerMove> {
        let bitboard_moves = Queen::get_moves(queen_bits, color, white_bitboard, black_bitboard);

        return AnalyzerMovement::translate_piece_moves_from_bitboard(
            queen_bits,
            bitboard_moves,
            color,
            AnalyzerPieceType::WhiteQueen,
            AnalyzerPieceType::BlackQueen,
        );
    }

    fn get_king_moves(
        king_bits: u64,
        color: Turn,
        white_bitboard: u64,
        black_bitboard: u64,
    ) -> Vec<AnalyzerMove> {
        let bitboard_moves = King::get_moves(king_bits, color, white_bitboard, black_bitboard);

        return AnalyzerMovement::translate_piece_moves_from_bitboard(
            king_bits,
            bitboard_moves,
            color,
            AnalyzerPieceType::WhiteKing,
            AnalyzerPieceType::BlackKing,
        );
    }
}
