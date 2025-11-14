use crate::board_ctrl::get_board_box;

/// Last free cell (3x3)
/// Finishes a cell by identifying the last missing value
/// Assumes the board state is valid
///
/// Parameters:
/// - `board: &[u8]` - Reference to board vector
/// - `box_id: u8` - Requested box ID from board to check for solution
pub fn last_free_cell(board: &[u8], box_id: u8) -> Option<Vec<u8>> {
    let box_to_check = get_board_box(board, box_id);
    let zeros = box_to_check.iter().filter(|&x| *x == 0).count();
    if zeros == 1 {
        let missing_val = (1u8..10u8).sum::<u8>() - box_to_check.iter().sum::<u8>();
        let idx = box_to_check.iter().position(|&x| x == 0).unwrap();
        let mut new_box = box_to_check.clone();
        new_box[idx] = missing_val;
        return Some(new_box);
    }
    None
}
