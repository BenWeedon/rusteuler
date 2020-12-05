//! > If we list all the natural numbers below 10 that are multiples of 3 or 5,
//! we get 3, 5, 6 and 9. The sum of these multiples is 23.
//! >
//! > Find the sum of all the multiples of 3 or 5 below 1000.
//!
//! This solution takes the brute force approach. In multiple threads, we
//! iterate over all numbers between 0 and 1000 and check which are multiples
//! of 3 or 5.
//!
//! ```
//! # let solution = rusteuler::problem_1::run().unwrap();
//! assert_eq!(solution, 233168);
//! ```

use std::{io::Write, sync::mpsc, thread};

#[doc(hidden)]
pub fn run() -> Result<u64, String> {
    const UPPER_BOUND: u64 = 1000;
    let (tx, rx) = mpsc::channel();

    let num_threads = num_cpus::get() as u64;
    for i in 0..num_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            let start = i * (UPPER_BOUND / num_threads);
            let end = if i == num_threads - 1 {
                // This is necessary to handle cases where UPPER_BOUND is not divisible by
                // num_threads.
                UPPER_BOUND
            } else {
                start + (UPPER_BOUND / num_threads)
            };
            for n in start..end {
                if (n % 3 == 0) || (n % 5 == 0) {
                    if let Err(err) = tx.send(n) {
                        let mut stderr = std::io::stderr();
                        writeln!(&mut stderr, "sending to channel failed: {}", err)
                            .expect("could not write to stderr");
                    }
                }
            }
        });
    }
    drop(tx);

    let mut sum = 0;
    for val in rx {
        sum += val;
    }
    Ok(sum)
}
