# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
cargo build          # compile
cargo run            # run (currently just prints "Hello, world!")
cargo test           # run notation tests
cargo check          # test code is right without compiling
cargo test <name>    # run a single test, e.g. cargo test pawn_white_single_push_e2_e3
```

## Architecture

`cherish` is a Rust chess engine. There are no external dependencies.

### Coordinate system

`board[row][col]` — row 0 is white's back rank (rank 1), row 7 is black's back rank (rank 8). Col 0 is the a-file, col 7 is the h-file. Algebraic notation is derived via `to_algebraic(pos)` which maps `(row, col)` → `(rank_number, file_char)`.

### Module layout

- **`board.rs`** — core data structures: `BoardState`, `Piece`, `PieceColor`, `PieceType`, plus helpers `get_icon`, `get_row_num`, `get_row_char`, `to_algebraic`.
  - `BoardState` holds the 8×8 `board` array, `en_passant: Option<(i32,i32)>`, `to_move`, and `playing_pieces: Vec<Piece>` (a redundant but convenient flat list of all active pieces — must be kept in sync with `board`).
  - `Piece.castle_rights` starts `true` for Kings and Rooks and must be cleared on first move to enforce castling rights.

- **`piece_moves.rs`** — all move generation and notation logic.
  - `get_possible_moves` dispatches per `PieceType`; returns pseudo-legal moves (ignores pins/check).
  - `get_player_possible_moves` collects pseudo-legal moves for all pieces of a color into a `HashMap<pos, Vec<pos>>`.
  - `get_player_legal_moves` filters the above by calling `is_pinned` for each candidate move — it clones the board, makes the move, and checks whether the enemy's pseudo-legal moves attack the king.
  - `move_to_notation` converts a `(piece, new_pos)` pair to standard algebraic notation. It handles captures, promotions (always queen), en passant, castling, and file/rank disambiguation via `doubled_piece_attacks_vertical`, `doubled_piece_attacks_horizontal`, and `doubled_assymetrical_horses_attack`.

- **`game_controller.rs`** — stub module; `is_pinned` returns `false` and `is_legal` is empty. Real pin logic lives in `piece_moves.rs`.

- **`tests.rs`** — comprehensive `#[cfg(test)]` suite for `move_to_notation`, covering quiet moves, captures, promotions, en passant, and disambiguation. Castling tests are commented out pending a signature change.

### Key invariants

- Every occupied square in `board` must also appear in `playing_pieces`, and vice versa.
- Castling eligibility is tracked via `castle_rights: bool` on the `Piece`; it is checked in `get_possible_king_moves` but is not yet cleared when pieces move.
- `is_pinned` in `piece_moves.rs` creates a `BoardState::new()` and overwrites its `board` field — it does **not** copy `playing_pieces`, so the cloned state's `playing_pieces` reflects the initial position, not the current one. This is a known limitation relevant when the king position lookup falls back to `playing_pieces`.
