use crate::engine::{
    board::{
        board::{Board, Turn},
        position::position::{CastleOptions, LegalMove, Position},
    },
    game::{analyzer::movement::movement_analyzer::AnalyzerMovement, game::Player},
    movement::movement::Movement,
};

pub struct Analyzer {}

impl Player for Analyzer {
    fn do_move(board: Board, color: Turn) -> LegalMove {
        println!("Analyzer does move");

        let legal = Analyzer::search(board, color, 0, 0.0);

        return legal;
    }
}
impl Analyzer {
    pub fn search(board: Board, color: Turn, depth: i8, max_reached_score: f64) -> LegalMove {
        let legal_moves = AnalyzerMovement::get_moves(board, color);

        let mut max_move: LegalMove = LegalMove {
            from: (Position { colum: 0, row: 0 }),
            to: Position { colum: 0, row: 0 },
            castle: CastleOptions::None,
        };
        let mut max_score: f64 = 0.0;

        for legal_move in legal_moves {
            let mut analize_board = board.to_owned();
            analize_board.perform_move(legal_move, color);

            let score = Analyzer::r_search(
                analize_board,
                Movement::get_oposite_color(color),
                5,
                max_reached_score,
            );
            if score > max_score {
                max_score = score;
                max_move = legal_move;
            }
        }

        return max_move;
    }

    fn r_search(board: Board, color: Turn, depth: i8, max_reached_score: f64) -> f64 {
        if depth == 0 {
            let activity_score = Analyzer::get_activity_score(color, board);
            let plain_score = Analyzer::get_plain_score(color, board);

            let score: f64 = activity_score + plain_score;
            return score;
        }
        let legal_moves = AnalyzerMovement::get_moves(board, color);

        let mut max_move: LegalMove = LegalMove {
            from: (Position { colum: 0, row: 0 }),
            to: Position { colum: 0, row: 0 },
            castle: CastleOptions::None,
        };
        let mut max_score: f64 = 0.0;

        for legal_move in legal_moves {
            let mut analize_board = board.to_owned();

            analize_board.perform_move(legal_move, color);
            let score = Analyzer::r_search(
                analize_board,
                Movement::get_oposite_color(color),
                depth - 1,
                max_reached_score,
            );
            if score > max_score {
                max_score = score;
                max_move = legal_move;
            }
        }

        return max_score;
    }

    pub fn get_plain_score(turn: Turn, board: Board) -> f64 {
        let mut pawns_score: f64 = 0.0;
        let mut knights_score: f64 = 0.0;
        let mut bishops_score: f64 = 0.0;
        let mut rooks_score: f64 = 0.0;
        let mut queen_score: f64 = 0.0;
        if matches!(turn, Turn::Black) {
            pawns_score = f64::from(Movement::pieces_count(board.b_pawns));
            knights_score = f64::from(Movement::pieces_count(board.b_knights));
            bishops_score = f64::from(Movement::pieces_count(board.b_bishops));
            rooks_score = f64::from(Movement::pieces_count(board.b_rooks));
            queen_score = f64::from(Movement::pieces_count(board.b_queen));
        }

        if matches!(turn, Turn::White) {
            pawns_score = f64::from(Movement::pieces_count(board.w_pawns));
            knights_score = f64::from(Movement::pieces_count(board.w_knights));
            bishops_score = f64::from(Movement::pieces_count(board.w_bishops));
            rooks_score = f64::from(Movement::pieces_count(board.w_rooks));
            queen_score = f64::from(Movement::pieces_count(board.w_queen));
        }
        return pawns_score
            + knights_score * 3.0
            + bishops_score * 3.5
            + rooks_score * 5.0
            + queen_score * 10.0;
    }

    pub fn get_activity_score(turn: Turn, board: Board) -> f64 {
        let mut pawns_score: f64 = 0.0;
        let mut knights_score: f64 = 0.0;
        let mut bishops_score: f64 = 0.0;
        let mut rooks_score: f64 = 0.0;
        let mut queen_score: f64 = 0.0;
        if matches!(turn, Turn::Black) {
            if board.b_pawns > 0 {
                pawns_score = f64::from(Movement::pieces_count(Movement::get_pawn_moves(
                    board.b_pawns,
                    Turn::Black,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                )));
            }
            if board.b_knights > 0 {
                knights_score = f64::from(Movement::pieces_count(Movement::get_knigth_moves(
                    board.b_knights,
                    Turn::Black,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                )));
            }
            if board.b_bishops > 0 {
                bishops_score = f64::from(Movement::pieces_count(Movement::get_bishop_moves(
                    board.b_bishops,
                    Turn::Black,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                )));
            }
            if board.b_rooks > 0 {
                rooks_score = f64::from(Movement::pieces_count(Movement::get_rook_moves(
                    board.b_rooks,
                    Turn::Black,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                )));
            }
            if board.b_queen > 0 {
                queen_score = f64::from(Movement::pieces_count(Movement::get_queen_moves(
                    board.b_queen,
                    Turn::Black,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                )));
            }
        }

        if matches!(turn, Turn::White) {
            if board.w_pawns > 0 {
                pawns_score = f64::from(Movement::pieces_count(Movement::get_pawn_moves(
                    board.w_pawns,
                    Turn::White,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                )));
            }
            if board.w_knights > 0 {
                knights_score = f64::from(Movement::pieces_count(Movement::get_knigth_moves(
                    board.w_knights,
                    Turn::White,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                )));
            }
            if board.w_bishops > 0 {
                bishops_score = f64::from(Movement::pieces_count(Movement::get_bishop_moves(
                    board.w_bishops,
                    Turn::White,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                )));
            }
            if board.w_rooks > 0 {
                rooks_score = f64::from(Movement::pieces_count(Movement::get_rook_moves(
                    board.w_rooks,
                    Turn::White,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                )));
            }
            if board.w_queen > 0 {
                queen_score = f64::from(Movement::pieces_count(Movement::get_queen_moves(
                    board.w_queen,
                    Turn::White,
                    board.getWhiteBitboard(),
                    board.getBlackBitboard(),
                )));
            }
        }
        return pawns_score + knights_score + bishops_score + rooks_score + queen_score;
    }

    pub fn get_attacked_score(turn: Turn, board: Board) -> f64 {
        let mut pawns_score: f64 = 0.0;
        let mut knights_score: f64 = 0.0;
        let mut bishops_score: f64 = 0.0;
        let mut rooks_score: f64 = 0.0;
        let mut queen_score: f64 = 0.0;
        if matches!(turn, Turn::Black) {
            pawns_score = f64::from(Movement::pieces_count(Movement::get_pawn_moves(
                board.b_pawns,
                Turn::Black,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            )));
            knights_score = f64::from(Movement::pieces_count(Movement::get_knigth_moves(
                board.b_knights,
                Turn::Black,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            )));
            bishops_score = f64::from(Movement::pieces_count(Movement::get_bishop_moves(
                board.b_bishops,
                Turn::Black,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            )));
            rooks_score = f64::from(Movement::pieces_count(Movement::get_rook_moves(
                board.b_rooks,
                Turn::Black,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            )));
            queen_score = f64::from(Movement::pieces_count(Movement::get_queen_moves(
                board.b_queen,
                Turn::Black,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            )));
        }

        if matches!(turn, Turn::White) {
            pawns_score = f64::from(Movement::pieces_count(Movement::get_pawn_moves(
                board.w_pawns,
                Turn::White,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            )));
            knights_score = f64::from(Movement::pieces_count(Movement::get_knigth_moves(
                board.w_knights,
                Turn::White,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            )));
            bishops_score = f64::from(Movement::pieces_count(Movement::get_bishop_moves(
                board.w_bishops,
                Turn::White,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            )));
            rooks_score = f64::from(Movement::pieces_count(Movement::get_rook_moves(
                board.w_rooks,
                Turn::White,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            )));
            queen_score = f64::from(Movement::pieces_count(Movement::get_queen_moves(
                board.w_queen,
                Turn::White,
                board.getWhiteBitboard(),
                board.getBlackBitboard(),
            )));
        }
        return pawns_score + knights_score + bishops_score + rooks_score + queen_score;
    }
}
