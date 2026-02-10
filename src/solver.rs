use crate::board_ctrl::{
    get_board_box, get_board_col, get_board_row, get_box_id, get_cell_col, get_cell_row, write_cell,
};

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

/// Pencil In some possible values
/// Much like solving a newspaper problem, keep track of what can possibly fit in a cell
/// Use logical eliminations to identify values to play
///
/// Parameters:
/// - `board: &[u8]` - reference to board vector
pub fn pencil_in(board: &[u8]) -> Vec<u16> {
    let base: u16 = 2; // Working with binary flags
    let possible: u16 = base.pow(9) - 1; // Fill a bit flag of 0000 0001 1111 1111
    let mut penciled_values: Vec<u16> = Vec::new(); // Vector to store all possible states

    // Loop through the full board space
    for i in 0..board.len() {
        // println!("*********** NEW CELL **********");  //debug
        let cell = board.get(i).unwrap(); // Fetch cell
        if *cell != 0 {
            // If the cell is already filled, skip
            penciled_values.push(0);
        } else {
            // Otherwise, pencil in
            let mut pencil_marks = possible.clone();
            let c_box = get_board_box(board, get_box_id(i as u8));
            let c_row = get_board_row(board, get_cell_row(i as u8));
            let c_col = get_board_col(board, get_cell_col(i as u8));

            // Skip this step if the value is 0
            // All cells are assumed to have all possibilities
            // Remove marks for found values using bitwise operations
            // !(1 << box_cell -1) is used to get the inverse value of a given bit index
            // By &'ing the value against pencil marks, that possibility is deactivated
            // Ex: box_cell=3; 1111 1111 1111 1011 & 0000 0001 1111 1111 = 0000 0001 1111 1011
            for box_cell in c_box {
                if box_cell != 0 {
                    // println!("Found value {}", box_cell);
                    pencil_marks = !(1 << box_cell - 1) & pencil_marks;
                    // println!("New pencil marks: {:09b}", pencil_marks);
                }
            }
            for row_cell in c_row {
                if row_cell != 0 {
                    // println!("Found value {}", row_cell);
                    pencil_marks = !(1 << row_cell - 1) & pencil_marks;
                    // println!("New pencil marks: {:09b}", pencil_marks);
                }
            }
            for col_cell in c_col {
                if col_cell != 0 {
                    // println!("Found value {}", col_cell);
                    pencil_marks = !(1 << col_cell - 1) & pencil_marks;
                    // println!("New pencil marks: {:09b}", pencil_marks);
                }
            }
            penciled_values.push(pencil_marks);
        }
    }
    // Return vector of all cell possibilities
    penciled_values
}

//TODO Implement advanced solving techniques to minimize possible values
// May not need board, may just be possible given pencil marks. TBD. Returns new pencil marks
//pub fn eliminate(board: &[u8], pencil_values: &[u16]) -> Vec<u16> {}

/// Write in value
/// Evaluate current pencil marks, fill ones that have only a single value remaining
///
/// Parameters:
/// - `board: &[u8]` - reference to current board
/// - `pencil_values: &[u16]` - reference to current pencil marks
pub fn write_in(board: &[u8], pencil_values: &[u16]) -> Vec<u8> {
    let mut new_board: Vec<u8> = board.to_vec(); // Use a copy of the old board
    for i in 0..pencil_values.len() {
        // Loop over all cells
        let cell: &u16 = pencil_values.get(i).unwrap(); // Fetch a given cell pencil mark
        if *cell != 0 {
            // 0 indicates that this cell has no pencil marks
            // println!("CELLIDX: {}, PENMARK: {:09b}", i, cell); // DEBUG
            let mut bit: u16 = 1; // Bit map to use to identify which are set
            let mut valset: u16 = 0; // A tracker to see if a given cell will be written in
            while bit < 0b1000000000 {
                // While the bits are < 10 bit position, there are still flags
                if bit & *cell != 0 && valset == 0 {
                    // Found flag and valset has never been set
                    // println!("Possible fill in identified: {}", bit.ilog2());   // debug
                    valset = bit.ilog2() as u16 + 1; // Value to write is 1 more than the position identified by log_2
                } else if bit & *cell != 0 && valset != 0 {
                    // Found flag with valset. This cell has more than 1 value
                    // println!(                                                    //debug
                    //     "More than one possibility remains for this cell. {}",   // debug
                    //     bit.ilog2()                                              //debug
                    // );                                                           //debug
                    valset = 0xFFFF; // Set duplicate flag indicator
                }
                bit <<= 1; // Move up the bits
            }
            if valset != 0xFFFF {
                println!("W- idx: {}, val: {}", i, valset);
                new_board = write_cell(&new_board, i as u8, valset as u8);
            }
        }
    }
    new_board
}
