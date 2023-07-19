use crate::{board::board::Board, engine::movement::movement::Movement};

#[derive(Clone, Copy)]
pub enum PlayingAs {
    White,
    Black,
}

struct Player {
    color: PlayingAs,
    has_castled: bool,
    first_rank_two_move_forward: Option<UnPassant>,
}

impl Player {
    pub fn new(playing_as: PlayingAs) -> Player {
        return Player {
            color: playing_as,
            has_castled: false,
            first_rank_two_move_forward: None,
        };
    }

    pub fn register_un_passant(&mut self, column: i8, row: i8) {
        self.first_rank_two_move_forward = Some(UnPassant { column, row })
    }

    pub fn un_register_un_passant(&mut self, column: i8, row: i8) {
        self.first_rank_two_move_forward = None
    }
}
struct UnPassant {
    column: i8,
    row: i8,
}

pub struct Analyzer {
    pub board: Board,
    player_white: Player,
    player_black: Player,
    pub playing_as: PlayingAs,
    pub movemnt: Movement,
}

impl Analyzer {
    pub fn new(playing_as: PlayingAs) -> Analyzer {
        return Analyzer {
            board: Board::new(),
            playing_as: playing_as,
            movemnt: Movement::new(),
            player_white: Player::new(PlayingAs::White),
            player_black: Player::new(PlayingAs::Black),
        };
    }

    pub fn new_with_board(playing_as: PlayingAs, board: Board) -> Analyzer {
        return Analyzer {
            board: board,
            playing_as: playing_as,
            movemnt: Movement::new(),
            player_white: Player::new(PlayingAs::White),
            player_black: Player::new(PlayingAs::Black),
        };
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
