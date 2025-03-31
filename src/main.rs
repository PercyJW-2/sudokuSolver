use std::process::exit;
use clap::{Parser, Subcommand};
use sudoku_definition::Sudoku;

mod sudoku_definition;
mod sudoku_solving;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Provide Sudoku as an Argument
    Direct {
        /// Reads Sudoku in the following Format:
        ///   - Empty Fields are represented as an 0
        ///   - In the Order Left to Right without linebreaks every Number is written seperated by spaces
        ///   - Example: 1 2 3 4 5 6 7 8 9 0 ...
        raw_sudoku: Vec<u8>
    },
    /// Read Sudoku from file
    File {
        /// Path to file containing Sudoku
        /// The file has the following format:
        ///   - Empty Fields are represented as an 0
        ///   - The Numbers are written in a 9x9 Grid, seperated by spaces
        ///   - Example: 1 0 0 2 3 9 8 0 0
        ///              0 2 3 0 8 1 0 0 0
        ///              ...
        file_path: String,
    },
}

fn main() {
    let args = Args::parse();
    
    let input_result: Result<[[u8; 9]; 9], String> = match args.command {
        Commands::Direct { raw_sudoku } => {
            parse_raw_sudoku(raw_sudoku)
        },
        Commands::File { file_path } => {
            let contents = std::fs::read_to_string(file_path).expect("Something went wrong reading the file");
            let mut raw_sudoku = vec![];
            contents.split('\n').for_each(|line| {
                line.split(' ').for_each(|digit| {
                    if let Ok(digit) = digit.parse::<u8>() {
                        raw_sudoku.push(digit)
                    }
                })
            });
            parse_raw_sudoku(raw_sudoku)
        }
    };
    let mut field = Sudoku::new(match input_result {
        Ok(field) => field,
        Err(err) => {
            eprintln!("Failed to read sudoku:\n {}", err);
            exit(-1);
        }
    });
    println!("{}", field);
    let valid = sudoku_solving::sudoku_options_solvable(&field);
    println!("Is Sudoku valid? {}", valid);
    sudoku_solving::build_initial_options(&mut field);
    match sudoku_solving::solve(field) {
        None => {
            println!("Could not solve");
        }
        Some(solved) => {
            let valid = sudoku_solving::sudoku_options_solvable(&solved);
            println!("Solution:\n{}", solved);
            println!("Valid: {}", valid);
        }
    };
}

fn parse_raw_sudoku(raw_sudoku: Vec<u8>) -> Result<[[u8; 9]; 9], String> {
    if raw_sudoku.len() != 9 * 9 {
        if raw_sudoku.len() < 9 * 9 {
            Err(format!("Provided Sudoku is missing {} numbers", 9 * 9 - raw_sudoku.len()))
        } else {
            Err(format!("Provided Sudoku has too many numbers: {}", raw_sudoku.len()))
        }
    } else {
        let mut field = [[0; 9]; 9];
        let mut valid = true;
        for (index, &raw_field) in raw_sudoku.iter().enumerate() {
            if raw_field > 9 {
                valid = false;
                break;
            }
            field[index / 9][index % 9] = raw_field;
        }
        if valid {
            Ok(field)
        } else {
            Err("One Digit is too large".to_string())
        }
    }
}
