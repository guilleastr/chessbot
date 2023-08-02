use crate::engine::{movement::movement::Movement, printer::printer::Printer};
use crate::engine::{
    movement::{
        bishop::bishop::Bishop, king::king::King, knight::knight::Knight, movement::Movement,
        pawn::pawn::Pawn, queen::queen::Queen, rook::rook::Rook,
    },
    printer::printer::Printer,
};

use super::{
    fenn::fenn::FEN,
    position::position::{CastleOptions, LegalMove, Position},
};

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Turn {
    White,
    Black,
}
impl std::fmt::Display for LegalMove {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            fmt,
            "From c r {} {} to c r{} {} .",
            self.from.colum, self.from.row, self.to.colum, self.to.row
        )
    }
}

#[derive(Clone, Copy)]
pub enum PieceType {
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,

    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,

    WhitePawnEnPassant,
    BlackPawnEnPassant,
}

pub struct PieceVec {
    vec: Vec<i8>,
}

impl PieceType {
    fn delete(&mut self) {}
}

#[derive(Clone, Copy)]
pub struct Board {
    /*     pub w_rooks_array_pos: Vec<i8>,
    pub w_knights_array_pos: Vec<i8>,
    pub w_bishops_array_pos: Vec<i8>,
    pub w_queen_array_pos: Vec<i8>,
    pub w_king_array_pos: Vec<i8>,
    pub w_pawns_array_pos: Vec<i8>,

    pub b_rooks_array_pos: Vec<i8>,
    pub b_knights_array_pos: Vec<i8>,
    pub b_bishops_array_pos: Vec<i8>,
    pub b_queen_array_pos: Vec<i8>,
    pub b_king_array_pos: Vec<i8>,
    pub b_pawns_array_pos: Vec<i8>, */
    pub w_rooks: u64,
    pub w_knights: u64,
    pub w_bishops: u64,
    pub w_queen: u64,
    pub w_king: u64,
    pub w_pawns: u64,

    pub b_rooks: u64,
    pub b_knights: u64,
    pub b_bishops: u64,
    pub b_queen: u64,
    pub b_king: u64,
    pub b_pawns: u64,

    pub has_w_king_side_castle: bool,
    pub has_w_queen_side_castle: bool,
    pub w_king_has_moved: bool,

    pub has_b_king_side_castle: bool,
    pub has_b_queen_side_castle: bool,
    pub b_king_has_moved: bool,

    pub w_en_passant: u64,
    pub b_en_passant: u64,

    pub full_move_count: i8,
    pub half_move_count: i8,

    pub black_attacks: u64,
    pub white_attacks: u64,

    pub turn: Turn,
}

impl Board {
    pub fn new() -> Board {
        return FEN::get_board_from_fenn_str(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        );
    }

    pub fn new_empty() -> Board {
        return Board {
            /* w_rooks_array_pos: Vec::new(),
            w_knights_array_pos: Vec::new(),
            w_bishops_array_pos: Vec::new(),
            w_queen_array_pos: Vec::new(),
            w_king_array_pos: Vec::new(),
            w_pawns_array_pos: Vec::new(),

            b_rooks_array_pos: Vec::new(),
            b_knights_array_pos: Vec::new(),
            b_bishops_array_pos: Vec::new(),
            b_queen_array_pos: Vec::new(),
            b_king_array_pos: Vec::new(),
            b_pawns_array_pos: Vec::new(), */
            w_rooks: 0,
            w_knights: 0,
            w_bishops: 0,
            w_queen: 0,
            w_king: 0,
            w_pawns: 0,

            b_rooks: 0,
            b_knights: 0,
            b_bishops: 0,
            b_queen: 0,
            b_king: 0,
            b_pawns: 0,

            w_en_passant: 0,
            b_en_passant: 0,

            has_w_king_side_castle: false,
            has_w_queen_side_castle: false,
            w_king_has_moved: false,

            has_b_king_side_castle: false,
            has_b_queen_side_castle: false,
            b_king_has_moved: false,

            full_move_count: 0,
            half_move_count: 0,

            black_attacks: 0,
            white_attacks: 0,

            turn: Turn::White,
        };
    }

    pub fn new_from_fenn_notation(board_fenn: &'static str) -> Board {
        return FEN::get_board_from_fenn_str(board_fenn);
    }

    pub fn get_turn(&self) -> Turn {
        return self.turn;
    }

    pub fn set_turn(&mut self, turn: Turn) {
        self.turn = turn;
    }

    pub fn getOcupancy(&self) -> u64 {
        return self.getWhiteBitboard() | self.getBlackBitboard();
    }

    pub fn getNotOcupancy(&self) -> u64 {
        return !self.getOcupancy();
    }

    pub fn getWhiteBitboard(&self) -> u64 {
        return self.w_rooks
            | self.w_knights
            | self.w_bishops
            | self.w_queen
            | self.w_king
            | self.w_pawns;
    }

    pub fn getBlackBitboard(&self) -> u64 {
        return self.b_rooks
            | self.b_knights
            | self.b_bishops
            | self.b_queen
            | self.b_king
            | self.b_pawns;
    }

    fn try_take(&mut self, destin_board: u64) -> bool {
        let board_ocupancy = self.getOcupancy();
        if self.getWhiteBitboard() & destin_board > 0 {
            if self.w_rooks & destin_board > 0 {
                if destin_board < 0x10 {
                    self.has_w_king_side_castle = true;
                } else {
                    self.has_w_queen_side_castle = true;
                }
                self.w_rooks = self.w_rooks & !destin_board;
            }
            if self.w_bishops & destin_board > 0 {
                self.w_bishops = self.w_bishops & !destin_board;
            }
            if self.w_knights & destin_board > 0 {
                self.w_knights = self.w_knights & !destin_board;
            }
            if self.w_pawns & destin_board > 0 {
                self.w_pawns = self.w_pawns & !destin_board;
            }
            if self.w_queen & destin_board > 0 {
                self.w_queen = self.w_queen & !destin_board;
            }
        }

        if self.getBlackBitboard() & destin_board > 0 {
            if self.b_rooks & destin_board > 0 {
                if destin_board < 0x10 {
                    self.has_b_king_side_castle = true;
                } else {
                    self.has_b_queen_side_castle = true;
                }
                self.b_rooks = self.b_rooks & !destin_board;
            }
            if self.b_bishops & destin_board > 0 {
                self.b_bishops = self.b_bishops & !destin_board;
            }
            if self.b_knights & destin_board > 0 {
                self.b_knights = self.b_knights & !destin_board;
            }
            if self.b_pawns & destin_board > 0 {
                self.b_pawns = self.b_pawns & !destin_board;
            }
            if self.b_queen & destin_board > 0 {
                self.b_queen = self.b_queen & !destin_board;
            }
        }

        if board_ocupancy != self.getOcupancy() {
            self.half_move_count = 0;
        }

        return board_ocupancy != self.getOcupancy();
    }

    fn try_castle(&mut self, movve: LegalMove, playing_as: Turn) {
        match playing_as {
            Turn::White => match movve.castle {
                CastleOptions::KingSide => {
                    if self.can_castle_king_side(playing_as) {
                        self.w_king = 0x2;
                        self.w_rooks = self.w_rooks & !0x1 | 0x4;
                        self.has_w_king_side_castle = true;
                        self.w_king_has_moved = true;
                    }
                }
                CastleOptions::QueenSide => {
                    if self.can_castle_queen_side(playing_as) {
                        self.w_king = 0x20;
                        self.w_rooks = self.w_rooks & !0x80 | 0x10;
                        self.has_w_queen_side_castle = true;
                        self.w_king_has_moved = true;
                    }
                }
                CastleOptions::None => {}
            },
            Turn::Black => match movve.castle {
                CastleOptions::KingSide => {
                    if self.can_castle_king_side(playing_as) {
                        self.b_king = 0x200000000000000;
                        self.b_rooks = self.b_rooks & !0x100000000000000 | 0x400000000000000;
                        self.has_b_king_side_castle = true;
                        self.b_king_has_moved = true;
                    }
                }
                CastleOptions::QueenSide => {
                    if self.can_castle_queen_side(playing_as) {
                        self.b_king = 0x2000000000000000;
                        self.b_rooks = self.b_rooks & !0x8000000000000000 | 0x1000000000000000;
                        self.has_b_queen_side_castle = true;
                        self.b_king_has_moved = true;
                    }
                }
                CastleOptions::None => {}
            },
        }
    }

    fn get_piece_bitboard_from_position(movve: Position) -> u64 {
        let mut piece_bitboard: u64 = (1 as u64) << movve.colum;
        if movve.row != 0 {
            piece_bitboard = piece_bitboard << movve.row * 8
        }

        return piece_bitboard;
    }

    fn pawn_do_promote(&mut self, destin_board: u64, color: Turn) {
        if destin_board > 0x80000000000000 && matches!(color, Turn::White)
            || destin_board < 0x80000000000100 && matches!(color, Turn::Black)
        {
            match color {
                Turn::White => {
                    self.w_pawns = self.w_pawns & !destin_board;
                    self.w_queen |= destin_board
                }

                Turn::Black => {
                    self.b_pawns = self.b_pawns & !destin_board;
                    self.b_queen |= destin_board
                }
            }
        }
    }

    fn do_try_move(&mut self, piece_board: u64, destin_board: u64, piece_type: PieceType) {
        match piece_type {
            PieceType::WhiteQueen => {
                self.try_take(destin_board);
                self.w_queen = self.w_queen & !piece_board;
                self.w_queen = self.w_queen | destin_board;
            }
            PieceType::WhiteKing => {
                self.try_take(destin_board);
                self.w_king = self.w_king & !piece_board;
                self.w_king = self.w_king | destin_board;

                self.w_king_has_moved = true;
            }
            PieceType::WhiteBishop => {
                self.try_take(destin_board);
                self.w_bishops = self.w_bishops & !piece_board;
                self.w_bishops = self.w_bishops | destin_board;
            }
            PieceType::WhiteRook => {
                if piece_board == 0x1 {
                    self.has_w_king_side_castle = true;
                }
                if piece_board == 0x80 {
                    self.has_w_queen_side_castle = true;
                }
                self.try_take(destin_board);

                self.w_rooks = self.w_rooks & !piece_board;
                self.w_rooks = self.w_rooks | destin_board;
            }
            PieceType::WhiteKnight => {
                self.try_take(destin_board);
                self.w_knights = self.w_knights & !piece_board;
                self.w_knights = self.w_knights | destin_board;
            }
            PieceType::WhitePawn => {
                self.try_take(destin_board);
                self.w_pawns = self.w_pawns & !piece_board;
                self.w_pawns = self.w_pawns | destin_board;
                self.pawn_do_promote(destin_board, Turn::White);
            }
            PieceType::WhitePawnEnPassant => {
                self.try_take(destin_board >> 8);
                self.w_pawns = self.w_pawns & !piece_board;
                self.w_pawns = self.w_pawns | destin_board;
            }
            PieceType::BlackQueen => {
                self.try_take(destin_board);
                self.b_queen = self.b_queen & !piece_board;
                self.b_queen = self.b_queen | destin_board;
            }
            PieceType::BlackKing => {
                self.try_take(destin_board);
                self.b_king = self.b_king & !piece_board;
                self.b_king = self.b_king | destin_board;

                self.b_king_has_moved = true;
            }
            PieceType::BlackBishop => {
                self.try_take(destin_board);
                self.b_bishops = self.b_bishops & !piece_board;
                self.b_bishops = self.b_bishops | destin_board;
            }
            PieceType::BlackRook => {
                if piece_board == 0x100000000000000 {
                    self.has_b_king_side_castle = true;
                }
                if piece_board == 0x8000000000000000 {
                    self.has_b_queen_side_castle = true;
                }
                self.try_take(destin_board);
                self.b_rooks = self.b_rooks & !piece_board;
                self.b_rooks = self.b_rooks | destin_board;
            }
            PieceType::BlackKnight => {
                self.try_take(destin_board);
                self.b_knights = self.b_knights & !piece_board;
                self.b_knights = self.b_knights | destin_board;
            }
            PieceType::BlackPawn => {
                self.try_take(destin_board);
                self.b_pawns = self.b_pawns & !piece_board;
                self.b_pawns = self.b_pawns | destin_board;
                self.pawn_do_promote(destin_board, Turn::White);
            }
            PieceType::BlackPawnEnPassant => {
                self.try_take(destin_board << 8);
                self.b_pawns = self.b_pawns & !piece_board;
                self.b_pawns = self.b_pawns | destin_board;
            }
        }
    }

    pub fn do_move(&mut self, movve: LegalMove, playing_as: Turn) {
        match movve.castle {
            CastleOptions::KingSide => self.try_castle(movve, playing_as),
            CastleOptions::QueenSide => self.try_castle(movve, playing_as),
            CastleOptions::None => {
                //Move is not castle
                let piece_bitboard: u64 = Board::get_piece_bitboard_from_position(movve.from);
                let destin_bitboard: u64 = Board::get_piece_bitboard_from_position(movve.to);

                match playing_as {
                    Turn::Black => {
                        self.b_en_passant = 0;
                        //Piece is Rook
                        if self.b_rooks & piece_bitboard > 0 {
                            self.do_try_move(piece_bitboard, destin_bitboard, PieceType::BlackRook);
                        }

                        //Piece is Knight
                        if self.b_knights & piece_bitboard > 0 {
                            self.do_try_move(
                                piece_bitboard,
                                destin_bitboard,
                                PieceType::BlackKnight,
                            );
                        }

                        //Piece is Bishop
                        if self.b_bishops & piece_bitboard > 0 {
                            self.do_try_move(
                                piece_bitboard,
                                destin_bitboard,
                                PieceType::BlackBishop,
                            );
                        }

                        //Piece is King
                        if self.b_king & piece_bitboard > 0 {
                            self.do_try_move(piece_bitboard, destin_bitboard, PieceType::BlackKing);
                        }
                        //Piece is Queen
                        if self.b_queen & piece_bitboard > 0 {
                            self.do_try_move(
                                piece_bitboard,
                                destin_bitboard,
                                PieceType::BlackQueen,
                            );
                        }
                        //Piece is Pawn
                        if self.b_pawns & piece_bitboard > 0 {
                            if piece_bitboard >> 16 == destin_bitboard {
                                self.b_en_passant = destin_bitboard;
                            }
                            self.do_try_move(piece_bitboard, destin_bitboard, PieceType::BlackPawn);
                        }
                    }
                    Turn::White => {
                        self.w_en_passant = 0;
                        //Piece is Rook
                        if self.w_rooks & piece_bitboard > 0 {
                            self.do_try_move(piece_bitboard, destin_bitboard, PieceType::WhiteRook);
                        }

                        //Piece is Knight
                        if self.w_knights & piece_bitboard > 0 {
                            self.do_try_move(
                                piece_bitboard,
                                destin_bitboard,
                                PieceType::WhiteKnight,
                            );
                        }

                        //Piece is Bishop
                        if self.w_bishops & piece_bitboard > 0 {
                            self.do_try_move(
                                piece_bitboard,
                                destin_bitboard,
                                PieceType::WhiteBishop,
                            );
                        }

                        //Piece is King
                        if self.w_king & piece_bitboard > 0 {
                            self.do_try_move(piece_bitboard, destin_bitboard, PieceType::WhiteKing);
                        }
                        //Piece is Queen
                        if self.w_queen & piece_bitboard > 0 {
                            self.do_try_move(
                                piece_bitboard,
                                destin_bitboard,
                                PieceType::WhiteQueen,
                            );
                        }
                        //Piece is Pawn
                        if self.w_pawns & piece_bitboard > 0 {
                            if piece_bitboard << 16 == destin_bitboard {
                                self.w_en_passant = destin_bitboard;
                            }
                            self.do_try_move(piece_bitboard, destin_bitboard, PieceType::WhitePawn);
                        }
                    }
                }
            }
        }
    }

    pub fn can_castle_king_side(&self, playing_as: Turn) -> bool {
        match playing_as {
            Turn::White => {
                let attacts = Movement::extract_all_attacks_for_color(*self, Turn::Black);

                return !(self.has_w_king_side_castle || self.w_king_has_moved)
                    && self.w_king & 0x8 > 0
                    && self.w_rooks & 0x1 > 0
                    && self.getOcupancy() & 0x6 == 0
                    && attacts & 0xe == 0;
            }
            Turn::Black => {
                let attacts = Movement::extract_all_attacks_for_color(*self, Turn::White);

                return !(self.has_b_king_side_castle || self.b_king_has_moved)
                    && self.b_king & 0x800000000000000 > 0
                    && self.b_rooks & 0x100000000000000 > 0
                    && self.getOcupancy() & 0x600000000000000 == 0
                    && attacts & 0xe00000000000000 == 0;
            }
        }
    }

    pub fn can_castle_queen_side(&self, playing_as: Turn) -> bool {
        match playing_as {
            Turn::White => {
                let attacts = Movement::extract_all_attacks_for_color(*self, Turn::Black);

                return !(self.has_w_queen_side_castle || self.w_king_has_moved)
                    && self.w_king & 0x8 > 0
                    && self.w_rooks & 0x80 > 0
                    && self.getOcupancy() & 0x70 == 0
                    && attacts & 0x78 == 0;
            }
            Turn::Black => {
                let attacts = Movement::extract_all_attacks_for_color(*self, Turn::White);

                return !(self.has_b_queen_side_castle || self.b_king_has_moved)
                    && self.b_king & 0x800000000000000 > 0
                    && self.b_rooks & 0x8000000000000000 > 0
                    && self.getOcupancy() & 0x7000000000000000 == 0
                    && attacts & 0x7800000000000000 == 0;
            }
        }
    }

    pub fn print_board_self(&self, board_name: &str) {
        Printer::print_board(board_name, *self);
    }

    pub fn get_moves(&self, color: Turn) -> Vec<LegalMove> {
        let mut board_copy = self.clone();
        let mut legal_moves = board_copy.extract_all_legal_moves_for_color(color);
        legal_moves.dedup();
        return legal_moves;
    }

    fn is_move_legal(self, legal_move: LegalMove, playing_as: Turn) -> bool {
        let mut board_check = self;
        board_check.do_move(legal_move, playing_as);
        match playing_as {
            Turn::Black => self.b_king & self.white_attacks == 0,
            Turn::White => self.w_king & self.black_attacks == 0,
        }
    }

    pub fn legalize_moves(&mut self, moves: Vec<LegalMove>, playing_as: Turn) -> Vec<LegalMove> {
        let mut legal_moves: Vec<LegalMove> = Vec::new();
        let board_clone = self.clone();
        match playing_as {
            Turn::White => {
                self.white_attacks = Movement::get_color_attacks(Turn::White, board_clone)
            }
            Turn::Black => {
                self.black_attacks = Movement::get_color_attacks(Turn::Black, board_clone)
            }
        }
        for movve in moves {
            if self.is_move_legal(movve, playing_as) {
                legal_moves.push(movve);
            }
        }

        return legal_moves;
    }

    pub fn extract_all_legal_moves_for_color(&mut self, color: Turn) -> Vec<LegalMove> {
        let mut moves: Vec<LegalMove> = Vec::new();
        let mut castling: Vec<LegalMove> = Vec::new();

        self.white_attacks = Movement::get_color_attacks(Turn::White, *self);
        self.black_attacks = Movement::get_color_attacks(Turn::Black, *self);

        if self.can_castle_king_side(color) {
            castling.push(LegalMove::king_side_castle(color));
        }

        if self.can_castle_queen_side(color) {
            castling.push(LegalMove::queen_side_castle(color));
        }

        moves.extend(castling);
        match color {
            Turn::White => {
                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.w_pawns,
                    color,
                    PieceType::WhitePawn,
                ));
                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.w_pawns,
                    color,
                    PieceType::WhitePawnEnPassant,
                ));

                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.w_rooks,
                    color,
                    PieceType::WhiteRook,
                ));

                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.w_bishops,
                    color,
                    PieceType::WhiteBishop,
                ));
                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.w_knights,
                    color,
                    PieceType::WhiteKnight,
                ));
                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.w_queen,
                    color,
                    PieceType::WhiteQueen,
                ));

                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.w_king,
                    color,
                    PieceType::WhiteKing,
                ));
            }
            Turn::Black => {
                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.b_pawns,
                    color,
                    PieceType::BlackPawn,
                ));

                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.b_pawns,
                    color,
                    PieceType::BlackPawnEnPassant,
                ));

                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.b_rooks,
                    color,
                    PieceType::BlackRook,
                ));

                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.b_bishops,
                    color,
                    PieceType::BlackBishop,
                ));
                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.b_knights,
                    color,
                    PieceType::BlackKnight,
                ));
                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.b_queen,
                    color,
                    PieceType::BlackQueen,
                ));

                moves.extend(self.extract_pieces_moves_from_bitboard(
                    self.b_king,
                    color,
                    PieceType::BlackKing,
                ));
            }
        }
        return moves;
    }

    pub fn extract_pieces_moves_from_bitboard(
        self,
        mut pieces_bits: u64,
        color: Turn,
        piece_type: PieceType,
    ) -> Vec<LegalMove> {
        if pieces_bits == 0 {
            return Vec::new();
        } else {
            let mut legal_moves: Vec<LegalMove> = Vec::new();

            while pieces_bits > 0 {
                let piece_pos = Movement::msb_pos(pieces_bits);
                let piece_bits = (1 as u64) << piece_pos;

                let result = self.extract_piece_moves_from_bitboard(piece_bits, color, piece_type);
                for elem in result {
                    legal_moves.push(elem)
                }
                pieces_bits = pieces_bits & !piece_bits;
            }

            return legal_moves;
        }
    }

    fn extract_piece_moves_from_bitboard(
        self,
        piece_bits: u64,
        color: Turn,
        piece_type: PieceType,
    ) -> Vec<LegalMove> {
        match piece_type {
            PieceType::WhiteBishop => {
                return self.get_bishop_moves(piece_bits, color);
            }
            PieceType::WhiteKnight => return self.get_knigth_moves(piece_bits, color),
            PieceType::WhiteRook => return self.get_rook_moves(piece_bits, color),
            PieceType::WhiteQueen => return self.get_queen_moves(piece_bits, color),
            PieceType::WhitePawn => return self.get_pawn_moves(piece_bits, color),
            PieceType::WhitePawnEnPassant => {
                return self.get_pawn_moves_enpassant(piece_bits, self.b_en_passant, color)
            }
            PieceType::WhiteKing => return self.get_king_moves(piece_bits, color),
            PieceType::BlackBishop => return self.get_bishop_moves(piece_bits, color),
            PieceType::BlackKnight => return self.get_knigth_moves(piece_bits, color),
            PieceType::BlackRook => return self.get_rook_moves(piece_bits, color),
            PieceType::BlackQueen => return self.get_queen_moves(piece_bits, color),
            PieceType::BlackPawn => return self.get_pawn_moves(piece_bits, color),
            PieceType::BlackPawnEnPassant => {
                return self.get_pawn_moves_enpassant(piece_bits, self.w_en_passant, color)
            }
            PieceType::BlackKing => return self.get_king_moves(piece_bits, color),
        }
    }

    fn translate_piece_moves_from_bitboard(
        self,
        piece_bits: u64,
        mut piece_move_bits: u64,
        color: Turn,
        board: Board,
    ) -> Vec<LegalMove> {
        let piece_index = Movement::get_piece_index(piece_bits);
        let from_column: i8 = piece_index % 8;
        let from_row: i8 = piece_index / 8;

        let mut legal_moves: Vec<LegalMove> = Vec::new();

        while piece_move_bits > 0 {
            let piece_pos = Movement::msb_pos(piece_move_bits);
            let piece_bits = (1 as u64) << piece_pos;

            let to_column = piece_pos % 8;
            let to_row = piece_pos / 8;

            let lega_move = LegalMove::new(from_row, from_column, to_row, to_column);

            if self.is_move_legal(lega_move, color) {
                legal_moves.push(lega_move.to_owned())
            }

            piece_move_bits = piece_move_bits & !piece_bits;
        }

        return legal_moves;
    }

    fn get_knigth_moves(self, knight_bits: u64, color: Turn) -> Vec<LegalMove> {
        let bitboard_moves = Knight::get_moves(knight_bits, color, self);

        return self.translate_piece_moves_from_bitboard(knight_bits, bitboard_moves, color, self);
    }

    fn get_pawn_moves(self, pawn_bits: u64, color: Turn) -> Vec<LegalMove> {
        let bitboard_moves = Pawn::get_moves(pawn_bits, color, self);
        return self.translate_piece_moves_from_bitboard(pawn_bits, bitboard_moves, color, self);
    }

    fn is_enpassant(piece_bits: u64, en_passant_pos: u64) -> bool {
        return (piece_bits << 1 & en_passant_pos != 0) || (piece_bits >> 1 & en_passant_pos != 0);
    }

    fn get_pawn_moves_enpassant(
        self,
        pawn_bits: u64,
        enpassant_bits: u64,
        color: Turn,
    ) -> Vec<LegalMove> {
        let bitboard_moves = Pawn::get_moves_enpassant(pawn_bits, enpassant_bits, color);
        return self.translate_piece_moves_from_bitboard(pawn_bits, bitboard_moves, color, self);
    }

    fn get_rook_moves(self, rook_bits: u64, color: Turn) -> Vec<LegalMove> {
        let bitboard_moves = Rook::get_moves(rook_bits, color, self);

        return self.translate_piece_moves_from_bitboard(rook_bits, bitboard_moves, color, self);
    }

    fn get_bishop_moves(self, bishop_bits: u64, color: Turn) -> Vec<LegalMove> {
        let bitboard_moves = Bishop::get_moves(bishop_bits, color, self);

        return self.translate_piece_moves_from_bitboard(bishop_bits, bitboard_moves, color, self);
    }

    fn get_queen_moves(self, queen_bits: u64, color: Turn) -> Vec<LegalMove> {
        let bitboard_moves = Queen::get_moves(queen_bits, color, self);

        return self.translate_piece_moves_from_bitboard(queen_bits, bitboard_moves, color, self);
    }

    fn get_king_moves(self, king_bits: u64, color: Turn) -> Vec<LegalMove> {
        let bitboard_moves = King::get_moves(king_bits, color, self);

        return self.translate_piece_moves_from_bitboard(king_bits, bitboard_moves, color, self);
    }

    //BITBoard calculations
    pub fn pub_get_knigth_moves_bitboard(self, knight_bits: u64, color: Turn) -> u64 {
        return Knight::get_moves(knight_bits, color, self);
    }

    pub fn pub_get_pawn_moves_bitboard(self, pawn_bits: u64, color: Turn) -> u64 {
        return Pawn::get_moves(pawn_bits, color, self);
    }

    pub fn pub_is_enpassant_bitboard(self, piece_bits: u64, en_passant_pos: u64) -> bool {
        return (piece_bits << 1 & en_passant_pos != 0) || (piece_bits >> 1 & en_passant_pos != 0);
    }

    pub fn pub_get_pawn_moves_enpassant_bitboard(
        self,
        pawn_bits: u64,
        enpassant_bits: u64,
        color: Turn,
    ) -> u64 {
        return Pawn::get_moves_enpassant(pawn_bits, enpassant_bits, color);
    }

    pub fn pub_get_rook_moves_bitboard(self, rook_bits: u64, color: Turn) -> u64 {
        return Rook::get_moves(rook_bits, color, self);
    }

    pub fn pub_get_bishop_moves_bitboard(self, bishop_bits: u64, color: Turn) -> u64 {
        return Bishop::get_moves(bishop_bits, color, self);
    }

    pub fn pub_get_queen_moves_bitboard(self, queen_bits: u64, color: Turn) -> u64 {
        return Queen::get_moves(queen_bits, color, self);
    }

    pub fn pub_get_king_moves_bitboard(self, king_bits: u64, color: Turn) -> u64 {
        return King::get_moves(king_bits, color, self);
    }
}
