#![deny(
    unsafe_code,
    clippy::correctness,
    clippy::suspicious,
    unused_must_use,
    unfulfilled_lint_expectations
)]
#![warn(clippy::complexity, clippy::perf, clippy::style)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::ignored_unit_patterns,
    clippy::match_same_arms,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::must_use_candidate,
    clippy::needless_continue,
    clippy::return_self_not_must_use,
    clippy::semicolon_if_nothing_returned,
    clippy::uninlined_format_args,
    clippy::wildcard_imports
)]

mod arbiter;
mod board;
mod rules;
mod solver;
mod util;

pub use arbiter::*;
pub use board::*;
pub use rules::*;
pub use solver::*;
pub use util::*;
