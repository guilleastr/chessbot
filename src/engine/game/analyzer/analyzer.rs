use crate::engine::{
    board::{
        board::Board,
        position::position::{Move, Position},
    },
    game::game::Player,
    movement::movement::Movement,
};

#[derive(Clone, Copy)]
pub enum PlayingAs {
    White,
    Black,
}

struct UnPassant {
    column: i8,
    row: i8,
}

pub struct Analyzer {}

impl Player for Analyzer {
    fn do_move() -> Move {
        println!("Analyzer does move");
        return Move {
            from: Position { colum: 3, row: 4 },
            to: Position { colum: 2, row: 4 },
        };
    }
}

impl Analyzer {
    pub fn new(playing_as: PlayingAs) -> Analyzer {
        return Analyzer {};
    }

    pub fn get_plain_score(playing_as: PlayingAs, board: Board) -> f64 {
        let mut pawns_score: f64 = 0.0;
        let mut knights_score: f64 = 0.0;
        let mut bishops_score: f64 = 0.0;
        let mut rooks_score: f64 = 0.0;
        let mut queen_score: f64 = 0.0;
        if matches!(playing_as, PlayingAs::Black) {
            pawns_score = f64::from(Movement::pieces_count(board.b_pawns));
            knights_score = f64::from(Movement::pieces_count(board.b_knights));
            bishops_score = f64::from(Movement::pieces_count(board.b_bishops));
            rooks_score = f64::from(Movement::pieces_count(board.b_rooks));
            queen_score = f64::from(Movement::pieces_count(board.b_queen));
        }

        if matches!(playing_as, PlayingAs::White) {
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
}
