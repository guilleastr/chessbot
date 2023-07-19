#[cfg(test)]
mod knigth_tests {
    use crate::{
        board::board::Board,
        engine::{
            game::analyzer::analyzer::{Analyzer, PlayingAs},
            movement::movement::Movement,
        },
    };

    const B_ROOKS: u64 = 0x8100000000000081;
    const B_KNIGHTS: u64 = 0x42000000004200;
    const B_BISHOPS: u64 = 0x240000240000;
    const B_QUEEN: u64 = 0x800000000;
    const B_KING: u64 = 0x0;
    const B_PAWNS: u64 = 0x18000000;

    const W_ROOKS: u64 = 0x8100000000000081;
    const W_KNIGHTS: u64 = 0x42000000004200;
    const W_BISHOPS: u64 = 0x240000240000;
    const W_QUEEN: u64 = 0x800000000;
    const W_KING: u64 = 0x0;
    const W_PAWNS: u64 = 0x18000000;

    fn get_board_by_color(color: &PlayingAs) -> Board {
        match color {
            PlayingAs::White => Board::new_from_values(
                0, 0, 0, 0, 0, 0, W_PAWNS, W_KNIGHTS, W_BISHOPS, W_ROOKS, W_QUEEN, W_KING,
            ),
            PlayingAs::Black => Board::new_from_values(
                B_PAWNS, B_KNIGHTS, B_BISHOPS, B_ROOKS, B_QUEEN, B_KING, 0, 0, 0, 0, 0, 0,
            ),
        }
    }

    //Empty board tests
    #[test]
    fn empty_board_bottom_right() {
        let analyzer = Analyzer::new_with_board(PlayingAs::Black, Board::new_empty());

        let test_bits: u64 = 0x1;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x20400);
    }

    #[test]
    fn empty_board_bottom_left() {
        let analyzer = Analyzer::new_with_board(PlayingAs::Black, Board::new_empty());

        let test_bits: u64 = 0x80;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x402000);
    }

    #[test]
    fn empty_board_top_right() {
        let analyzer = Analyzer::new_with_board(PlayingAs::Black, Board::new_empty());

        let test_bits: u64 = 0x100000000000000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x4020000000000);
    }

    #[test]
    fn empty_board_top_left() {
        let analyzer = Analyzer::new_with_board(PlayingAs::Black, Board::new_empty());

        let test_bits: u64 = 0x8000000000000000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x20400000000000);
    }

    #[test]
    fn empty_board_middle() {
        let analyzer = Analyzer::new_with_board(PlayingAs::Black, Board::new_empty());

        let test_bits: u64 = 0x10000000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x284400442800);
    }

    #[test]
    fn empty_board_bottom() {
        let analyzer = Analyzer::new_with_board(PlayingAs::Black, Board::new_empty());

        let test_bits: u64 = 0x8;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x142200);
    }

    #[test]
    fn empty_board_top() {
        let analyzer = Analyzer::new_with_board(PlayingAs::Black, Board::new_empty());

        let test_bits: u64 = 0x1000000000000000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x44280000000000);
    }

    #[test]
    fn empty_board_left() {
        let analyzer = Analyzer::new_with_board(PlayingAs::Black, Board::new_empty());

        let test_bits: u64 = 0x8000000000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x40200020400000);
    }

    #[test]
    fn empty_board_rigth() {
        let analyzer = Analyzer::new_with_board(PlayingAs::Black, Board::new_empty());

        let test_bits: u64 = 0x1000000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x20400040200);
    }

    //Full board tests
    // with same color block
    #[test]
    fn full_board_bottom_right_block() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::Black));

        let test_bits: u64 = 0x4;

        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0xa1100);
    }

    #[test]
    fn full_board_bottom_left_block() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::Black));

        let test_bits: u64 = 0x20;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x508800);
    }

    #[test]
    fn full_board_left_block() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::Black));

        let test_bits: u64 = 0x800000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x4020002040);
    }

    #[test]
    fn full_board_rigth_block() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::Black));

        let test_bits: u64 = 0x10000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x204000402);
    }

    #[test]
    fn full_board_top_block() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::Black));

        let test_bits: u64 = 0x800000000000000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x20100000000000);
    }

    #[test]
    fn full_board_middle_top_block() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::Black));

        let test_bits: u64 = 0x8000000000000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x2200021400000000);
    }
    #[test]
    fn full_board_middle_bottom_block() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::Black));

        let test_bits: u64 = 0x800;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x4020022);
    }

    //with enemy color block
    #[test]
    fn full_board_bottom_right_block_enemy() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::White));

        let test_bits: u64 = 0x4;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0xa1100);
    }
    #[test]
    fn full_board_bottom_left_block_enemy() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::White));

        let test_bits: u64 = 0x20;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x508800);
    }

    #[test]
    fn full_board_left_block_enemy() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::White));

        let test_bits: u64 = 0x800000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x4020002040);
    }

    #[test]
    fn full_board_rigth_block_enemy() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::White));

        let test_bits: u64 = 0x10000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x204000402);
    }

    #[test]
    fn full_board_top_block_enemy() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::White));

        let test_bits: u64 = 0x800000000000000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x22140000000000);
    }

    #[test]
    fn full_board_middle_top_block_enemy() {
        let analyzer =
            Analyzer::new_with_board(PlayingAs::Black, get_board_by_color(&PlayingAs::White));

        let test_bits: u64 = 0x8000000000000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x2200221400000000);
    }
    #[test]
    fn full_board_middle_bottom_block_enemy() {
        let analyzer = Analyzer::new_with_board(PlayingAs::Black, Board::new_empty());

        let test_bits: u64 = 0x1000000;
        let result: u64 = Movement::get_knigth_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        assert_eq!(result, 0x20400040200);
    }
}
