use crate::sudoku_definition::{Field, Sudoku};

pub fn build_initial_options(sudoku: &mut Sudoku) {
    // At the beginning, there is no Options in the Sudoku
    // Check rows
    for i in 0..9 {
        let mut row_options = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut col_options = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for j in 0..9 {
            sudoku.gen_options_from_field(i, j, &mut row_options);
            sudoku.gen_options_from_field(j, i, &mut col_options);
        }
        for j in 0..9 {
            sudoku.update_options(i, j, &row_options);
            sudoku.update_options(j, i, &col_options);
        }
    }
    // Check squares
    for square_num in 0..9 {
        let row_start = (square_num / 3) * 3;
        let col_start = (square_num % 3) * 3;
        let mut square_options = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        for row in row_start..row_start + 3 {
            for col in col_start..col_start + 3 {
                sudoku.gen_options_from_field(row, col, &mut square_options);
            }
        }
        for row in row_start..row_start + 3 {
            for col in col_start..col_start + 3 {
                sudoku.update_options(row, col, &square_options);
            }
        }
    }
}

pub fn sudoku_options_solvable(sudoku: &Sudoku) -> bool {
    // check for every field if it is valid
    for row in 0..9 {
        for col in 0..9 {
            if !sudoku_field_valid(sudoku, row, col) {
                return false;
            }
        }
    }
    true
}

fn sudoku_field_valid(sudoku: &Sudoku, row: usize, col: usize) -> bool {
    let current_value = match sudoku.0[row][col] {
        Field::Empty | Field::Options(_) => return true,
        Field::Filled(val) => {
            val
        }
    };
    for i in 0..9 {
        // check row
        if i != row && sudoku.0[i][col] == current_value {
            return false;
        }
        // check col
        if i != col && sudoku.0[row][i] == current_value {
            return false;
        }
    }
    // check square
    let row_offset = (row / 3) * 3;
    let col_offset = (col / 3) * 3;
    for r in row_offset..row_offset + 3 {
        for c in col_offset..col_offset + 3 {
            if r == row && c == col {
                continue;
            }
            if sudoku.0[r][c] == current_value {
                return false;
            }
        }
    }
    true
}

fn fill_field(sudoku: &mut Sudoku, row: usize, col: usize, new_value: u8) -> bool {
    match &sudoku.0[row][col] {
        Field::Empty => {
            sudoku.0[row][col] = Field::Filled(new_value);
        }
        Field::Filled(_) => {
            return false;
        }
        Field::Options(options) => {
            if !options.contains(&new_value) {
                return false;
            }
            sudoku.0[row][col] = Field::Filled(new_value);
        }
    }
    // Update remaining options
    // update row and col
    for index in 0..9 {
        if !sudoku.update_existing_options(row, index, new_value)
            || !sudoku.update_existing_options(index, col, new_value)
        {
            return false;
        }
    }
    // update square
    let row_offset = (row / 3) * 3;
    let col_offset = (col / 3) * 3;
    for r in row_offset..row_offset + 3 {
        for c in col_offset..col_offset + 3 {
            if !sudoku.update_existing_options(r, c, new_value) {
                return false;
            }
        }
    }
    true
}

pub fn solve(mut sudoku: Sudoku) -> Option<Sudoku> {
    // find position with the fewest options
    let mut min_row = 0;
    let mut min_col = 0;
    let mut count = 9;
    let mut finished = true;
    for row in 0..9 {
        for col in 0..9 {
            if let Field::Options(options) = &sudoku.0[row][col] {
                finished = false;
                if options.len() < count {
                    count = options.len();
                    min_row = row;
                    min_col = col;
                }
            }
        }
    }
    if finished {
        return Some(sudoku);
    }
    // find correct value
    if let Field::Options(options) = sudoku.0[min_row][min_col].clone() {
        for option in options {
            let backup = sudoku.clone();
            if fill_field(&mut sudoku, min_row, min_col, option) {
                if let Some(sudoku) = solve(sudoku) { 
                    return Some(sudoku);
                }
            }
            sudoku = backup;
        }
    }
    None
}