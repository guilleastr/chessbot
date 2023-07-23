use std::io::stdin;

use crate::engine::{
    board::{
        board::{Board, Turn},
        position::position::{LegalMove, Position},
    },
    game::game::Player,
};

const COLUMNS: [&'static str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

pub struct Human {}

impl Human {
    pub fn new() -> Human {
        return Human {};
    }
}

impl Player for Human {
    fn do_move(board: Board, color: Turn) -> LegalMove {
        println!("Enter your move (a1;e2):");
        let mut buffer = String::new();

        // `read_line` returns `Result` of bytes read
        stdin().read_line(&mut buffer).unwrap();

        let input: Vec<&str> = buffer.trim().split(";").collect();

        if input.len() < 2 {
            match buffer.trim() {
                "O-O" => {
                    return LegalMove {
                        from: Position { colum: 0, row: 0 },
                        to: Position { colum: 0, row: 0 },
                        castle: crate::engine::board::position::position::CastleOptions::KingSide,
                    };
                }
                "O-O-O" => {
                    return LegalMove {
                        from: Position { colum: 0, row: 0 },
                        to: Position { colum: 0, row: 0 },
                        castle: crate::engine::board::position::position::CastleOptions::QueenSide,
                    };
                }
                _ => {}
            }
        }

        let from_str = input[0];
        let to_str = input[1];

        let from: Vec<&str> = from_str.trim().split("").collect();
        let to: Vec<&str> = to_str.split("").collect();

        let from_column = 7 - COLUMNS.iter().position(|&x| from[1] == x).unwrap_or(0) as i8;
        let to_column = 7 - COLUMNS.iter().position(|&x| to[1] == x).unwrap_or(0) as i8;

        let legal_move = LegalMove {
            from: Position {
                colum: from_column,
                row: from[2].parse().unwrap_or(0) - 1,
            },
            to: Position {
                colum: to_column,
                row: to[2].parse().unwrap_or(0) - 1,
            },
            castle: crate::engine::board::position::position::CastleOptions::None,
        };
        return legal_move;
    }
}
