use crate::engine::{
    board::{board::Board, position::position::Move},
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
                println!("White turn");
                while !self.take_turn_white() {}
                self.turn = Turn::Black;
            }
            Turn::Black => {
                println!("Black turn");
                while !self.take_turn_black() {}
                self.turn = Turn::White;
            }
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
