use engine::game::game::Game;

use std::{
    thread,
    time::{self, Instant},
};

mod engine;

fn main() {
    //play_game()
}

fn test() {
    let fen_board = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let mut game = Game::setup_from_fenn(
        fen_board,
        engine::game::game::PlayerTypes::HUMAN,
        engine::game::game::PlayerTypes::AI,
    );
}

fn play_game() {
    let fen_board = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    let mut game = Game::setup_from_fenn(
        fen_board,
        engine::game::game::PlayerTypes::HUMAN,
        engine::game::game::PlayerTypes::AI,
    );

    loop {
        let start = Instant::now();
        game.take_turn();
        let duration = start.elapsed();
        println!("{}ns", duration.as_millis());
        thread::sleep(time::Duration::from_millis(500));
    }
}

/*

const test_bits: u64 = 0x100000000000;

   let board_to_play = Board::new();

   let analyzer = Analyzer::new_with_board(Turn::White, board_to_play);

   //const test_bits: u64 = 0b0000000000000000000000000000000000000000000000000000000000000010;

   let piece = board::board::PieceType::BlackKnight;

   let mut value = 0;
   for index in 0..100000 {
       let start = Instant::now();
       let movement_bits = Movement::extract_pieces_moves_from_bitboard(
           analyzer.board.b_knights,
           Turn::Black,
           piece,
           analyzer.board.getWhiteBitboard(),
           analyzer.board.getBlackBitboard(),
       );

       let duration = start.elapsed();
       value += duration.as_nanos();
   }

   analyzer.board.print_board_self("Board State");

   //Board::print_board_moves(piece, movement_bits);
   println!("{}ns", value / 100000);





*/
