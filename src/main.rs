use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use log::{error, info, trace};

use sudoku::{BoardStatus, GameDescription, HumanSolver};

/// Sudoku solver
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
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

fn parse_game(sudoku_str: &str) -> Option<GameDescription> {
    match toml::from_str::<GameDescription>(sudoku_str) {
        Ok(game) => Some(game),
        Err(e) => {
            error!("Error parsing game: {}", e);
            None
        }
    }
}

fn read_game<P>(path: P) -> Option<GameDescription>
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
    parse_game(sudoku_str.as_str())
}

fn main() {
    colog::basic_builder()
        .filter_level(log::LevelFilter::Debug)
        .init();

    trace!("Parsing args...");
    let args = Args::parse();
    trace!("{:?}", args);

    match args.command {
        Commands::Check { input } => {
            let Some(game) = read_game(&input) else {
                error!("Could not read board from {}", input.display());
                return;
            };
            let (board, rules) = game.build();
            match sudoku::verify(board, Some(rules)) {
                BoardStatus::AlreadySolved => println!("Board is already solved"),
                BoardStatus::Unsolvable => println!("Board is unsolvable"),
                BoardStatus::OneSolution => println!("Board has one solution"),
                BoardStatus::MultipleSolutions => println!("Board has multiple solutions"),
            }
        }
        Commands::Solve { input } => {
            let Some(game) = read_game(&input) else {
                error!("Could not read board from {}", input.display());
                return;
            };
            let (board, rules) = game.build();
            let solved = sudoku::solve(board, Some(rules));
            if let Ok(solved) = solved {
                info!("Solved!");
                solved.print();
            } else {
                println!("Board is not solvable")
            }
        }
        Commands::Play { input } => {
            let Some(game) = read_game(&input) else {
                error!("Could not read board from {}", input.display());
                return;
            };
            let (board, rules) = game.build();
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
