use std::fmt::Display;

pub struct Sudoku(pub(crate) [[Field; 9]; 9]);

impl Sudoku {
    pub(crate) fn clone(&self) -> Sudoku {
        Sudoku(self.0.clone())
    }
}

impl Sudoku {
    pub(crate) fn default() -> Sudoku {
        Sudoku(Default::default())
    }
}

pub enum Field {
    Empty,
    Options(Vec<u8>),
    Filled(u8)
}

impl Default for Field {
    fn default() -> Self {
        Field::Empty
    }
}

impl Clone for Field {
    fn clone(&self) -> Self {
        match &self {
            Field::Empty => Field::Empty,
            Field::Options(v) => Field::Options(v.clone()),
            Field::Filled(v) => Field::Filled(*v)
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

impl Display for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "-------------------")?;
        for row in self.0.iter() {
            writeln!(f, "")?;
            for cell in row.iter() {
                write!(f, "|{}", cell)?;
            }
            write!(f, "|\n-------------------")?;
        }
        Ok(())
    }
}

impl PartialEq<u8> for Field {
    fn eq(&self, other: &u8) -> bool {
        match &self {
            Field::Empty => {false}
            Field::Options(_) => {false}
            Field::Filled(value) => {*value == *other}
        }
    }
}