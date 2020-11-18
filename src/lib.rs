mod util;

use macros::get_problem_numbers;
use paste::paste;

macro_rules! declare_problem_mods {
    () => {
        get_problem_numbers!(declare_problem_mods!{1});
    };
    ($($n:literal),+) => {
        paste! {
            $(
                mod [<problem_ $n>];
            )+
        }
    };
}

macro_rules! match_problems {
    ($problem_number:expr, $($n:literal),+) => {
        paste! {
            match $problem_number {
                $(
                    $n => [<problem_ $n>]::run(),
                )+
                _ => Err(format!("{} is not a valid problem number.", $problem_number)),
            }
        }
    };
}

declare_problem_mods!();

pub fn run_problem(problem_number: usize) -> Result<u64, String> {
    get_problem_numbers!(match_problems!(problem_number, 1))
}
