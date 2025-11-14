use std::fs;

/// # Terminology
/// - Box: the 3x3 square that makes subparts to the problem
/// - Row: A left to right reading of the board
/// - Col: A top-down reading of the board
/// - Cell: A given position in the board to fill
const TERMINOLOGY: () = ();

/// Parse board input file and create board vector
/// file should be in 'zorder' esque box orientation for input, not row input.
///
/// Expects the parameters:
/// - `filename: &str` - path to input file
pub fn read_board_file(filename: &str) -> Vec<u8> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut board: Vec<u8> = Vec::new();
    for c in contents.chars() {
        if c.is_ascii_digit() {
            board.push(c.to_digit(10).unwrap() as u8);
        }
    }
    board
}

/// Find box ID for given cell ID
///
/// Expects parameters:
/// - `cell_id: u8`  - index of a given cell in 1D array
///
/// | 0 | 1 | 2 |
///
/// | 3 | 4 | 5 |
///
/// | 6 | 7 | 8 |
pub fn get_box_id(cell_id: u8) -> u8 {
    cell_id / 9 // floored due to integer division
}

/// Find box row on board for a given box_id
///
/// Expects parameters:
/// - `box_id: u8`  - index of a given cell in 1D array
///
/// | 0 | 0 | 0 |
///
/// | 1 | 1 | 1 |
///
/// | 2 | 2 | 2 |
pub fn get_box_row(box_id: u8) -> u8 {
    box_id / 3 // floored due to integer division
}

/// Find box col on board for a given box_id
///
/// Expects parameters:
/// - `box_id: u8`  - index of a given cell in 1D array
///
/// | 0 | 1 | 2 |
///
/// | 0 | 1 | 2 |
///
/// | 0 | 1 | 2 |
pub fn get_box_col(box_id: u8) -> u8 {
    box_id % 3
}

/// Find the board row pertaining to a given cell
///
/// Expects parameters:
/// - `cell_id: u8`  - index of given cell
pub fn get_cell_row(cell_id: u8) -> u8 {
    let box_row = get_box_row(get_box_id(cell_id));
    let inner_box_row = cell_id % 9 / 3;
    let board_row = inner_box_row + box_row * 3;
    board_row
}

/// Find the board col pertaining to a given cell
///
/// Expects parameters:
/// - `cell_id: u8`  - index of given cell
pub fn get_cell_col(cell_id: u8) -> u8 {
    let box_col = get_box_col(get_box_id(cell_id));
    let inner_box_col = cell_id % 3;
    let board_col = inner_box_col + box_col * 3;
    board_col
}

/// Find row starting idx given any cell idx [0-81)
///
/// Expects parameters:
/// - `cell_id: u8`  - index of given cell
pub fn row_start_from_cid(cell_id: u8) -> u8 {
    let pos_in_box = cell_id % 9; // Mod 9 to identify which cell position in box
    let row_in_box = pos_in_box / 3; // Div 3 to floor to nearest multiple of 3
    let idx_row_box = row_in_box * 3; // Mul 3 to find first idx of row in box
    let box_row = get_box_row(get_box_id(cell_id)); // Find the box row that this cell is in
    let start_row_index = (box_row * 27) + idx_row_box; // Locate first idx position of row
    start_row_index
}

/// Find the row starting idx given a requested row id [0-9)
///
/// Expects parameters:
/// - `row_id: u8`  - index of given row (Not left cell)
pub fn row_start_from_idx(row_id: u8) -> u8 {
    if row_id < 3 {
        row_id * 3 // Each row is spaced by 3
    } else if row_id < 6 {
        (row_id % 3) * 3 + 27 // Subsequent box rows are spaced by 27
    } else {
        (row_id % 3) * 3 + 54
    }
}

/// Find col starting idx given any cell idx [0-81)
///
/// Expects parameters:
/// - `cell_id: u8`  - index of given cell
pub fn col_start_from_cid(cell_id: u8) -> u8 {
    let pos_in_box = cell_id % 9; // Mod 9 to identify cell position within box
    let col_in_box = pos_in_box % 3; // Mod 3 to identify the column within box
    let box_col = get_box_col(get_box_id(cell_id)); // Identify column of box
    let start_col_index = (box_col * 9) + col_in_box; // Find top cell index of column
    start_col_index
}

/// Find the col starting idx given a requested col idx [0-9)
///
/// Expects parameters:
/// - `col_id: u8`  - index of given column (Not top cell)
pub fn col_start_from_idx(col_id: u8) -> u8 {
    if col_id < 3 {
        col_id
    } else if col_id < 6 {
        (col_id % 3) + 9 // Each box column is spaced by 9
    } else {
        (col_id % 3) + 18
    }
}

/// Fetch a single box from board for further processing
///
/// expects the parameters:
/// - `board: &[u8]` - representing board in 1D array
/// - `box_id: u8` - the box ID from the board
pub fn get_board_box(board: &[u8], box_id: u8) -> Vec<u8> {
    let start_idx = (box_id * 9) as usize;
    let box_to_check = board[start_idx..start_idx + 9].to_vec();
    box_to_check
}

/// Fetch a single row from board for further processing
///
/// Expects the parameters:
/// - `board: &[u8]`  - representing board in 1D array
/// - `row_id: u8`  - representing the row ID requested (Y axis coordinate)
pub fn get_board_row(board: &[u8], row_id: u8) -> Vec<u8> {
    let mut row: Vec<u8> = Vec::new();
    let rci: usize = row_start_from_idx(row_id) as usize; // Must be cast to usize for indexing
    for b in 0..3 {
        for c in 0..3 {
            let cell = *board.get(rci + c + (b * 9)).unwrap();
            row.push(cell);
        }
    }
    row
}

/// Fetch a single col from board for further processing
///
/// Expects the parameters:
/// - `board: &[u8]`  - representing board in 1D array
/// - `col_id: u8`  - representing the col ID requested (X axis coordinate)
pub fn get_board_col(board: &[u8], col_id: u8) -> Vec<u8> {
    let mut col: Vec<u8> = Vec::new();
    let cci: usize = col_start_from_idx(col_id) as usize; // must be cast to usize for indexing
    for b in 0..3 {
        for c in 0..3 {
            let cell = *board.get(cci + (c * 3) + (b * 27)).unwrap();
            col.push(cell);
        }
    }
    col
}

/// Print row of board with coordinate marker
///
/// Expects the parameters:
/// - `board: &[u8]`  - representing board in 1D array
/// - `row_id: u8`  - Representing the row ID requested
pub fn print_one_row(board: &[u8], row_id: u8) {
    println!("Y X  0, 1, 2,  3, 4, 5,  6, 7, 8");
    print!("{}  ", row_id);
    let row = get_board_row(board, row_id);
    for i in 0..9 {
        let cell = if *row.get(i).unwrap() == 0 {
            ' '
        } else {
            char::from(*row.get(i).unwrap() + 0x30)
        };
        if i % 3 == 0 {
            print!("| {}, ", cell);
        } else if i % 3 == 1 {
            print!("{}, ", cell);
        } else if i % 3 == 2 {
            print!("{} ", cell);
        }
    }
    print!("|\n");
}

/// Print a single column of board with coordinate marker
///
/// Expects the parameters:
/// - `board: &[u8]`  - representing board in 1D array
/// - `col_id: u8`  - Representing the Idx of the column start (topmost position)
// Print a column, given the top number starting the column reading down
pub fn print_one_col(board: &[u8], col_id: u8) {
    println!("X Y  0, 1, 2,  3, 4, 5,  6, 7, 8");
    print!("{}  ", col_id);
    let col = get_board_col(board, col_id);
    for i in 0..9 {
        let cell = if *col.get(i).unwrap() == 0 {
            ' '
        } else {
            char::from(*col.get(i).unwrap() + 0x30)
        };
        if i % 3 == 0 {
            print!("| {}, ", cell);
        } else if i % 3 == 1 {
            print!("{}, ", cell);
        } else if i % 3 == 2 {
            print!("{} ", cell);
        }
    }
    print!("|\n");
}

/// Print row of board with coordinate marker
///
/// Expects the parameters:
/// - `board: &[u8]`  - representing board in 1D array
/// - `row_id: u8`  - Representing the Idx of the row start (leftmost position)
pub fn print_row(board: &[u8], row_id: u8) {
    let row = get_board_row(board, row_id);
    for i in 0..9 {
        let cell = if *row.get(i).unwrap() == 0 {
            ' '
        } else {
            char::from(*row.get(i).unwrap() + 0x30)
        };

        if i % 3 == 0 {
            print!("| {}, ", cell);
        } else if i % 3 == 1 {
            print!("{}, ", cell);
        } else if i % 3 == 2 {
            print!("{} ", cell);
        }
    }
    print!("|\n");
}

/// Print a single column of board with coordinate marker
///
/// Expects the parameters:
/// - `board: &[u8]`  - representing board in 1D array
/// - `colstart_cell_id: u8`  - Representing the Idx of the column start (topmost position)
// Print a column, given the top number starting the column reading down
pub fn print_col(board: &[u8], colstart_cell_id: u8) {
    let col = get_board_col(board, colstart_cell_id);
    for i in 0..9 {
        let cell = if *col.get(i).unwrap() == 0 {
            ' '
        } else {
            char::from(*col.get(i).unwrap() + 0x30)
        };
        if i % 3 == 0 {
            print!("| {}, ", cell);
        } else if i % 3 == 1 {
            print!("{}, ", cell);
        } else if i % 3 == 2 {
            print!("{} ", cell);
        }
    }
    print!("|\n");
}

/// Print the full Sudoku Board State
///
/// Expects the parameters:
/// - `board: &[u8]`  - representing board in 1D array
pub fn print_board(board: &[u8]) {
    println!(" X  0  1  2   3  4  5   6  7  8");
    print!("Y");
    for row_id in 0..9 {
        if row_id % 3 == 0 {
            println!(" -------------------------------");
        }
        print!("{} ", row_id);
        print_row(board, row_id);
    }
    println!("  -------------------------------");
}

//TODO Add some board generation capabilities instead of just reading from file for known good
