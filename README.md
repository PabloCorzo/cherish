Personal implementation of a chess engine written fully in rust.

There are 2 versions as of now:

  -Standard version:
  More legible & abstracted code, can support a game and aim an aim training mode to learn the square names. code is on cherish_basic
  -Bit version:
  Uses i32 numbers as a bitboard to store boardstates. meant to be functionally equivalent but faster for move searching or general processing. code is on root src/
