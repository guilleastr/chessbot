#[cfg(test)]
mod knigth_tests {
    use crate::engine::{
        board::board::{Board, Turn},
        movement::movement::Movement,
    };

    fn get_board_by_color(color: &Turn) -> Board {
        match color {
            Turn::White => {
                Board::new_from_fenn_notation("R6R/1N4N1/2B2B2/4Q3/3PP3/2B2B2/1N4N1/R6R w - - 0 1")
            }
            Turn::Black => {
                Board::new_from_fenn_notation("R6R/1N4N1/2B2B2/4Q3/3PP3/2B2B2/1N4N1/R6R w - - 0 1")
            }
        }
    }

    //Empty board tests
    #[test]
    fn empty_board_bottom_right() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x1;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x20400);
    }

    #[test]
    fn empty_board_bottom_left() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x80;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x402000);
    }

    #[test]
    fn empty_board_top_right() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x100000000000000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x4020000000000);
    }

    #[test]
    fn empty_board_top_left() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x8000000000000000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x20400000000000);
    }

    #[test]
    fn empty_board_middle() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x10000000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x284400442800);
    }

    #[test]
    fn empty_board_bottom() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x8;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x142200);
    }

    #[test]
    fn empty_board_top() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x1000000000000000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x44280000000000);
    }

    #[test]
    fn empty_board_left() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x8000000000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x40200020400000);
    }

    #[test]
    fn empty_board_rigth() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x1000000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x20400040200);
    }

    //Full board tests
    // with same color block
    #[test]
    fn full_board_bottom_right_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x4;

        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0xa1100);
    }

    #[test]
    fn full_board_bottom_left_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x20;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x508800);
    }

    #[test]
    fn full_board_left_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x800000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x4020002040);
    }

    #[test]
    fn full_board_rigth_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x10000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x204000402);
    }

    #[test]
    fn full_board_top_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x800000000000000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x20100000000000);
    }

    #[test]
    fn full_board_middle_top_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x8000000000000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x2200021400000000);
    }
    #[test]
    fn full_board_middle_bottom_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x800;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x4020022);
    }

    //with enemy color block
    #[test]
    fn full_board_bottom_right_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x4;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0xa1100);
    }
    #[test]
    fn full_board_bottom_left_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x20;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x508800);
    }

    #[test]
    fn full_board_left_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x800000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x4020002040);
    }

    #[test]
    fn full_board_rigth_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x10000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x204000402);
    }

    #[test]
    fn full_board_top_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x800000000000000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x22140000000000);
    }

    #[test]
    fn full_board_middle_top_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x8000000000000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x2200221400000000);
    }
    #[test]
    fn full_board_middle_bottom_block_enemy() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x1000000;
        let result: u64 = Movement::get_knigth_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x20400040200);
    }
}
