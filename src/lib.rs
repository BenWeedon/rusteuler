mod utils;

mod problem_1;
mod problem_2;
mod problem_3;

pub fn run_problem(problem_number: usize) -> Result<u64, String> {
    match problem_number {
        1 => problem_1::problem_1(),
        2 => problem_2::problem_2(),
        3 => problem_3::problem_3(),
        _ => Err(format!("{} is not a valid problem number.", problem_number)),
    }
}
