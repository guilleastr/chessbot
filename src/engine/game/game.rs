use crate::engine::{
    board::{
        board::{Board, Turn},
        position::position::Move,
    },
    movement::movement::Movement,
};

use super::{
    analyzer::analyzer::{Analyzer, PlayingAs},
    human::human::Human,
};

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
        self.board.print_board_self("Board");
        match self.board.get_turn() {
            Turn::White => {
                println!("White turn");
                while !self.take_turn_white() {}
                self.board.set_turn(Turn::Black);
            }
            Turn::Black => {
                println!("Black turn");
                while !self.take_turn_black() {}
                self.board.set_turn(Turn::White);
            }
        }
    }

    pub fn is_checkmate(&self, color: PlayingAs) -> bool {
        match self.board.get_turn() {
            Turn::White => Movement::check_for_checkmate(color, self.board),
            Turn::Black => Movement::check_for_checkmate(color, self.board),
        }
    }

    fn print_invalid_move(&self, move_result: bool, check_result: bool) {
        if !move_result {
            println!("Invalid Move!");
        }

        if check_result {
            println!("Can't end turn on check!!");
        }
    }

    fn take_turn_white(&mut self) -> bool {
        let mut move_result = false;
        match self.white {
            Players::Robot => {
                move_result = self.board.perform_move(
                    Analyzer::do_move(),
                    super::analyzer::analyzer::PlayingAs::White,
                );
            }
            Players::Player => {
                move_result = self.board.perform_move(
                    Human::do_move(),
                    super::analyzer::analyzer::PlayingAs::White,
                )
            }
        };

        let check_result = Movement::check_for_check(PlayingAs::White, self.board);
        if !move_result || check_result {
            self.print_invalid_move(move_result, check_result);
            return false;
        }
        return true;
    }
    fn take_turn_black(&mut self) -> bool {
        let mut move_result = false;
        match self.black {
            Players::Robot => {
                move_result = self.board.perform_move(
                    Analyzer::do_move(),
                    super::analyzer::analyzer::PlayingAs::Black,
                )
            }
            Players::Player => {
                move_result = self.board.perform_move(
                    Human::do_move(),
                    super::analyzer::analyzer::PlayingAs::Black,
                )
            }
        };

        let check_result = Movement::check_for_check(PlayingAs::Black, self.board);
        if !move_result || check_result {
            self.print_invalid_move(move_result, check_result);
            return false;
        }
        return true;
    }
}
