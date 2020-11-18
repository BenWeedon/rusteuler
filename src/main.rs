use std::env;
use std::process;
use std::time;

fn main() {
    // start timing performance
    let start_time = time::SystemTime::now();

    let mut args = env::args();
    args.next(); // drop the first argument

    let problem_number = args.next().unwrap_or_else(|| {
        eprintln!("You must provide a problem number to run on.");
        process::exit(1);
    });
    let problem_number = problem_number.parse().unwrap_or_else(|_| {
        eprintln!("The value \"{}\" is not a valid number.", problem_number);
        process::exit(1);
    });

    match rusteuler::run_problem(problem_number) {
        Ok(answer) => println!("The solution is {}.", answer),
        Err(err) => {
            eprintln!("An error was encountered running the problem: {}", err);
            process::exit(1);
        }
    }

    // display the elapsed time
    let elapsed_time = start_time.elapsed().unwrap_or_else(|err| {
        eprintln!("Failed to get elapsed time: {}", err);
        process::exit(1);
    });
    println!(
        "Solution computed in {} seconds.",
        (elapsed_time.as_secs() as f64) + (elapsed_time.subsec_nanos() as f64 * 0.000000001)
    );
}
