use std::fmt::Display;

#[derive(Default)]
pub struct Sudoku(pub(crate) [[Field; 9]; 9]);

impl Clone for Sudoku {
    fn clone(&self) -> Sudoku {
        Sudoku(self.0.clone())
    }
}


impl Sudoku {
    pub(crate) fn gen_options_from_field(&self, row: usize, col: usize, options: &mut Vec<u8>) {
        match self.0[row][col] {
            Field::Empty => {}
            Field::Options(_) => {}
            Field::Filled(value) => {
                options.retain(|&x| x != value);
            }
        }
    }

    pub(crate) fn update_options(&mut self, row: usize, col: usize, options: &[u8]) {
        match &mut self.0[row][col] {
            Field::Empty => {
                self.0[row][col] = Field::Options(options.to_vec());
            }
            Field::Options(current_options) => {
                current_options.retain(|x| options.contains(x));
            }
            Field::Filled(_) => {}
        }
    }

    pub(crate) fn update_existing_options(&mut self, row: usize, col: usize, number: u8) -> bool {
        match &mut self.0[row][col] {
            Field::Empty | Field::Filled(_) => {}
            Field::Options(options) => {
                options.retain(|&x| x != number);
                if options.is_empty() {
                    return false;
                }
            }
        }
        true
    }
}

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "-------------------")?;
        for row in self.0.iter() {
            writeln!(f)?;
            for cell in row.iter() {
                write!(f, "|{}", cell)?;
            }
            write!(f, "|\n-------------------")?;
        }
        Ok(())
    }
}

#[derive(Default)]
#[derive(Debug)]
pub enum Field {
    #[default]
    Empty,
    Options(Vec<u8>),
    Filled(u8),
}

impl Clone for Field {
    fn clone(&self) -> Self {
        match &self {
            Field::Empty => Field::Empty,
            Field::Options(v) => Field::Options(v.clone()),
            Field::Filled(v) => Field::Filled(*v),
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            Field::Empty => write!(f, " "),
            Field::Options(_) => write!(f, " "),
            Field::Filled(v) => write!(f, "{}", *v as i32),
        }
    }
}

impl PartialEq<u8> for Field {
    fn eq(&self, other: &u8) -> bool {
        match &self {
            Field::Empty => false,
            Field::Options(_) => false,
            Field::Filled(value) => *value == *other,
        }
    }
}
