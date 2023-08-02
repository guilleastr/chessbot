use std::process::exit;

use crate::engine::{
    board::{
        board::{Board, Turn},
        position::position::LegalMove,
    },
    movement::movement::Movement,
};

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
    fn do_move(board: Board, color: Turn) -> LegalMove;
}

pub struct Game {
    white: Players,
    black: Players,
    board: Board,
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
        };
    }
    pub fn setup_from_fenn(
        board_state: &'static str,
        white: PlayerTypes,
        black: PlayerTypes,
    ) -> Game {
        let white_player: Players;
        let black_player: Players;

        if matches!(white, PlayerTypes::AI) {
            white_player = Players::Robot;
        } else {
            white_player = Players::Player;
        }

        if matches!(black, PlayerTypes::AI) {
            black_player = Players::Robot;
        } else {
            black_player = Players::Player;
        }

        return Game {
            white: white_player,
            black: black_player,
            board: Board::new_from_fenn_notation(board_state),
        };
    }

    pub fn take_turn(&mut self) {
        if self.is_checkmate(self.board.get_turn()) {
            print!("CheckMate!");
            exit(0);
        }
        self.board.print_board_self("Board");
        match self.board.get_turn() {
            Turn::White => {
                println!("White turn");
                self.take_turn_white();
                self.board.set_turn(Turn::Black);
            }
            Turn::Black => {
                println!("Black turn");
                self.take_turn_black();
                self.board.set_turn(Turn::White);
            }
        }
    }

    pub fn is_checkmate(&self, color: Turn) -> bool {
        match self.board.get_turn() {
            Turn::White => Movement::check_for_checkmate(color, self.get_board()),
            Turn::Black => Movement::check_for_checkmate(color, self.get_board()),
        }
    }

    fn take_turn_white(&mut self) {
        match self.white {
            Players::Robot => {
                self.board
                    .do_move(Analyzer::do_move(self.board, Turn::White), Turn::White);
            }
            Players::Player => self
                .board
                .do_move(Human::do_move(self.board, Turn::White), Turn::White),
        };
    }

    fn take_turn_black(&mut self) {
        match self.black {
            Players::Robot => self
                .board
                .do_move(Analyzer::do_move(self.board, Turn::Black), Turn::Black),
            Players::Player => self
                .board
                .do_move(Human::do_move(self.board, Turn::Black), Turn::Black),
        };
    }

    pub fn get_board(&self) -> Board {
        self.board
    }

    pub fn get_board_copy(&self) -> Board {
        self.board.clone()
    }
}
