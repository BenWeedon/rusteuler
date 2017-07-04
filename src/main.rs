extern crate rusteuler;

use std::env;
use std::io::prelude::*;
use std::process;
use std::time;

fn main() {
    // start timing performance
    let start_time = time::SystemTime::now();

    let mut args = env::args();
    args.next(); // drop the first argument
    let mut stderr = std::io::stderr();

    let problem_number = args.next().unwrap_or_else(|| {
        writeln!(&mut stderr, "You must provide a problem number to run on.")
            .expect("could not write to stderr");
        process::exit(1);
    });
    let problem_number = problem_number.parse().unwrap_or_else(|_| {
        writeln!(
            &mut stderr,
            "The value \"{}\" is not a valid number.",
            problem_number
        ).expect("could not write to stderr");
        process::exit(1);
    });

    match rusteuler::run_problem(problem_number) {
        Ok(answer) => println!("The solution is {}.", answer),
        Err(err) => {
            writeln!(
                &mut stderr,
                "An error was encountered running the problem: {}",
                err
            ).expect("could not write to stderr");
            process::exit(1);
        }
    }

    // display the elapsed time
    let elapsed_time = start_time.elapsed().unwrap_or_else(|err| {
        writeln!(&mut stderr, "Failed to get elapsed time: {}", err)
            .expect("could not write to stderr");
        process::exit(1);
    });
    println!(
        "Solution computed in {} seconds.",
        (elapsed_time.as_secs() as f64) + (elapsed_time.subsec_nanos() as f64 * 0.000000001)
    );
}
