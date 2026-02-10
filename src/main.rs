mod board_ctrl;
mod solver;
mod validation;

use board_ctrl::*;
use solver::*;
use validation::*;

// TODO Add board pencil, elim, write loop.
// Board should always be immutable within this function. Any board parameters should be refs
// Pencil will mark possibilities,
// Eliminate will try advanced strategies to remove possibilies (WIP)
// Write in will take all marks and board state, build new board, and transfer ownership to board var
// Then loop again to start over.
fn main() {
    println!("START");
    let board = read_board_file("board.txt");
    // println!("Board: {:?}", board);
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");
    print_board(&board);
    println!("Valid: {}", board_validation(&board));
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");

    println!("Starting to write new board");
    let pencil_marks = pencil_in(&board);
    let new_board = write_in(&board, &pencil_marks);
    // println!("Board: {:?}", new_board);
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");
    print_board(&new_board);
    println!("Valid: {}", board_validation(&new_board));
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");

    println!("Iteration 2");
    let pencil_marks_2 = pencil_in(&new_board);
    let new_board_2 = write_in(&new_board, &pencil_marks_2);
    // println!("Board: {:?}", new_board_2);
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");
    print_board(&new_board_2);
    println!("Valid: {}", board_validation(&new_board_2));
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");

    println!("Iteration 3");
    let pencil_marks_3 = pencil_in(&new_board_2);
    let new_board_3 = write_in(&new_board_2, &pencil_marks_3);
    // println!("Board: {:?}", new_board_3);
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");
    print_board(&new_board_3);
    println!("Valid: {}", board_validation(&new_board_3));
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");

    println!("Iteration 4");
    let pencil_marks_4 = pencil_in(&new_board_3);
    let new_board_4 = write_in(&new_board_3, &pencil_marks_4);
    // println!("Board: {:?}", new_board_4);
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");
    print_board(&new_board_4);
    println!("Valid: {}", board_validation(&new_board_4));
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");

    println!("Iteration 5");
    let pencil_marks_5 = pencil_in(&new_board_4);
    let new_board_5 = write_in(&new_board_4, &pencil_marks_5);
    // println!("Board: {:?}", new_board_5);
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");
    print_board(&new_board_5);
    println!("Valid: {}", board_validation(&new_board_5));
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");

    println!("Iteration 6");
    let pencil_marks_6 = pencil_in(&new_board_5);
    let new_board_6 = write_in(&new_board_5, &pencil_marks_6);
    // println!("Board: {:?}", new_board_6);
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");
    print_board(&new_board_6);
    println!("Valid: {}", board_validation(&new_board_6));
    println!("%%%%%%%%%%%%%%%%%%%%%%%%");

    let solved = read_board_file("board_solved.txt");
    print_board(&solved);
    println!("Valid: {}", board_validation(&solved));
}
