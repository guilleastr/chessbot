#[cfg(test)]
mod pawn_tests {
    use crate::engine::{
        board::board::{Board, Turn},
        movement::movement::Movement,
    };

    fn get_board_by_color(color: &Turn) -> Board {
        match color {
            Turn::White => {
                Board::new_from_fenn_notation("R6R/1N4N1/2B2B2/4Q3/3PP3/2B2B2/1N4N1/R6R b - - 0 1")
            }
            Turn::Black => {
                Board::new_from_fenn_notation("R6R/1N4N1/2B2B2/4Q3/3PP3/2B2B2/1N4N1/R6R b - - 0 1")
            }
        }
    }

    fn do_move(board: Board, test_bits: u64) -> u64 {
        let result: u64 = Movement::get_pawn_moves(
            test_bits,
            Turn::White,
            board.getWhiteBitboard(),
            board.getBlackBitboard(),
        );
        return result;
    }

    //Empty board tests
    #[test]
    fn empty_board_bottom_right() {
        let test_bits: u64 = 0x1;
        let board = Board::new_empty();

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x100);
    }

    #[test]
    fn empty_board_first_rank_rigth() {
        let test_bits: u64 = 0x100;
        let board = Board::new_empty();

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x1010000);
    }

    #[test]
    fn empty_board_first_rank_left() {
        let test_bits: u64 = 0x8000;
        let board = Board::new_empty();

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x80800000);
    }

    #[test]
    fn empty_board_middle_rank_middle() {
        let test_bits: u64 = 0x10000000;
        let board = Board::new_empty();

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x1000000000);
    }

    #[test]
    fn empty_board_middle_rank_right() {
        let test_bits: u64 = 0x100000000;
        let board = Board::new_empty();

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x10000000000);
    }

    #[test]
    fn empty_board_middle_rank_left() {
        let test_bits: u64 = 0x8000000000;
        let board = Board::new_empty();

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x800000000000);
    }

    #[test]
    fn empty_board_top_rank_left() {
        let test_bits: u64 = 0x8000000000000000;
        let board = Board::new_empty();

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x0);
    }

    #[test]
    fn empty_board_top_rank_right() {
        let test_bits: u64 = 0x100000000000000;
        let board = Board::new_empty();

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x0);
    }

    #[test]
    fn empty_board_top_rank_middle() {
        let test_bits: u64 = 0x1000000000000000;
        let board = Board::new_empty();

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x0);
    }

    //Full board
    //with ally
    #[test]
    fn full_board_first_rank_rigth() {
        let test_bits: u64 = 0x400;
        let board = get_board_by_color(&Turn::White);

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x0);
    }

    #[test]
    fn full_board_first_rank_middle() {
        let test_bits: u64 = 0x800;
        let board = get_board_by_color(&Turn::White);

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x80000);
    }

    #[test]
    fn full_board_first_rank_left() {
        let test_bits: u64 = 0x2000;
        let board = get_board_by_color(&Turn::White);

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x0);
    }

    #[test]
    fn full_board_middle_rank_left() {
        let test_bits: u64 = 0x2000000000;
        let board = get_board_by_color(&Turn::White);

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x0);
    }

    #[test]
    fn full_board_middle_rank_right() {
        let test_bits: u64 = 0x400000000;
        let board = get_board_by_color(&Turn::White);

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x0);
    }

    //with enemy
    #[test]
    fn full_board_first_rank_rigth_enemy() {
        let test_bits: u64 = 0x800;
        let board = get_board_by_color(&Turn::Black);

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x80000);
    }

    #[test]
    fn full_board_first_rank_left_enemy() {
        let test_bits: u64 = 0x1000;
        let board = get_board_by_color(&Turn::Black);

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x100000);
    }

    #[test]
    fn full_board_middle_rank_left_enemy() {
        let test_bits: u64 = 0x100000;
        let board = get_board_by_color(&Turn::Black);

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x8000000);
    }
    #[test]
    fn full_board_middle_rank_right_enemy() {
        let test_bits: u64 = 0x80000;
        let board = get_board_by_color(&Turn::Black);

        let result: u64 = do_move(board, test_bits);
        assert_eq!(result, 0x10000000);
    }
}
