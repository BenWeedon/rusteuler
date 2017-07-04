mod problem_1;

pub fn run_problem(problem_number: usize) -> Result<u32, String> {
    match problem_number {
        1 => problem_1::problem_1(),
        _ => Err(format!("{} is not a valid problem number.", problem_number)),

    }
}
