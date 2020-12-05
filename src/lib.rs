use macros::{declare_problem_mods, match_problems};

mod util;

declare_problem_mods!();

#[doc(hidden)]
pub fn run_problem(problem_number: usize) -> Result<u64, String> {
    match_problems!()
}
