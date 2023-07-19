use crate::engine::board::{board::Board, position::position::Move};

use super::{analyzer::analyzer::Analyzer, human::human::Human};

pub enum PlayerTypes {
    AI,
    HUMAN,
}

enum Players {
    Robot,
    Player,
}

pub trait Player {
    fn do_move() -> Move;
}

pub enum Turn {
    White,
    Black,
}

pub struct Game {
    white: Players,
    black: Players,
    board: Board,
    turn: Turn,
}

impl Game {
    pub fn setup(white: PlayerTypes, black: PlayerTypes) -> Game {
        let white_player: Players;

        if matches!(white, PlayerTypes::AI) {
            white_player = Players::Robot;
        } else {
            white_player = Players::Player;
        }

        let black_player: Players;

        if matches!(black, PlayerTypes::AI) {
            black_player = Players::Robot;
        } else {
            black_player = Players::Player;
        }

        return Game {
            white: white_player,
            black: black_player,
            board: Board::new(),
            turn: Turn::White,
        };
    }

    pub fn take_turn(&mut self) {
        self.board.print_board_self("Board");
        match self.turn {
            Turn::White => {
                while !self.take_turn_white() {}
                self.turn = Turn::Black;
            }
            Turn::Black => {
                while !self.take_turn_black() {}
                self.turn = Turn::White;
            }
        }
    }

    fn take_turn_white(&mut self) -> bool {
        match self.white {
            Players::Robot => {
                return self.board.perform_move(
                    Analyzer::do_move(),
                    super::analyzer::analyzer::PlayingAs::White,
                )
            }
            Players::Player => {
                return self.board.perform_move(
                    Human::do_move(),
                    super::analyzer::analyzer::PlayingAs::White,
                )
            }
        };
    }
    fn take_turn_black(&mut self) -> bool {
        match self.black {
            Players::Robot => {
                return self.board.perform_move(
                    Analyzer::do_move(),
                    super::analyzer::analyzer::PlayingAs::Black,
                )
            }
            Players::Player => {
                return self.board.perform_move(
                    Human::do_move(),
                    super::analyzer::analyzer::PlayingAs::Black,
                )
            }
        };
    }
}
