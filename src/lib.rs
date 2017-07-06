mod problem_1;
mod problem_2;

pub fn run_problem(problem_number: usize) -> Result<u32, String> {
    match problem_number {
        1 => problem_1::problem_1(),
        2 => problem_2::problem_2(),
        _ => Err(format!("{} is not a valid problem number.", problem_number)),
    }
}
