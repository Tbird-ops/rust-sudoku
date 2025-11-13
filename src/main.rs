pub mod board_ctrl;
mod validation;

use board_ctrl::*;
use validation::*;

fn main() {
    let board = read_board_file("board.txt");
    print_board(&board);
    print_one_row(&board, 3);
    println!("");
    print_one_col(&board, 1);
}
