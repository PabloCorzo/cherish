# cherish

A chess engine and bot-vs-bot testbed written from scratch in Rust, with no external chess libraries. Board state is a set of `u64` bitboards (`src/bitboard.rs`); move generation, legality, and notation live in `src/piece_moves.rs`.

## Running

```
cargo run -- [flags]
```

### Flags

- `-h` — human plays (via stdin) instead of bot-vs-bot.
- `-960` — Chess960 (Fischer Random) starting position.
- `-speed` — "speed chess" mode: capturing a piece grants another turn.
- `-minlog` — log each game's moves in algebraic notation and print the record at the end.
- `-s` — print the board after every move (bot games only).
- `-n <num>` — play `<num>` head-to-head matches between bots instead of one game.
- `-learn` — bots query the Lichess opening explorer API to build/extend a local opening book (`opening_book.bin`) instead of playing purely from it. Requires a `LICHESS_TOKEN` env var (see `.env`).
- `-evaltrain` — skip play entirely and train the NNUE-style eval model from `~/Downoads/chessData.csv`, saving weights to `nnue_weights.safetensors`.

With no flags, two `BardBot`s play one game against each other.

## Bots

- `RandomBot` (`src/bots/randombot.rs`) — picks a uniformly random legal move.
- `BardBot` (`src/bots/bardbot.rs`) — plays from a locally stored opening book (`opening_book.bin`) for its first several moves, weighted by how often each move was recorded; falls back to `RandomBot` outside the book. In `-learn` mode it instead calls the Lichess masters explorer API to pick and record opening moves, rate-limited to avoid hammering the API. Midgame search is currently a stub (see Status below).

## Eval model

`src/bots/eval_model.rs` builds a small feed-forward net (768 → 256 → 32 → 1, one-hot piece/color/square input) intended as an NNUE-style position evaluator, trained on a Lichess evaluation CSV. It is trained via `-evaltrain` and currently loaded on `Game` construction, but not yet wired into move selection.

## Status

This is under active development; expect sharp edges. In particular, right now:

- Bot search beyond the opening book is not implemented — `BardBot` just picks randomly once it runs out of book moves.
- The eval model is trained and loadable but not yet consulted when choosing moves.
- `Game::new*()` requires `nnue_weights.safetensors` to exist on disk (via `-evaltrain`) even for modes that don't use it yet.
