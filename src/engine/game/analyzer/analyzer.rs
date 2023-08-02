use std::f64::{INFINITY, NEG_INFINITY};

use crate::engine::{
    board::{
        board::{Board, Turn},
        position::position::{CastleOptions, LegalMove, Position},
    },
    game::game::Player,
    movement::movement::Movement,
};

struct EvalMove {
    legal: LegalMove,
    eval: f64,
}

pub struct Analyzer {}

const SEARCH_DEPTH: i8 = 6;

impl Player for Analyzer {
    fn do_move(board: Board, color: Turn) -> LegalMove {
        println!("Analyzer does move");

        let legal = Analyzer::minimax(
            board,
            color,
            color,
            SEARCH_DEPTH,
            f64::NEG_INFINITY,
            f64::INFINITY,
        );

        return legal;
    }
}

impl Analyzer {
    fn analyze_state(board: Board) -> f64 {
        let activity_score = Analyzer::get_activity_score(board);
        let plain_score = Analyzer::get_plain_score(board);

        return plain_score + activity_score;
    }

    pub fn minimax(
        board: Board,
        playing_as: Turn,
        player_color: Turn,
        depth: i8,
        alpha: f64,
        betha: f64,
    ) -> LegalMove {
        println!(
            "Activity: {} Plain:{}",
            Analyzer::get_activity_score(board),
            Analyzer::get_plain_score(board)
        );
        let legal_moves = board.get_moves(playing_as);
        let mut eval_move: LegalMove = LegalMove {
            from: (Position { colum: 0, row: 0 }),
            to: Position { colum: 0, row: 0 },
            castle: CastleOptions::None,
        };
        let mut max_value = f64::NEG_INFINITY;
        for legal_move in legal_moves.to_owned() {
            println!("{}", legal_move);
            let mut analize_board = board;
            analize_board.do_move(legal_move, playing_as);
            let minimax_score = Analyzer::r_minimax(
                analize_board,
                Movement::get_oposite_color(playing_as),
                player_color,
                depth - 1,
                alpha,
                betha,
            );

            if (minimax_score > max_value) {
                max_value = minimax_score;
                eval_move = legal_move;
            }
        }

        return eval_move;
    }

    fn r_minimax(
        board: Board,
        playing_As: Turn,
        original_turn: Turn,
        depth: i8,
        mut alpha: f64,
        mut betha: f64,
    ) -> f64 {
        // println!("Alpha: {} Beetha: {}", alpha, betha);
        if depth == 0 {
            return Analyzer::analyze_state(board);
        }
        let legal_moves = board.get_moves(playing_As);

        if legal_moves.len() == 0 {
            if Movement::check_for_checkmate(Turn::White, board) {
                return INFINITY;
            };
            if Movement::check_for_checkmate(Turn::White, board) {
                return NEG_INFINITY;
            };
        }

        if playing_As == original_turn {
            let mut max_value = f64::NEG_INFINITY;
            for legal_move in legal_moves.to_owned() {
                let mut analize_board = board;
                analize_board.do_move(legal_move, playing_As);
                let eval = Analyzer::r_minimax(
                    analize_board,
                    Movement::get_oposite_color(playing_As),
                    original_turn,
                    depth - 1,
                    alpha,
                    betha,
                );
                max_value = max_value.max(eval.to_owned());
                alpha = alpha.max(eval.to_owned());

                if betha <= alpha {
                    break;
                }
            }

            return max_value;
        } else {
            let mut min_value = f64::INFINITY;
            for legal_move in legal_moves {
                let mut analize_board = board;
                analize_board.do_move(legal_move, playing_As);
                let eval = Analyzer::r_minimax(
                    analize_board,
                    Movement::get_oposite_color(playing_As),
                    original_turn,
                    depth - 1,
                    alpha,
                    betha,
                );
                min_value = min_value.min(eval.to_owned());
                betha = betha.min(eval);

                if betha <= alpha || eval <= betha {
                    break;
                }
            }

            return min_value;
        }
    }

    pub fn get_plain_score(board: Board) -> f64 {
        let mut pawns_score_white: f64 = 0.0;
        let mut knights_score_white: f64 = 0.0;
        let mut bishops_score_white: f64 = 0.0;
        let mut rooks_score_white: f64 = 0.0;
        let mut queen_score_white: f64 = 0.0;

        let mut pawns_score_black: f64 = 0.0;
        let mut knights_score_black: f64 = 0.0;
        let mut bishops_score_black: f64 = 0.0;
        let mut rooks_score_black: f64 = 0.0;
        let mut queen_score_black: f64 = 0.0;

        pawns_score_black = f64::from(Movement::pieces_count(board.b_pawns));
        knights_score_black = f64::from(Movement::pieces_count(board.b_knights));
        bishops_score_black = f64::from(Movement::pieces_count(board.b_bishops));
        rooks_score_black = f64::from(Movement::pieces_count(board.b_rooks));
        queen_score_black = f64::from(Movement::pieces_count(board.b_queen));

        pawns_score_white = f64::from(Movement::pieces_count(board.w_pawns));
        knights_score_white = f64::from(Movement::pieces_count(board.w_knights));
        bishops_score_white = f64::from(Movement::pieces_count(board.w_bishops));
        rooks_score_white = f64::from(Movement::pieces_count(board.w_rooks));
        queen_score_white = f64::from(Movement::pieces_count(board.w_queen));

        return pawns_score_white
            + knights_score_white * 3.0
            + bishops_score_white * 3.5
            + rooks_score_white * 5.0
            + queen_score_white * 10.0
            - (pawns_score_black
                + knights_score_black * 3.0
                + bishops_score_black * 3.5
                + rooks_score_black * 5.0
                + queen_score_black * 10.0);
    }

    pub fn get_activity_score(board: Board) -> f64 {
        let mut pawns_score_black: f64 = 0.0;
        let mut knights_score_black: f64 = 0.0;
        let mut bishops_score_black: f64 = 0.0;
        let mut rooks_score_black: f64 = 0.0;
        let mut queen_score_black: f64 = 0.0;

        let mut pawns_score_white: f64 = 0.0;
        let mut knights_score_white: f64 = 0.0;
        let mut bishops_score_white: f64 = 0.0;
        let mut rooks_score_white: f64 = 0.0;
        let mut queen_score_white: f64 = 0.0;

        pawns_score_black = f64::from(Movement::pieces_count(
            board.pub_get_pawn_moves_bitboard(board.b_pawns, Turn::Black),
        ));

        knights_score_black = f64::from(Movement::pieces_count(
            board.pub_get_knigth_moves_bitboard(board.b_knights, Turn::Black),
        ));

        bishops_score_black = f64::from(Movement::pieces_count(
            board.pub_get_bishop_moves_bitboard(board.b_bishops, Turn::Black),
        ));

        rooks_score_black = f64::from(Movement::pieces_count(
            board.pub_get_rook_moves_bitboard(board.b_rooks, Turn::Black),
        ));

        queen_score_black = f64::from(Movement::pieces_count(
            board.pub_get_queen_moves_bitboard(board.b_queen, Turn::Black),
        ));

        pawns_score_white = f64::from(Movement::pieces_count(
            board.pub_get_pawn_moves_bitboard(board.w_pawns, Turn::White),
        ));

        knights_score_white = f64::from(Movement::pieces_count(
            board.pub_get_knigth_moves_bitboard(board.w_knights, Turn::White),
        ));

        bishops_score_white = f64::from(Movement::pieces_count(
            board.pub_get_bishop_moves_bitboard(board.w_bishops, Turn::White),
        ));

        rooks_score_white = f64::from(Movement::pieces_count(
            board.pub_get_rook_moves_bitboard(board.w_rooks, Turn::White),
        ));

        queen_score_white = f64::from(Movement::pieces_count(
            board.pub_get_queen_moves_bitboard(board.w_queen, Turn::White),
        ));

        return (pawns_score_white
            + knights_score_white
            + bishops_score_white
            + rooks_score_white
            + queen_score_white
            - (pawns_score_black
                + knights_score_black
                + bishops_score_black
                + rooks_score_black
                + queen_score_black))
            / 50.0;
    }

    pub fn get_attacked_score(turn: Turn, board: Board) -> f64 {
        let mut pawns_score: f64 = 0.0;
        let mut knights_score: f64 = 0.0;
        let mut bishops_score: f64 = 0.0;
        let mut rooks_score: f64 = 0.0;
        let mut queen_score: f64 = 0.0;
        if matches!(turn, Turn::Black) {
            pawns_score = f64::from(Movement::pieces_count(
                board.pub_get_pawn_moves_bitboard(board.b_pawns, Turn::Black),
            ));
            knights_score = f64::from(Movement::pieces_count(
                board.pub_get_knigth_moves_bitboard(board.b_knights, Turn::Black),
            ));
            bishops_score = f64::from(Movement::pieces_count(
                board.pub_get_bishop_moves_bitboard(board.b_bishops, Turn::Black),
            ));
            rooks_score = f64::from(Movement::pieces_count(
                board.pub_get_rook_moves_bitboard(board.b_rooks, Turn::Black),
            ));
            queen_score = f64::from(Movement::pieces_count(
                board.pub_get_queen_moves_bitboard(board.b_queen, Turn::Black),
            ));
        }

        if matches!(turn, Turn::White) {
            pawns_score = f64::from(Movement::pieces_count(
                board.pub_get_pawn_moves_bitboard(board.w_pawns, Turn::White),
            ));
            knights_score = f64::from(Movement::pieces_count(
                board.pub_get_knigth_moves_bitboard(board.w_knights, Turn::White),
            ));
            bishops_score = f64::from(Movement::pieces_count(
                board.pub_get_bishop_moves_bitboard(board.w_bishops, Turn::White),
            ));
            rooks_score = f64::from(Movement::pieces_count(
                board.pub_get_rook_moves_bitboard(board.w_rooks, Turn::White),
            ));
            queen_score = f64::from(Movement::pieces_count(
                board.pub_get_queen_moves_bitboard(board.w_queen, Turn::White),
            ));
        }
        return pawns_score + knights_score + bishops_score + rooks_score + queen_score;
    }
}
