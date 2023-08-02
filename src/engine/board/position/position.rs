use crate::engine::board::board::{PieceType, Turn};
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub enum CastleOptions {
    None,
    KingSide,
    QueenSide,
}
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct Position {
    pub colum: i8,
    pub row: i8,
}

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

#[derive(Clone, Copy)]
pub struct UnCheckedMove {
    pub from: Position,
    pub to: Position,
    pub piece: PieceType,
    pub castle: CastleOptions,
}

impl UnCheckedMove {
    pub fn new(
        from_row: i8,
        from_column: i8,
        to_row: i8,
        to_colum: i8,
        piece: PieceType,
    ) -> UnCheckedMove {
        return UnCheckedMove {
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

    pub fn to_legal_move(analyzer_move: UnCheckedMove) -> LegalMove {
        return LegalMove {
            from: analyzer_move.from,
            to: analyzer_move.to,
            castle: analyzer_move.castle,
        };
    }

    pub fn king_side_castle(color: Turn) -> UnCheckedMove {
        match color {
            Turn::White => {
                return UnCheckedMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    piece: PieceType::WhiteKing,
                    castle: CastleOptions::KingSide,
                };
            }
            Turn::Black => {
                return UnCheckedMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    piece: PieceType::WhiteKing,
                    castle: CastleOptions::KingSide,
                };
            }
        }
    }

    pub fn queen_side_castle(color: Turn) -> UnCheckedMove {
        match color {
            Turn::White => {
                return UnCheckedMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    piece: PieceType::WhiteKing,
                    castle: CastleOptions::QueenSide,
                };
            }
            Turn::Black => {
                return UnCheckedMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    piece: PieceType::WhiteKing,
                    castle: CastleOptions::QueenSide,
                };
            }
        }
    }
}
#[derive(PartialEq, PartialOrd, Clone, Copy)]
pub struct LegalMove {
    pub from: Position,
    pub to: Position,
    pub castle: CastleOptions,
}

impl LegalMove {
    pub fn new(from_row: i8, from_column: i8, to_row: i8, to_column: i8) -> LegalMove {
        return LegalMove {
            from: (Position {
                colum: from_column,
                row: from_row,
            }),
            to: Position {
                colum: to_column,
                row: to_row,
            },
            castle: CastleOptions::None,
        };
    }

    pub fn king_side_castle(color: Turn) -> LegalMove {
        match color {
            Turn::White => {
                return LegalMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    castle: CastleOptions::KingSide,
                };
            }
            Turn::Black => {
                return LegalMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    castle: CastleOptions::KingSide,
                };
            }
        }
    }

    pub fn queen_side_castle(color: Turn) -> LegalMove {
        match color {
            Turn::White => {
                return LegalMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    castle: CastleOptions::QueenSide,
                };
            }
            Turn::Black => {
                return LegalMove {
                    from: (Position { colum: 0, row: 0 }),
                    to: Position { colum: 0, row: 0 },
                    castle: CastleOptions::QueenSide,
                };
            }
        }
    }
}
