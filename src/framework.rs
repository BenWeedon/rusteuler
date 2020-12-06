#![doc(hidden)]

use std::{error, fs::File, io::Write, time};

pub fn run_solution<S: FnOnce() -> Result<(), Box<dyn error::Error>>>(
    problem_number: u16,
    solution: S,
) -> Result<(), Box<dyn error::Error>> {
    // start timing performance
    let start_time = time::SystemTime::now();

    solution()?;

    // get the elapsed time
    let elapsed_time = start_time.elapsed()?;
    let elapsed_time_string = format!(
        "{:.10}",
        (elapsed_time.as_secs() as f64) + (elapsed_time.subsec_nanos() as f64 * 0.000000001)
    );

    let filename = format!("target/problem_{}", problem_number);
    let mut file = File::create(filename)?;
    file.write_all(elapsed_time_string.as_bytes())?;

    Ok(())
}
