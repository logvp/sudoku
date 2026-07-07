use std::fs;
use std::path::{Path, PathBuf};

use clap::Parser;
use log::{error, trace};

use sudoku::Board;

/// Sudoku solver
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Input file to solve
    input: PathBuf,
}

fn read_board<P>(path: P) -> Option<Board>
where
    P: AsRef<Path>,
{
    let sudoku_str = match fs::read_to_string(path) {
        Ok(ok) => ok,
        Err(e) => todo!("io error: {}", e),
    };
    let parsed_digits = sudoku_str
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<u32>)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<_>, _>>();
    let parsed_digits = match parsed_digits {
        Ok(rows) => {
            if rows.len() != 9 {
                error!(
                    "Sudoku board expected 9 rows but found {} instead",
                    rows.len()
                );
                return None;
            }
            for row in rows.iter() {
                if row.len() != 9 {
                    error!(
                        "Sudoku board expected 9 columns but found {} instead",
                        rows.len()
                    );
                    return None;
                }
            }
            let board_digits: [u32; 9 * 9] = rows
                .into_iter()
                .flatten()
                .collect::<Vec<_>>()
                .try_into()
                .expect("Parsed board should be 9x9");
            board_digits
        }
        Err(e) => todo!("Could not parse digit {}", e),
    };
    Some(Board::make(parsed_digits))
}

fn main() {
    colog::basic_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    trace!("Parsing args...");
    let args = Args::parse();
    trace!("{:?}", args);

    let Some(board) = read_board(&args.input) else {
        error!("Could not read board from {}", args.input.display());
        return;
    };

    let solved = sudoku::solve(board).unwrap();
    println!("Solved!");
    solved.print();
}
