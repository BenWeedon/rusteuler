extern crate rusteuler;

use std::env;
use std::io::prelude::*;
use std::process;

fn main() {
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
            "The value {} is not a valid number.",
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
            ).expect("could not write to stderr")
        }
    }
}
