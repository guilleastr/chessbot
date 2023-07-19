#[cfg(test)]
mod pawn_tests {
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

    fn do_move(analyzer: Analyzer, test_bits: u64) -> u64 {
        let result: u64 = Movement::get_pawn_moves(
            test_bits,
            analyzer.playing_as,
            analyzer.board.getWhiteBitboard(),
            analyzer.board.getBlackBitboard(),
        );
        return result;
    }

    //Empty board tests
    #[test]
    fn empty_board_bottom_right() {
        let test_bits: u64 = 0x1;
        let analyzer = Analyzer::new_with_board(PlayingAs::White, Board::new_empty());

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x100);
    }

    #[test]
    fn empty_board_first_rank_rigth() {
        let test_bits: u64 = 0x100;
        let analyzer = Analyzer::new_with_board(PlayingAs::White, Board::new_empty());

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x1010000);
    }

    #[test]
    fn empty_board_first_rank_left() {
        let test_bits: u64 = 0x8000;
        let analyzer = Analyzer::new_with_board(PlayingAs::White, Board::new_empty());

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x80800000);
    }

    #[test]
    fn empty_board_middle_rank_middle() {
        let test_bits: u64 = 0x10000000;
        let analyzer = Analyzer::new_with_board(PlayingAs::White, Board::new_empty());

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x1000000000);
    }

    #[test]
    fn empty_board_middle_rank_right() {
        let test_bits: u64 = 0x100000000;
        let analyzer = Analyzer::new_with_board(PlayingAs::White, Board::new_empty());

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x10000000000);
    }

    #[test]
    fn empty_board_middle_rank_left() {
        let test_bits: u64 = 0x8000000000;
        let analyzer = Analyzer::new_with_board(PlayingAs::White, Board::new_empty());

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x800000000000);
    }

    #[test]
    fn empty_board_top_rank_left() {
        let test_bits: u64 = 0x8000000000000000;
        let analyzer = Analyzer::new_with_board(PlayingAs::White, Board::new_empty());

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x0);
    }

    #[test]
    fn empty_board_top_rank_right() {
        let test_bits: u64 = 0x100000000000000;
        let analyzer = Analyzer::new_with_board(PlayingAs::White, Board::new_empty());

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x0);
    }

    #[test]
    fn empty_board_top_rank_middle() {
        let test_bits: u64 = 0x1000000000000000;
        let analyzer = Analyzer::new_with_board(PlayingAs::White, Board::new_empty());

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x0);
    }

    //Full board
    //with ally
    #[test]
    fn full_board_first_rank_rigth() {
        let test_bits: u64 = 0x400;
        let analyzer =
            Analyzer::new_with_board(PlayingAs::White, get_board_by_color(&PlayingAs::White));

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x0);
    }

    #[test]
    fn full_board_first_rank_middle() {
        let test_bits: u64 = 0x800;
        let analyzer =
            Analyzer::new_with_board(PlayingAs::White, get_board_by_color(&PlayingAs::White));

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x80000);
    }

    #[test]
    fn full_board_first_rank_left() {
        let test_bits: u64 = 0x2000;
        let analyzer =
            Analyzer::new_with_board(PlayingAs::White, get_board_by_color(&PlayingAs::White));

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x0);
    }

    #[test]
    fn full_board_middle_rank_left() {
        let test_bits: u64 = 0x2000000000;
        let analyzer =
            Analyzer::new_with_board(PlayingAs::White, get_board_by_color(&PlayingAs::White));

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x0);
    }

    #[test]
    fn full_board_middle_rank_right() {
        let test_bits: u64 = 0x400000000;
        let analyzer =
            Analyzer::new_with_board(PlayingAs::White, get_board_by_color(&PlayingAs::White));

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x0);
    }

    //with enemy
    #[test]
    fn full_board_first_rank_rigth_enemy() {
        let test_bits: u64 = 0x800;
        let analyzer =
            Analyzer::new_with_board(PlayingAs::White, get_board_by_color(&PlayingAs::Black));

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x80000);
    }

    #[test]
    fn full_board_first_rank_left_enemy() {
        let test_bits: u64 = 0x1000;
        let analyzer =
            Analyzer::new_with_board(PlayingAs::White, get_board_by_color(&PlayingAs::Black));

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x100000);
    }

    #[test]
    fn full_board_middle_rank_left_enemy() {
        let test_bits: u64 = 0x100000;
        let analyzer =
            Analyzer::new_with_board(PlayingAs::White, get_board_by_color(&PlayingAs::Black));

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x8000000);
    }
    #[test]
    fn full_board_middle_rank_right_enemy() {
        let test_bits: u64 = 0x80000;
        let analyzer =
            Analyzer::new_with_board(PlayingAs::White, get_board_by_color(&PlayingAs::Black));

        let result: u64 = do_move(analyzer, test_bits);
        assert_eq!(result, 0x10000000);
    }
}
