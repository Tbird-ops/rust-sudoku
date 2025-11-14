use crate::board_ctrl::{get_board_box, get_board_col, get_board_row};
use std::collections::HashSet;

/// Validate the provided box for rules of sudoku.
/// All numbers appear once between 1-9
///
/// Parameters:
/// - `board: &[u8]` - A reference to the board vector
/// - `box_id: u8` - The box ID from the board
pub fn box_validation(board: &[u8], box_id: u8) -> bool {
    let box_to_check = get_board_box(board, box_id);
    let mut tracker: HashSet<u8> = HashSet::new();
    for cell in box_to_check.iter() {
        if *cell == 0 {
            continue;
        } else if !tracker.insert(*cell) {
            return false;
        }
    }
    true
}

/// Validate the provided row for rules of sudoku.
/// All numbers appear once between 1-9
///
/// Parameters:
/// - `board: &[u8]` - A reference to the board vector
/// - `row_id: u8` - The row ID requested (Y axis coordinate)
pub fn row_validation(board: &[u8], row_id: u8) -> bool {
    let row_to_check = get_board_row(board, row_id);
    let mut tracker: HashSet<u8> = HashSet::new();
    for cell in row_to_check.iter() {
        if *cell == 0 {
            continue;
        } else if !tracker.insert(*cell) {
            return false;
        }
    }
    true
}

/// Validate the provided col for rules of sudoku.
/// All numbers appear once between 1-9
///
/// Parameters:
/// - `board: &[u8]` - A reference to the board vector
/// - `col_id: u8` - The col ID requested (X axis coordinate)
pub fn col_validation(board: &[u8], col_id: u8) -> bool {
    let col_to_check = get_board_col(board, col_id);
    let mut tracker: HashSet<u8> = HashSet::new();
    for cell in col_to_check.iter() {
        if *cell == 0 {
            continue;
        } else if !tracker.insert(*cell) {
            return false;
        }
    }
    true
}

/// Validate the whole board by checking every box|row|col
///
/// parameters:
/// - `board: &[u8]` - a reference to the board vector
/// TODO add more verbose failure messages
pub fn board_validation(board: &[u8]) -> bool {
    for b in 0..9 {
        if !box_validation(board, b) {
            return false;
        }
    }
    for r in 0..9 {
        if !row_validation(board, r) {
            return false;
        }
    }
    for c in 0..9 {
        if !col_validation(board, c) {
            return false;
        }
    }
    true
}
