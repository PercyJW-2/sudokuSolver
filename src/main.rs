
use sudoku_definition::{Field, Sudoku};

mod sudoku_definition;

fn numbers_to_sudoku(numbers: [[u8; 9]; 9]) -> Sudoku {
    let mut sudoku = Sudoku::default();
    for row in 0..9 {
        for col in 0..9 {
            if numbers[row][col] == 0 {
                sudoku.0[row][col] = Field::Empty;
            } else { 
                sudoku.0[row][col] = Field::Filled(numbers[row][col]);
            }
        }
    }
    sudoku
}

fn build_initial_options(sudoku: &mut Sudoku) {
    // At the beginning, there is no Options in the Sudoku
    // Check rows
    for row in 0..9 {
        let mut row_options = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for col in 0..9 {
            match sudoku.0[row][col] {
                Field::Empty => {}
                Field::Options(_) => {}
                Field::Filled(value) => {row_options.retain(|x| *x != value);}
            }
        }
        for col in 0..9 {
            match &mut sudoku.0[row][col] {
                Field::Empty => {sudoku.0[row][col] = Field::Options(row_options.clone())},
                Field::Options(current_options) => {current_options.retain(|x| row_options.contains(x))},
                Field::Filled(_) => {}
            }
        }
    }
    // Check cols
    for col in 0..9 {
        let mut col_options = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for row in 0..9 {
            match sudoku.0[row][col] {
                Field::Empty => {}
                Field::Options(_) => {}
                Field::Filled(value) => {col_options.retain(|x| *x != value);}
            }
        }
        for row in 0..9 {
            match &mut sudoku.0[row][col] {
                Field::Empty => {sudoku.0[row][col] = Field::Options(col_options.clone());},
                Field::Options(current_options) => {current_options.retain(|x| col_options.contains(x));}
                Field::Filled(_) => {}
            }
        }
    }
    // Check squares
    for square_num in 0..9 {
        let row_start = (square_num / 3) * 3;
        let col_start = (square_num % 3) * 3;
        let mut square_options = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        for row in row_start..row_start + 3 {
            for col in col_start..col_start + 3 {
                match sudoku.0[row][col] {
                    Field::Empty => {}
                    Field::Options(_) => {}
                    Field::Filled(value) => {square_options.retain(|x| *x != value);}
                }
            }
        }
        for row in row_start..row_start + 3 {
            for col in col_start..col_start + 3 {
                match &mut sudoku.0[row][col] {
                    Field::Empty => {sudoku.0[row][col] = Field::Options(square_options.clone());},
                    Field::Options(current_options) => {current_options.retain(|x| square_options.contains(x));}
                    Field::Filled(_) => {}
                }
            }
        }
    }
}

fn sudoku_options_solvable(sudoku: &Sudoku) -> bool {
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
    let current_value: u8;
    match sudoku.0[row][col] {
        Field::Empty | Field::Options(_) => return true,
        Field::Filled(val) => {current_value = val;},
    };
    for i in 0..9 {
        // check row
        if i != row {
            if sudoku.0[i][col] == current_value {
                return false;
            }
        }
        // check col
        if i != col { 
            if sudoku.0[row][i] == current_value {
                return false;
            }
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
        Field::Empty => { sudoku.0[row][col] = Field::Filled(new_value); }
        Field::Filled(_) => { return false; }
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
        if index != col {
            match &mut sudoku.0[row][index] {
                Field::Empty | Field::Filled(_) => {}
                Field::Options(options) => {
                    options.retain(|x| *x != new_value);
                    if options.is_empty() {
                        return false;
                    }
                }
            }
        }
        if index != row {
            match &mut sudoku.0[index][col] {
                Field::Empty | Field::Filled(_) => {}
                Field::Options(options) => {
                    options.retain(|x| *x != new_value);
                    if options.is_empty() {
                        return false;
                    }
                }
            }
        }
    }
    // update square
    let row_offset = (row / 3) * 3;
    let col_offset = (col / 3) * 3;
    for r in row_offset..row_offset + 3 {
        for c in col_offset..col_offset + 3 {
            match &mut sudoku.0[r][c] {
                Field::Empty | Field::Filled(_) => {}
                Field::Options(options) => {
                    options.retain(|x| *x != new_value);
                    if options.is_empty() {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn solve(mut sudoku: Sudoku) -> Option<Sudoku> {
    // find position with the fewest options
    let mut min_row = 0;
    let mut min_col = 0;
    let mut count = 9;
    let mut finished = true;
    for row in 0..9 {
        for col in 0..9 {
            match &sudoku.0[row][col] {
                Field::Empty | Field::Filled(_) => {}
                Field::Options(options) => {
                    finished = false;
                    if options.len() < count {
                        count = options.len();
                        min_row = row;
                        min_col = col;
                    }
                }
            }
        }
    }
    if finished {
        return Some(sudoku);
    }
    // find correct value
    match sudoku.0[min_row][min_col].clone() {
        Field::Empty | Field::Filled(_) => {}
        Field::Options(options) => {
            for option in options {
                let backup = sudoku.clone();
                if fill_field(&mut sudoku, min_row, min_col, option) {
                    return solve(sudoku);
                }
                sudoku = backup;
            }
        }
    }
    None
}

fn main() {
    let input = [
        [0, 0, 8, 7, 0, 0, 2, 4, 0],
        [3, 7, 0, 1, 0, 2, 8, 0, 0],
        [0, 0, 0, 6, 8, 0, 3, 0, 0],
        [0, 1, 0, 3, 0, 8, 0, 0, 5],
        [5, 0, 7, 0, 1, 0, 6, 0, 0],
        [8, 0, 9, 5, 0, 0, 0, 1, 0],
        [0, 0, 3, 8, 6, 0, 0, 0, 0],
        [0, 0, 2, 0, 4, 5, 0, 3, 0],
        [4, 9, 0, 2, 3, 0, 5, 0, 8],
    ];
    let mut field = numbers_to_sudoku(input);
    println!("{}", field);
    let valid = sudoku_options_solvable(&field);
    println!("Is Sudoku valid? {}", valid);
    build_initial_options(&mut field);
    match solve(field) {
        None => {println!("Could not solve");}
        Some(solved) => {
            let valid = sudoku_options_solvable(&solved);
            println!("Solution:\n{}", solved);
            println!("Valid: {}", valid);
        }
    };
}
