mod board_ctrl;
mod solver;
mod validation;

use board_ctrl::*;
use solver::*;
use validation::*;

fn main() {
    let board = read_board_file("board_almost_solved.txt");
    println!("Board: {:?}", board);
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");
    print_board(&board);
    println!("Valid: {}", board_validation(&board));
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");
    let solved_box = last_free_cell(&board, 2).expect("No solution found");
    let mut new_board = board.clone();
    new_board[(2 * 9)..(2 * 9 + 9)].copy_from_slice(&solved_box);
    print_board(&new_board);
    println!("Valid: {}", board_validation(&board));
}
