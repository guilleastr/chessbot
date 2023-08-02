#[cfg(test)]
mod bishop_tests {
    use crate::engine::{
        board::board::{Board, Turn},
        game::analyzer::analyzer::Analyzer,
        movement::movement::Movement,
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
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x8040201008040200);
    }

    #[test]
    fn empty_board_bottom_left() {
        let board = Board::new_empty();

        let test_bits: u64 = 0b0000000000000000000000000000000000000000000000000000000010000000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );

        assert_eq!(result, 0x102040810204000);
    }

    #[test]
    fn empty_board_top_right() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x100000000000000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x2040810204080);
    }

    #[test]
    fn empty_board_top_left() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x8000000000000000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x40201008040201);
    }

    #[test]
    fn empty_board_middle() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x10000000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x182442800284482);
    }

    #[test]
    fn empty_board_bottom() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x8;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x8041221400);
    }

    #[test]
    fn empty_board_top() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x1000000000000000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x28448201000000);
    }

    #[test]
    fn empty_board_left() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x8000000000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x1020400040201008);
    }

    #[test]
    fn empty_board_rigth() {
        let board = Board::new_empty();

        let test_bits: u64 = 0x1000000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x1008040200020408);
    }

    //Full board tests
    // with same color block
    #[test]
    fn full_board_bottom_right_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x4;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x804020100800);
    }

    #[test]
    fn full_board_bottom_left_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x20;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x10204081000);
    }

    #[test]
    fn full_board_left_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x800000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x408102040000000);
    }

    #[test]
    fn full_board_rigth_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x10000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x2010080402000000);
    }

    #[test]
    fn full_board_top_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x800000000000000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x14020100000000);
    }

    #[test]
    fn full_board_middle_top_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x8000000000000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x1400102040800000);
    }
    #[test]
    fn full_board_middle_bottom_block() {
        let board = get_board_by_color(&Turn::Black);

        let test_bits: u64 = 0x800;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x804020100014);
    }

    //with enemy color block

    #[test]
    fn full_board_bottom_right_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x4;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x804020100a00);
    }
    #[test]
    fn full_board_bottom_left_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x20;
        board.print_board_self("Test board");
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );

        assert_eq!(result, 0x10204085000);
    }

    #[test]
    fn full_board_left_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x800000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x408102040004000);
    }

    #[test]
    fn full_board_rigth_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x10000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x2010080402000200);
    }

    #[test]
    fn full_board_top_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x800000000000000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x14220100000000);
    }

    #[test]
    fn full_board_middle_top_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x8000000000000;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x1400142040800000);
    }
    #[test]
    fn full_board_middle_bottom_block_enemy() {
        let board = get_board_by_color(&Turn::White);

        let test_bits: u64 = 0x800;
        let result: u64 = Movement::get_bishop_moves_bitboard(
            test_bits,
            Turn::Black,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        assert_eq!(result, 0x804020140014);
    }
}
