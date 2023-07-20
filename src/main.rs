use engine::game::game::Game;

use std::{thread, time};

mod engine;

const W_ROOKS: u64 = 0x8100000000000081;
const W_KNIGHTS: u64 = 0x42000000004200;
const W_BISHOPS: u64 = 0x240000240000;
const W_QUEEN: u64 = 0x800000000;
const W_KING: u64 = 0x800000000000000;
const W_PAWNS: u64 = 0x18000000;

fn main() {
    let mut game = Game::setup(
        engine::game::game::PlayerTypes::HUMAN,
        engine::game::game::PlayerTypes::HUMAN,
    );

    loop {
        game.take_turn();
        thread::sleep(time::Duration::from_millis(500));
    }
}

/*

const test_bits: u64 = 0x100000000000;

   let board_to_play = Board::new();

   let analyzer = Analyzer::new_with_board(PlayingAs::White, board_to_play);

   //const test_bits: u64 = 0b0000000000000000000000000000000000000000000000000000000000000010;

   let piece = board::board::PieceType::BlackKnight;

   let mut value = 0;
   for index in 0..100000 {
       let start = Instant::now();
       let movement_bits = Movement::extract_pieces_moves_from_bitboard(
           analyzer.board.b_knights,
           PlayingAs::Black,
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
