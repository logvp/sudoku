use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use log::{error, info, trace};

use sudoku::{Board, BoardStatus, HumanSolver, Rules, RulesDescription};

/// Sudoku solver
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    #[arg(long)]
    rules_file: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Solve {
        /// Input file to solve
        input: PathBuf,
    },
    Check {
        /// Input file to count
        input: PathBuf,
    },
    Play {
        /// Input file to play
        input: PathBuf,
    },
}

fn parse_board(sudoku_str: &str) -> Option<Board> {
    let parsed_digits = sudoku_str
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(str::parse::<u32>)
                .collect::<Result<Vec<_>, _>>()
        })
        .collect::<Result<Vec<Vec<_>>, _>>();
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

fn read_board<P>(path: P) -> Option<Board>
where
    P: AsRef<Path>,
{
    let sudoku_str = match fs::read_to_string(path) {
        Ok(ok) => ok,
        Err(e) => {
            error!("IO error: {}", e);
            return None;
        }
    };
    parse_board(sudoku_str.as_str())
}

fn parse_rules(rules_str: &str) -> Option<Rules> {
    match serde_json::from_str::<RulesDescription>(rules_str) {
        Ok(rules) => Some(rules.build()),
        Err(e) => {
            error!("Error parsing rules: {}", e);
            None
        }
    }
}

fn read_rules<P>(path: P) -> Option<Rules>
where
    P: AsRef<Path>,
{
    let rules_str = match fs::read_to_string(path) {
        Ok(ok) => ok,
        Err(e) => {
            error!("IO error: {}", e);
            return None;
        }
    };
    parse_rules(rules_str.as_str())
}

fn main() {
    colog::basic_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    trace!("Parsing args...");
    let args = Args::parse();
    trace!("{:?}", args);

    let rules = args
        .rules_file
        .map(|rules_file| read_rules(rules_file).expect("Could not read rules file"));

    match args.command {
        Commands::Check { input } => {
            let Some(board) = read_board(&input) else {
                error!("Could not read board from {}", input.display());
                return;
            };
            match sudoku::verify(board, rules) {
                BoardStatus::AlreadySolved => println!("Board is already solved"),
                BoardStatus::Unsolvable => println!("Board is unsolvable"),
                BoardStatus::OneSolution => println!("Board has one solution"),
                BoardStatus::MultipleSolutions => println!("Board has multiple solutions"),
            }
        }
        Commands::Solve { input } => {
            let Some(board) = read_board(&input) else {
                error!("Could not read board from {}", input.display());
                return;
            };
            let solved = sudoku::solve(board, rules);
            if let Ok(solved) = solved {
                info!("Solved!");
                solved.print();
            } else {
                println!("Board is not solvable")
            }
        }
        Commands::Play { input } => {
            let Some(board) = read_board(&input) else {
                error!("Could not read board from {}", input.display());
                return;
            };
            let rules = rules.unwrap_or_else(Rules::standard_sudoku_rules);
            let mut solver = HumanSolver::default();
            let solved = sudoku::solve_with(board, rules, &mut solver);
            if let Ok(solved) = solved {
                info!("Solved!");
                solved.print();
            } else {
                println!("Board was not solved!")
            }
        }
    }
}
