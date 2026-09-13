use crate::{Action, Arbiter, Board, Digit, DigitPos, Solver};

#[derive(Default)]
pub struct HumanSolver {}
impl Solver for HumanSolver {
    fn make_move(&mut self, board: &Board, _rules: &Arbiter) -> Action {
        let mut buf = String::new();
        loop {
            println!("Board:");
            board.print();

            println!("Enter your move");
            println!("x y digit");
            buf.clear();
            std::io::stdin().read_line(&mut buf).unwrap();

            let nums: Result<Vec<usize>, _> =
                buf.split_whitespace().map(str::parse::<usize>).collect();
            let Ok(nums) = nums else {
                println!("Could not parse input");
                continue;
            };
            if nums.len() != 3 {
                println!("Invalid number of entries");
                continue;
            }
            let x = nums[0];
            let y = nums[1];
            let digit = match u32::try_from(nums[2]) {
                Ok(ok) => ok,
                Err(e) => {
                    println!("Could not parse digit: {}", e);
                    continue;
                }
            };
            let Ok(digit) = Digit::try_from(digit) else {
                println!("Could not parse digit");
                continue;
            };

            break Action::Set(vec![DigitPos { digit, x, y }]);
        }
    }
}
