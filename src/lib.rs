use macros::get_problem_numbers;
use paste::paste;

macro_rules! declare_mods {
    () => {
        get_problem_numbers!(declare_mods!);
    };
    ($($n:literal),+) => {
        $(
            paste! { mod [<problem_ $n>]; }
        )+
    };
}

declare_mods!();

mod util;

pub fn run_problem(problem_number: usize) -> Result<u64, String> {
    match problem_number {
        1 => problem_1::problem_1(),
        2 => problem_2::problem_2(),
        3 => problem_3::problem_3(),
        4 => problem_4::problem_4(),
        5 => problem_5::problem_5(),
        _ => Err(format!("{} is not a valid problem number.", problem_number)),
    }
}
