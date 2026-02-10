# A simple sudoku solver

This is my first attempt to write a sudoku solver.
I found it an interesting way to learn and get familiar with Rust

## Repo structure

The file structure is largely broken up into these main categories:

- `main.rs`: implements the program loop to solve the board
- `board_ctrl.rs`: implements board interaction operations. Ideally use a function here to interact with the board to
  avoid custom board hack scripts
- `solver.rs`: My concept for solving a sudoku problem. The idea is to pencil in possibilities, and then write in values
  that are the only possibility
- `validation.rs`: A series of tests I wrote to ensure the board state between iterations had not become invalid

Additionally, a sample problem is provided. `board.txt` is the very start of the problem, `board_almost_solved.txt`
provides one missing value,
and finally `board_solved.txt` for sanity checking

