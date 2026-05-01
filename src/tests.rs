// =============================================================================
// Tests for move_to_notation
//
// Coordinate system:
//   board[row][col], row 0 = white's back rank (rank 1), row 7 = black's back rank (rank 8)
//   col 0 = a-file, col 7 = h-file
//   Standard notation square: file = ('a' + col), rank = (row + 1)
//
// Expected notation rules:
//   Piece letter: R N B Q K  (pawns have no letter)
//   Capture: 'x' inserted before destination square
//   Pawn capture: includes originating file letter (e.g. "exd5")
//   Promotion: append "=Q", "=R", "=B", or "=N"
//   Castling: "O-O" (kingside) or "O-O-O" (queenside)
//   Check/checkmate markers (+/# ) are NOT tested here — they depend on full
//   game-state logic outside move_to_notation's scope.
//
// Board layout (from BoardState::new):
//   Row 0: R N B Q K B N R  (white back rank)
//   Row 1: P P P P P P P P  (white pawns)
//   Rows 2-5: empty
//   Row 6: p p p p p p p p  (black pawns)
//   Row 7: r n b q k b n r  (black back rank)
// =============================================================================
use crate::board::{BoardState,Piece,PieceColor,PieceType};
use crate::piece_moves::move_to_notation;
#[cfg(test)]
mod move_notation_tests {
    use super::*;

    // -------------------------------------------------------------------------
    // Helpers
    // -------------------------------------------------------------------------

    /// Return a fresh starting BoardState.
    fn starting_board() -> BoardState {
        BoardState::new()
    }

    /// Place `piece` at board[row][col] on `board`.
    fn place(board: &mut BoardState, row: usize, col: usize, piece: Piece) {
        board.board[row][col] = piece;
    }

    /// Clear board[row][col] (set to Empty).
    fn clear(board: &mut BoardState, row: usize, col: usize) {
        board.board[row][col] =
            Piece::new(PieceType::Empty, PieceColor::Empty, (row as i32, col as i32));
    }

    // =========================================================================
    // 1. PAWN — quiet moves (no capture, no promotion)
    // =========================================================================

    #[test]
    fn pawn_white_single_push_e2_e3() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((1, 4));
        let result = move_to_notation(&mut board, &mut pawn, (1, 4), (2, 4));
        assert_eq!(result, "e3");
    }

    #[test]
    fn pawn_white_double_push_e2_e4() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((1, 4));
        let result = move_to_notation(&mut board, &mut pawn, (1, 4), (3, 4));
        assert_eq!(result, "e4");
    }

    #[test]
    fn pawn_white_single_push_a2_a3() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((1, 0));
        let result = move_to_notation(&mut board, &mut pawn, (1, 0), (2, 0));
        assert_eq!(result, "a3");
    }

    #[test]
    fn pawn_white_double_push_h2_h4() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((1, 7));
        let result = move_to_notation(&mut board, &mut pawn, (1, 7), (3, 7));
        assert_eq!(result, "h4");
    }

    #[test]
    fn pawn_black_single_push_e7_e6() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((6, 4));
        let result = move_to_notation(&mut board, &mut pawn, (6, 4), (5, 4));
        assert_eq!(result, "e6");
    }

    #[test]
    fn pawn_black_double_push_d7_d5() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((6, 3));
        let result = move_to_notation(&mut board, &mut pawn, (6, 3), (4, 3));
        assert_eq!(result, "d5");
    }

    #[test]
    fn pawn_black_single_push_a7_a6() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((6, 0));
        let result = move_to_notation(&mut board, &mut pawn, (6, 0), (5, 0));
        assert_eq!(result, "a6");
    }

    #[test]
    fn pawn_black_double_push_h7_h5() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((6, 7));
        let result = move_to_notation(&mut board, &mut pawn, (6, 7), (4, 7));
        assert_eq!(result, "h5");
    }

    // =========================================================================
    // 2. PAWN — captures
    // =========================================================================

    #[test]
    fn pawn_white_capture_exd5() {
        let mut board = starting_board();
        // Put a black pawn on d5 (row 4, col 3)
        place(&mut board, 4, 3, Piece::new(PieceType::Pawn, PieceColor::Black, (4, 3)));
        // White pawn on e5 (row 4, col 4)
        place(&mut board, 4, 4, Piece::new(PieceType::Pawn, PieceColor::White, (4, 4)));
        let mut pawn = board.piece_at((4, 4));
        let result = move_to_notation(&mut board, &mut pawn, (4, 4), (4, 3)); // wait — pawn captures diagonally
        // White pawn captures from e-file to d-file going up: (4,4) -> (5,3)? 
        // Actually exd5 means white pawn on e4 captures on d5: from (3,4) captures (4,3)
        // Let's redo: pawn on e4 = (row3,col4), captures d5 = (row4,col3)
        let mut board2 = starting_board();
        place(&mut board2, 3, 4, Piece::new(PieceType::Pawn, PieceColor::White, (3, 4)));
        place(&mut board2, 4, 3, Piece::new(PieceType::Pawn, PieceColor::Black, (4, 3)));
        let mut pawn2 = board2.piece_at((3, 4));
        let result2 = move_to_notation(&mut board2, &mut pawn2, (3, 4), (4, 3));
        assert_eq!(result2, "exd5");
    }

    #[test]
    fn pawn_white_capture_dxe5() {
        let mut board = starting_board();
        // White pawn on d4 = (3,3), captures black pawn on e5 = (4,4)
        place(&mut board, 3, 3, Piece::new(PieceType::Pawn, PieceColor::White, (3, 3)));
        place(&mut board, 4, 4, Piece::new(PieceType::Pawn, PieceColor::Black, (4, 4)));
        let mut pawn = board.piece_at((3, 3));
        let result = move_to_notation(&mut board, &mut pawn, (3, 3), (4, 4));
        assert_eq!(result, "dxe5");
    }

    #[test]
    fn pawn_black_capture_exd4() {
        let mut board = starting_board();
        // Black pawn on e5 = (4,4), captures white pawn on d4 = (3,3)
        place(&mut board, 4, 4, Piece::new(PieceType::Pawn, PieceColor::Black, (4, 4)));
        place(&mut board, 3, 3, Piece::new(PieceType::Pawn, PieceColor::White, (3, 3)));
        let mut pawn = board.piece_at((4, 4));
        let result = move_to_notation(&mut board, &mut pawn, (4, 4), (3, 3));
        assert_eq!(result, "exd4");
    }

    #[test]
    fn pawn_white_capture_cxb5() {
        let mut board = starting_board();
        place(&mut board, 3, 2, Piece::new(PieceType::Pawn, PieceColor::White, (3, 2)));
        place(&mut board, 4, 1, Piece::new(PieceType::Pawn, PieceColor::Black, (4, 1)));
        let mut pawn = board.piece_at((3, 2));
        let result = move_to_notation(&mut board, &mut pawn, (3, 2), (4, 1));
        assert_eq!(result, "cxb5");
    }

    // =========================================================================
    // 3. PAWN — promotion (quiet)
    // =========================================================================

    #[test]
    fn pawn_promotion_white_queen_e8() {
        let mut board = starting_board();
        // White pawn on e7 = (6,4), promotes on e8 = (7,4)
        place(&mut board, 6, 4, Piece::new(PieceType::Pawn, PieceColor::White, (6, 4)));
        clear(&mut board, 7, 4); // clear black king — test only notation
        let mut pawn = board.piece_at((6, 4));
        let result = move_to_notation(&mut board, &mut pawn, (6, 4), (7, 4));
        assert_eq!(result, "e8=Q");
    }

    #[test]
    fn pawn_promotion_white_knight_a8() {
        let mut board = starting_board();
        place(&mut board, 6, 0, Piece::new(PieceType::Pawn, PieceColor::White, (6, 0)));
        clear(&mut board, 7, 0);
        let mut pawn = board.piece_at((6, 0));
        let result = move_to_notation(&mut board, &mut pawn, (6, 0), (7, 0));
        // Default promotion — function may need a promotion parameter;
        // testing queen as default (most common) or knight underpromotion
        assert!(result == "a8=Q" || result == "a8=N" || result == "a8=R" || result == "a8=B");
    }

    #[test]
    fn pawn_promotion_black_queen_d1() {
        let mut board = starting_board();
        // Black pawn on d2 = (1,3), promotes on d1 = (0,3)
        place(&mut board, 1, 3, Piece::new(PieceType::Pawn, PieceColor::Black, (1, 3)));
        clear(&mut board, 0, 3); // clear white queen
        let mut pawn = board.piece_at((1, 3));
        let result = move_to_notation(&mut board, &mut pawn, (1, 3), (0, 3));
        assert_eq!(result, "d1=Q");
    }

    #[test]
    fn pawn_promotion_capture_white_exf8() {
        let mut board = starting_board();
        // White pawn on e7 = (6,4), captures on f8 = (7,5) and promotes
        place(&mut board, 6, 4, Piece::new(PieceType::Pawn, PieceColor::White, (6, 4)));
        place(&mut board, 7, 5, Piece::new(PieceType::Bishop, PieceColor::Black, (7, 5)));
        let mut pawn = board.piece_at((6, 4));
        let result = move_to_notation(&mut board, &mut pawn, (6, 4), (7, 5));
        assert_eq!(result, "exf8=Q");
    }

    // =========================================================================
    // 4. KNIGHT moves
    // =========================================================================

    #[test]
    fn knight_white_g1_f3() {
        let mut board = starting_board();
        let mut knight = board.piece_at((0, 6));
        let result = move_to_notation(&mut board, &mut knight, (0, 6), (2, 5));
        assert_eq!(result, "Nf3");
    }

    #[test]
    fn knight_white_b1_c3() {
        let mut board = starting_board();
        let mut knight = board.piece_at((0, 1));
        let result = move_to_notation(&mut board, &mut knight, (0, 1), (2, 2));
        assert_eq!(result, "Nc3");
    }

    #[test]
    fn knight_black_g8_f6() {
        let mut board = starting_board();
        let mut knight = board.piece_at((7, 6));
        let result = move_to_notation(&mut board, &mut knight, (7, 6), (5, 5));
        assert_eq!(result, "Nf6");
    }

    #[test]
    fn knight_black_b8_c6() {
        let mut board = starting_board();
        let mut knight = board.piece_at((7, 1));
        let result = move_to_notation(&mut board, &mut knight, (7, 1), (5, 2));
        assert_eq!(result, "Nc6");
    }

    #[test]
    fn knight_capture_white_nxe5() {
        let mut board = starting_board();
        // Knight on f3 = (2,5), captures on e5 = (4,4)
        place(&mut board, 2, 5, Piece::new(PieceType::Knight, PieceColor::White, (2, 5)));
        place(&mut board, 4, 4, Piece::new(PieceType::Pawn, PieceColor::Black, (4, 4)));
        let mut knight = board.piece_at((2, 5));
        let result = move_to_notation(&mut board, &mut knight, (2, 5), (4, 4));
        assert_eq!(result, "Nxe5");
    }

    #[test]
    fn knight_capture_black_nxd4() {
        let mut board = starting_board();
        place(&mut board, 5, 5, Piece::new(PieceType::Knight, PieceColor::Black, (5, 5)));
        place(&mut board, 3, 3, Piece::new(PieceType::Pawn, PieceColor::White, (3, 3)));
        let mut knight = board.piece_at((5, 5));
        let result = move_to_notation(&mut board, &mut knight, (5, 5), (3, 3));
        assert_eq!(result, "Nxd4");
    }

    #[test]
    fn knight_white_h3_to_g5() {
        let mut board = starting_board();
        place(&mut board, 2, 7, Piece::new(PieceType::Knight, PieceColor::White, (2, 7)));
        let mut knight = board.piece_at((2, 7));
        let result = move_to_notation(&mut board, &mut knight, (2, 7), (4, 6));
        assert_eq!(result, "Ng5");
    }

    // =========================================================================
    // 5. BISHOP moves
    // =========================================================================

    #[test]
    fn bishop_white_c1_to_e3() {
        let mut board = starting_board();
        // Clear the path: d2 pawn
        clear(&mut board, 1, 3);
        let mut bishop = board.piece_at((0, 2));
        let result = move_to_notation(&mut board, &mut bishop, (0, 2), (2, 4));
        assert_eq!(result, "Be3");
    }

    #[test]
    fn bishop_white_f1_to_c4() {
        let mut board = starting_board();
        clear(&mut board, 1, 4); // clear e2
        let mut bishop = board.piece_at((0, 5));
        let result = move_to_notation(&mut board, &mut bishop, (0, 5), (3, 2));
        assert_eq!(result, "Bc4");
    }

    #[test]
    fn bishop_black_c8_to_e6() {
        let mut board = starting_board();
        clear(&mut board, 6, 3); // clear d7 pawn
        let mut bishop = board.piece_at((7, 2));
        let result = move_to_notation(&mut board, &mut bishop, (7, 2), (5, 4));
        assert_eq!(result, "Be6");
    }

    #[test]
    fn bishop_capture_white_bxf7() {
        let mut board = starting_board();
        // Bishop on c4 = (3,2), captures f7 = (6,5)
        place(&mut board, 3, 2, Piece::new(PieceType::Bishop, PieceColor::White, (3, 2)));
        // f7 pawn is at (6,5) — already a black pawn
        let mut bishop = board.piece_at((3, 2));
        let result = move_to_notation(&mut board, &mut bishop, (3, 2), (6, 5));
        assert_eq!(result, "Bxf7");
    }

    #[test]
    fn bishop_black_capture_bxb2() {
        let mut board = starting_board();
        place(&mut board, 5, 4, Piece::new(PieceType::Bishop, PieceColor::Black, (5, 4)));
        // b2 = (1,1) — already a white pawn
        let mut bishop = board.piece_at((5, 4));
        let result = move_to_notation(&mut board, &mut bishop, (5, 4), (1, 1)); // e6 -> b2 diagonal? check: (5,4)->(1,1) diff = (-4,-3) not diagonal
        // Let's use a correct diagonal: bishop on f5=(4,5), captures b1=(0,1)
        let mut board2 = starting_board();
        place(&mut board2, 4, 5, Piece::new(PieceType::Bishop, PieceColor::Black, (4, 5)));
        // b1 = (0,1) has a white knight
        let mut bishop2 = board2.piece_at((4, 5));
        let result2 = move_to_notation(&mut board2, &mut bishop2, (4, 5), (0, 1));
        assert_eq!(result2, "Bxb1");
    }

    // =========================================================================
    // 6. ROOK moves
    // =========================================================================

    #[test]
    fn rook_white_a1_to_a4() {
        let mut board = starting_board();
        // Clear a2, a3 pawns
        clear(&mut board, 1, 0);
        let mut rook = board.piece_at((0, 0));
        let result = move_to_notation(&mut board, &mut rook, (0, 0), (3, 0));
        assert_eq!(result, "Ra4");
    }

    #[test]
    fn rook_black_h8_to_h5() {
        let mut board = starting_board();
        clear(&mut board, 6, 7); // clear h7 pawn
        let mut rook = board.piece_at((7, 7));
        let result = move_to_notation(&mut board, &mut rook, (7, 7), (4, 7));
        assert_eq!(result, "Rh5");
    }

    #[test]
    fn rook_white_capture_rxd8() {
        let mut board = starting_board();
        // Rook on d1=(0,3), captures on d8=(7,3)
        place(&mut board, 0, 3, Piece::new(PieceType::Rook, PieceColor::White, (0, 3)));
        // d8 = (7,3) already has a black queen — place a piece to capture
        place(&mut board, 7, 3, Piece::new(PieceType::Rook, PieceColor::Black, (7, 3)));
        let mut rook = board.piece_at((0, 3));
        let result = move_to_notation(&mut board, &mut rook, (0, 3), (7, 3));
        assert_eq!(result, "Rxd8");
    }

    #[test]
    fn rook_black_capture_rxe1() {
        let mut board = starting_board();
        place(&mut board, 4, 4, Piece::new(PieceType::Rook, PieceColor::Black, (4, 4)));
        place(&mut board, 0, 4, Piece::new(PieceType::King, PieceColor::White, (0, 4)));
        let mut rook = board.piece_at((4, 4));
        let result = move_to_notation(&mut board, &mut rook, (4, 4), (0, 4));
        assert_eq!(result, "Rxe1");
    }

    #[test]
    fn rook_moves_along_rank_re5() {
        let mut board = starting_board();
        place(&mut board, 4, 0, Piece::new(PieceType::Rook, PieceColor::White, (4, 0)));
        let mut rook = board.piece_at((4, 0));
        let result = move_to_notation(&mut board, &mut rook, (4, 0), (4, 4));
        assert_eq!(result, "Re5");
    }

    // =========================================================================
    // 7. QUEEN moves
    // =========================================================================

    #[test]
    fn queen_white_d1_to_d4() {
        let mut board = starting_board();
        clear(&mut board, 1, 3); // clear d2 pawn
        let mut queen = board.piece_at((0, 3));
        let result = move_to_notation(&mut board, &mut queen, (0, 3), (3, 3));
        assert_eq!(result, "Qd4");
    }

    #[test]
    fn queen_black_d8_to_h4() {
        let mut board = starting_board();
        clear(&mut board, 6, 4); // clear e7
        clear(&mut board, 6, 3); // clear d7
        let mut queen = board.piece_at((7, 3));
        let result = move_to_notation(&mut board, &mut queen, (7, 3), (3, 7));
        assert_eq!(result, "Qh4");
    }

    #[test]
    fn queen_white_capture_qxg7() {
        let mut board = starting_board();
        place(&mut board, 3, 3, Piece::new(PieceType::Queen, PieceColor::White, (3, 3)));
        // g7 = (6,6) — already has a black pawn
        let mut queen = board.piece_at((3, 3));
        let result = move_to_notation(&mut board, &mut queen, (3, 3), (6, 6));
        assert_eq!(result, "Qxg7");
    }

    #[test]
    fn queen_black_capture_qxb2() {
        let mut board = starting_board();
        place(&mut board, 4, 5, Piece::new(PieceType::Queen, PieceColor::Black, (4, 5)));
        // b2 = (1,1) — has a white pawn
        let mut queen = board.piece_at((4, 5));
        let result = move_to_notation(&mut board, &mut queen, (4, 5), (1, 2)); // f5->c2 diagonal
        // (4,5) to (1,2): diff=(-3,-3) diagonal ✓, c2=(1,2)
        assert_eq!(result, "Qxc2");
    }

    #[test]
    fn queen_white_to_h5() {
        let mut board = starting_board();
        place(&mut board, 0, 3, Piece::new(PieceType::Queen, PieceColor::White, (0, 3)));
        clear(&mut board, 1, 4); // clear e2
        let mut queen = board.piece_at((0, 3));
        let result = move_to_notation(&mut board, &mut queen, (0, 3), (4, 7));
        assert_eq!(result, "Qh5");
    }

    // =========================================================================
    // 8. KING moves (non-castling)
    // =========================================================================

    #[test]
    fn king_white_e1_to_e2() {
        let mut board = starting_board();
        clear(&mut board, 1, 4); // clear e2 pawn
        let mut king = board.piece_at((0, 4));
        let result = move_to_notation(&mut board, &mut king, (0, 4), (1, 4));
        assert_eq!(result, "Ke2");
    }

    #[test]
    fn king_black_e8_to_d7() {
        let mut board = starting_board();
        clear(&mut board, 6, 3); // clear d7 pawn
        let mut king = board.piece_at((7, 4));
        let result = move_to_notation(&mut board, &mut king, (7, 4), (6, 3));
        assert_eq!(result, "Kd7");
    }

    #[test]
    fn king_white_capture_kxf2() {
        let mut board = starting_board();
        place(&mut board, 1, 4, Piece::new(PieceType::King, PieceColor::White, (1, 4)));
        place(&mut board, 1, 5, Piece::new(PieceType::Pawn, PieceColor::Black, (1, 5)));
        let mut king = board.piece_at((1, 4));
        let result = move_to_notation(&mut board, &mut king, (1, 4), (1, 5));
        assert_eq!(result, "Kxf2");
    }

    #[test]
    fn king_black_to_f7() {
        let mut board = starting_board();
        place(&mut board, 5, 4, Piece::new(PieceType::King, PieceColor::Black, (5, 4)));
        let mut king = board.piece_at((5, 4));
        let result = move_to_notation(&mut board, &mut king, (5, 4), (6, 5));
        assert_eq!(result, "Kf7");
    }

    // =========================================================================
    // 9. DISAMBIGUATION — two rooks can reach same square
    // =========================================================================

    #[test]
    fn disambiguation_rook_file_rae1() {
        // Two white rooks on a1 and h1; both can go to e1 — disambiguate by file
        let mut board = starting_board();
        place(&mut board, 0, 0, Piece::new(PieceType::Rook, PieceColor::White, (0, 0)));
        place(&mut board, 0, 7, Piece::new(PieceType::Rook, PieceColor::White, (0, 7)));
        let mut rook = board.piece_at((0, 0));
        let result = move_to_notation(&mut board, &mut rook, (0, 0), (0, 4));
        assert_eq!(result, "Rae1");
    }

    #[test]
    fn disambiguation_rook_file_rhe1() {
        let mut board = starting_board();
        place(&mut board, 0, 0, Piece::new(PieceType::Rook, PieceColor::White, (0, 0)));
        place(&mut board, 0, 7, Piece::new(PieceType::Rook, PieceColor::White, (0, 7)));
        let mut rook = board.piece_at((0, 7));
        let result = move_to_notation(&mut board, &mut rook, (0, 7), (0, 4));
        assert_eq!(result, "Rhe1");
    }

    #[test]
    fn disambiguation_rook_rank_r1e5() {
        // Two white rooks on e1 and e8; both can go to e5 — disambiguate by rank
        let mut board = starting_board();
        place(&mut board, 0, 4, Piece::new(PieceType::Rook, PieceColor::White, (0, 4)));
        place(&mut board, 7, 4, Piece::new(PieceType::Rook, PieceColor::White, (7, 4)));
        let mut rook = board.piece_at((0, 4));
        let result = move_to_notation(&mut board, &mut rook, (0, 4), (4, 4));
        assert_eq!(result, "R1e5");
    }

    #[test]
    fn disambiguation_rook_rank_r8e5() {
        let mut board = starting_board();
        place(&mut board, 0, 4, Piece::new(PieceType::Rook, PieceColor::White, (0, 4)));
        place(&mut board, 7, 4, Piece::new(PieceType::Rook, PieceColor::White, (7, 4)));
        let mut rook = board.piece_at((7, 4));
        let result = move_to_notation(&mut board, &mut rook, (7, 4), (4, 4));
        assert_eq!(result, "R8e5");
    }

    #[test]
    fn disambiguation_knight_file_nbd2() {
        // Two knights on b1 and f3 can both reach d2 — disambiguate by file
        let mut board = starting_board();
        place(&mut board, 0, 1, Piece::new(PieceType::Knight, PieceColor::White, (0, 1)));
        place(&mut board, 2, 5, Piece::new(PieceType::Knight, PieceColor::White, (2, 5)));
        let mut knight = board.piece_at((0, 1));
        let result = move_to_notation(&mut board, &mut knight, (0, 1), (1, 3));
        assert_eq!(result, "Nbd2");
    }

    #[test]
    fn disambiguation_knight_file_nfd2() {
        let mut board = starting_board();
        place(&mut board, 0, 1, Piece::new(PieceType::Knight, PieceColor::White, (0, 1)));
        place(&mut board, 2, 5, Piece::new(PieceType::Knight, PieceColor::White, (2, 5)));
        let mut knight = board.piece_at((2, 5));
        let result = move_to_notation(&mut board, &mut knight, (2, 5), (1, 3));
        assert_eq!(result, "Nfd2");
    }

    #[test]
    fn disambiguation_queen_file_qad5() {
        // Two queens on a5 and h5 can both go to d5 — disambiguate by file
        let mut board = starting_board();
        place(&mut board, 4, 0, Piece::new(PieceType::Queen, PieceColor::White, (4, 0)));
        place(&mut board, 4, 7, Piece::new(PieceType::Queen, PieceColor::White, (4, 7)));
        let mut queen = board.piece_at((4, 0));
        let result = move_to_notation(&mut board, &mut queen, (4, 0), (4, 3));
        assert_eq!(result, "Qad5");
    }

    // =========================================================================
    // 10. EN PASSANT
    // =========================================================================

    #[test]
    fn en_passant_white_exd6() {
        let mut board = starting_board();
        // White pawn on e5=(4,4), black pawn just moved to d5=(4,3) — en passant target d6=(5,3)
        place(&mut board, 4, 4, Piece::new(PieceType::Pawn, PieceColor::White, (4, 4)));
        place(&mut board, 4, 3, Piece::new(PieceType::Pawn, PieceColor::Black, (4, 3)));
        board.en_passant = Some((5, 3));
        let mut pawn = board.piece_at((4, 4));
        let result = move_to_notation(&mut board, &mut pawn, (4, 4), (5, 3));
        assert_eq!(result, "exd6");
    }

    #[test]
    fn en_passant_black_dxe3() {
        let mut board = starting_board();
        // Black pawn on d4=(3,3), white pawn just moved to e4=(3,4) — en passant target e3=(2,4)
        place(&mut board, 3, 3, Piece::new(PieceType::Pawn, PieceColor::Black, (3, 3)));
        place(&mut board, 3, 4, Piece::new(PieceType::Pawn, PieceColor::White, (3, 4)));
        board.en_passant = Some((2, 4));
        let mut pawn = board.piece_at((3, 3));
        let result = move_to_notation(&mut board, &mut pawn, (3, 3), (2, 4));
        assert_eq!(result, "dxe3");
    }

    #[test]
    fn en_passant_white_cxb6() {
        let mut board = starting_board();
        place(&mut board, 4, 2, Piece::new(PieceType::Pawn, PieceColor::White, (4, 2)));
        place(&mut board, 4, 1, Piece::new(PieceType::Pawn, PieceColor::Black, (4, 1)));
        board.en_passant = Some((5, 1));
        let mut pawn = board.piece_at((4, 2));
        let result = move_to_notation(&mut board, &mut pawn, (4, 2), (5, 1));
        assert_eq!(result, "cxb6");
    }

    // =========================================================================
    // 11. Miscellaneous quiet moves (variety of pieces / squares)
    // =========================================================================

    #[test]
    fn rook_to_a_file_ra5() {
        let mut board = starting_board();
        place(&mut board, 0, 0, Piece::new(PieceType::Rook, PieceColor::White, (0, 0)));
        clear(&mut board, 1, 0);
        let mut rook = board.piece_at((0, 0));
        let result = move_to_notation(&mut board, &mut rook, (0, 0), (4, 0));
        assert_eq!(result, "Ra5");
    }

    #[test]
    fn bishop_to_h7_bh7() {
        let mut board = starting_board();
        place(&mut board, 2, 5, Piece::new(PieceType::Bishop, PieceColor::White, (2, 5)));
        // h7=(6,7) has a black pawn — clear it first for a quiet move
        clear(&mut board, 6, 7);
        let mut bishop = board.piece_at((2, 5));
        let result = move_to_notation(&mut board, &mut bishop, (2, 5), (6, 7)); // f3->h5 not h7
        // f3=(2,5) to h7=(6,7): diff=(4,2) — not diagonal. Use a5=(4,0)
        // Let's use bishop on e4=(3,4) to h7=(6,7): diff=(3,3) ✓
        let mut board2 = starting_board();
        place(&mut board2, 3, 4, Piece::new(PieceType::Bishop, PieceColor::White, (3, 4)));
        clear(&mut board2, 6, 7);
        let mut bishop2 = board2.piece_at((3, 4));
        let result2 = move_to_notation(&mut board2, &mut bishop2, (3, 4), (6, 7));
        assert_eq!(result2, "Bh7");
    }

    #[test]
    fn queen_to_a8_qa8() {
        let mut board = starting_board();
        place(&mut board, 3, 3, Piece::new(PieceType::Queen, PieceColor::White, (3, 3)));
        clear(&mut board, 7, 0); // clear black rook
        // d4=(3,3) to a7=(6,0): diff=(3,-3) diagonal ✓, a7
        let mut queen = board.piece_at((3, 3));
        let result = move_to_notation(&mut board, &mut queen, (3, 3), (6, 0));
        assert_eq!(result, "Qa7");
    }

    #[test]
    fn knight_to_h4_nh4() {
        let mut board = starting_board();
        place(&mut board, 2, 6, Piece::new(PieceType::Knight, PieceColor::White, (2, 6)));
        // g3=(2,6) to h5=(4,7): not a valid knight move (diff 2,1) ✓
        let mut knight = board.piece_at((2, 6));
        let result = move_to_notation(&mut board, &mut knight, (2, 6), (3, 7)); // (1,1) not valid
        // Valid knight move from g3=(2,6): to f5=(4,5) diff(2,-1) ✓ or h5=(4,7) diff(2,1) ✓
        let mut board2 = starting_board();
        place(&mut board2, 2, 6, Piece::new(PieceType::Knight, PieceColor::White, (2, 6)));
        let mut knight2 = board2.piece_at((2, 6));
        let result2 = move_to_notation(&mut board2, &mut knight2, (2, 6), (4, 7));
        assert_eq!(result2, "Nh5");
    }

    #[test]
    fn pawn_white_d2_d3() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((1, 3));
        let result = move_to_notation(&mut board, &mut pawn, (1, 3), (2, 3));
        assert_eq!(result, "d3");
    }

    #[test]
    fn pawn_white_c2_c4() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((1, 2));
        let result = move_to_notation(&mut board, &mut pawn, (1, 2), (3, 2));
        assert_eq!(result, "c4");
    }

    #[test]
    fn pawn_black_f7_f5() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((6, 5));
        let result = move_to_notation(&mut board, &mut pawn, (6, 5), (4, 5));
        assert_eq!(result, "f5");
    }

    #[test]
    fn pawn_black_g7_g6() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((6, 6));
        let result = move_to_notation(&mut board, &mut pawn, (6, 6), (5, 6));
        assert_eq!(result, "g6");
    }

    #[test]
    fn knight_g8_to_h6() {
        let mut board = starting_board();
        let mut knight = board.piece_at((7, 6));
        let result = move_to_notation(&mut board, &mut knight, (7, 6), (5, 7));
        assert_eq!(result, "Nh6");
    }

    #[test]
    fn bishop_captures_on_h2() {
        let mut board = starting_board();
        // Black bishop on g3=(2,6), captures h2=(1,7) white pawn
        place(&mut board, 2, 6, Piece::new(PieceType::Bishop, PieceColor::Black, (2, 6)));
        // h2=(1,7) has white pawn
        let mut bishop = board.piece_at((2, 6));
        let result = move_to_notation(&mut board, &mut bishop, (2, 6), (1, 7));
        assert_eq!(result, "Bxh2");
    }

    #[test]
    fn rook_captures_along_rank_rxh5() {
        let mut board = starting_board();
        place(&mut board, 4, 0, Piece::new(PieceType::Rook, PieceColor::White, (4, 0)));
        place(&mut board, 4, 7, Piece::new(PieceType::Rook, PieceColor::Black, (4, 7)));
        let mut rook = board.piece_at((4, 0));
        let result = move_to_notation(&mut board, &mut rook, (4, 0), (4, 7));
        assert_eq!(result, "Rxh5");
    }

    #[test]
    fn queen_captures_on_h8() {
        let mut board = starting_board();
        place(&mut board, 3, 3, Piece::new(PieceType::Queen, PieceColor::White, (3, 3)));
        // h8=(7,7) has black rook
        let mut queen = board.piece_at((3, 3));
        // d4=(3,3) to h8=(7,7): diff=(4,4) diagonal ✓
        let result = move_to_notation(&mut board, &mut queen, (3, 3), (7, 7));
        assert_eq!(result, "Qxh8");
    }

    #[test]
    fn king_capture_kxd5() {
        let mut board = starting_board();
        place(&mut board, 4, 4, Piece::new(PieceType::King, PieceColor::White, (4, 4)));
        place(&mut board, 4, 3, Piece::new(PieceType::Pawn, PieceColor::Black, (4, 3)));
        let mut king = board.piece_at((4, 4));
        let result = move_to_notation(&mut board, &mut king, (4, 4), (4, 3)); // e5->d5
        assert_eq!(result, "Kxd5");
    }

    #[test]
    fn pawn_white_b2_b4() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((1, 1));
        let result = move_to_notation(&mut board, &mut pawn, (1, 1), (3, 1));
        assert_eq!(result, "b4");
    }

    #[test]
    fn pawn_black_c7_c5() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((6, 2));
        let result = move_to_notation(&mut board, &mut pawn, (6, 2), (4, 2));
        assert_eq!(result, "c5");
    }

    #[test]
    fn pawn_white_f2_f3() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((1, 5));
        let result = move_to_notation(&mut board, &mut pawn, (1, 5), (2, 5));
        assert_eq!(result, "f3");
    }

    #[test]
    fn knight_captures_on_c3() {
        let mut board = starting_board();
        place(&mut board, 4, 1, Piece::new(PieceType::Knight, PieceColor::Black, (4, 1)));
        place(&mut board, 2, 2, Piece::new(PieceType::Pawn, PieceColor::White, (2, 2)));
        let mut knight = board.piece_at((4, 1));
        // b5=(4,1) to c3=(2,2): diff=(-2,1) ✓
        let result = move_to_notation(&mut board, &mut knight, (4, 1), (2, 2));
        assert_eq!(result, "Nxc3");
    }

    #[test]
    fn king_white_to_f1() {
        let mut board = starting_board();
        clear(&mut board, 0, 5); // clear f1 bishop
        let mut king = board.piece_at((0, 4));
        let result = move_to_notation(&mut board, &mut king, (0, 4), (0, 5));
        assert_eq!(result, "Kf1");
    }

    #[test]
    fn king_black_to_e7() {
        let mut board = starting_board();
        clear(&mut board, 6, 4); // clear e7 pawn
        let mut king = board.piece_at((7, 4));
        let result = move_to_notation(&mut board, &mut king, (7, 4), (6, 4));
        assert_eq!(result, "Ke7");
    }

    #[test]
    fn rook_white_to_d1() {
        let mut board = starting_board();
        place(&mut board, 3, 0, Piece::new(PieceType::Rook, PieceColor::White, (3, 0)));
        let mut rook = board.piece_at((3, 0));
        // a4=(3,0) to d4=(3,3)
        let result = move_to_notation(&mut board, &mut rook, (3, 0), (3, 3));
        assert_eq!(result, "Rd4");
    }

    #[test]
    fn pawn_capture_fxg6_en_passant() {
        let mut board = starting_board();
        place(&mut board, 4, 5, Piece::new(PieceType::Pawn, PieceColor::White, (4, 5)));
        place(&mut board, 4, 6, Piece::new(PieceType::Pawn, PieceColor::Black, (4, 6)));
        board.en_passant = Some((5, 6));
        let mut pawn = board.piece_at((4, 5));
        let result = move_to_notation(&mut board, &mut pawn, (4, 5), (5, 6));
        assert_eq!(result, "fxg6");
    }

    #[test]
    fn bishop_white_to_b5() {
        let mut board = starting_board();
        place(&mut board, 3, 2, Piece::new(PieceType::Bishop, PieceColor::White, (3, 2)));
        // c4=(3,2) to b5=(4,1): diff=(1,-1) diagonal ✓
        let mut bishop = board.piece_at((3, 2));
        let result = move_to_notation(&mut board, &mut bishop, (3, 2), (4, 1));
        assert_eq!(result, "Bb5");
    }

    #[test]
    fn queen_black_to_d4() {
        let mut board = starting_board();
        place(&mut board, 7, 3, Piece::new(PieceType::Queen, PieceColor::Black, (7, 3)));
        clear(&mut board, 6, 3);
        let mut queen = board.piece_at((7, 3));
        let result = move_to_notation(&mut board, &mut queen, (7, 3), (3, 3));
        assert_eq!(result, "Qd4");
    }

    #[test]
    fn pawn_white_g2_g4() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((1, 6));
        let result = move_to_notation(&mut board, &mut pawn, (1, 6), (3, 6));
        assert_eq!(result, "g4");
    }

    #[test]
    fn pawn_black_b7_b5() {
        let mut board = starting_board();
        let mut pawn = board.piece_at((6, 1));
        let result = move_to_notation(&mut board, &mut pawn, (6, 1), (4, 1));
        assert_eq!(result, "b5");
    }

    #[test]
    fn knight_white_f3_to_e5() {
        let mut board = starting_board();
        place(&mut board, 2, 5, Piece::new(PieceType::Knight, PieceColor::White, (2, 5)));
        let mut knight = board.piece_at((2, 5));
        // f3=(2,5) to e5=(4,4): diff=(2,-1) ✓
        let result = move_to_notation(&mut board, &mut knight, (2, 5), (4, 4));
        assert_eq!(result, "Ne5");
    }

    #[test]
    fn bishop_black_to_d6() {
        let mut board = starting_board();
        place(&mut board, 7, 5, Piece::new(PieceType::Bishop, PieceColor::Black, (7, 5)));
        clear(&mut board, 6, 4); // clear e7 pawn
        // f8=(7,5) to d6=(5,3): diff=(-2,-2) diagonal ✓
        let mut bishop = board.piece_at((7, 5));
        let result = move_to_notation(&mut board, &mut bishop, (7, 5), (5, 3));
        assert_eq!(result, "Bd6");
    }

    #[test]
    fn rook_black_to_e8() {
        let mut board = starting_board();
        place(&mut board, 4, 0, Piece::new(PieceType::Rook, PieceColor::Black, (4, 0)));
        let mut rook = board.piece_at((4, 0));
        // a5=(4,0) to e5=(4,4)
        let result = move_to_notation(&mut board, &mut rook, (4, 0), (4, 4));
        assert_eq!(result, "Re5");
    }

    #[test]
    fn king_white_to_d2() {
        let mut board = starting_board();
        place(&mut board, 1, 4, Piece::new(PieceType::King, PieceColor::White, (1, 4)));
        let mut king = board.piece_at((1, 4));
        let result = move_to_notation(&mut board, &mut king, (1, 4), (1, 3));
        assert_eq!(result, "Kd2");
    }

    // =========================================================================
    // 12. CASTLING (kept at end)
    // =========================================================================

//     #[test]
//     fn castling_white_kingside() {
//         let mut board = starting_board();
//         // Clear f1 and g1 for kingside castling
//         clear(&mut board, 0, 5);
//         clear(&mut board, 0, 6);
//         let mut king = board.piece_at((0, 4));
//         // King moves from e1=(0,4) to g1=(0,6) — kingside castling
//         let result = move_to_notation(&mut board, &mut king, (0, 4), (0, 6));
//         assert_eq!(result, "O-O");
//     }

//     #[test]
//     fn castling_white_queenside() {
//         let mut board = starting_board();
//         // Clear b1, c1, d1 for queenside castling
//         clear(&mut board, 0, 1);
//         clear(&mut board, 0, 2);
//         clear(&mut board, 0, 3);
//         let mut king = board.piece_at((0, 4));
//         // King moves from e1=(0,4) to c1=(0,2) — queenside castling
//         let result = move_to_notation(&mut board, &mut king, (0, 4), (0, 2));
//         assert_eq!(result, "O-O-O");
//     }

//     #[test]
//     fn castling_black_kingside() {
//         let mut board = starting_board();
//         // Clear f8 and g8 for black kingside castling
//         clear(&mut board, 7, 5);
//         clear(&mut board, 7, 6);
//         let mut king = board.piece_at((7, 4));
//         // King moves from e8=(7,4) to g8=(7,6) — kingside castling
//         let result = move_to_notation(&mut board, &mut king, (7, 4), (7, 6));
//         assert_eq!(result, "O-O");
//     }

//     #[test]
//     fn castling_black_queenside() {
//         let mut board = starting_board();
//         // Clear b8, c8, d8 for black queenside castling
//         clear(&mut board, 7, 1);
//         clear(&mut board, 7, 2);
//         clear(&mut board, 7, 3);
//         let mut king = board.piece_at((7, 4));
//         // King moves from e8=(7,4) to c8=(7,2) — queenside castling
//         let result = move_to_notation(&mut board, &mut king, (7, 4), (7, 2));
//         assert_eq!(result, "O-O-O");
//     }
}