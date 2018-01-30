use std::sync::{Arc, Mutex};
use std::thread;

pub fn problem_5() -> Result<u64, String> {
    let current_bound = Arc::new(Mutex::new(20));
    let lowest_result = Arc::new(Mutex::new(u64::max_value()));
    const NUM_THREADS: u64 = 4;
    let threads = (0..NUM_THREADS).map(|_| {
        let current_bound = Arc::clone(&current_bound);
        let lowest_result = Arc::clone(&lowest_result);
        thread::spawn(move || loop {
            let low_bound: u64;
            let high_bound: u64;
            {
                let mut bound = current_bound.lock().unwrap();
                low_bound = *bound;
                *bound += 10000;
                high_bound = *bound;
            }

            let opt = (low_bound..high_bound).find(|n| (2..21).all(|d| n % d == 0));
            {
                let mut lowest = lowest_result.lock().unwrap();
                match opt {
                    Some(v) => if v < *lowest {
                        *lowest = v;
                    },
                    None => (),
                }
                if *lowest < u64::max_value() {
                    break;
                }
            }
        })
    });

    for t in threads {
        t.join().unwrap();
    }
    let result = *lowest_result.lock().unwrap();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let solution = problem_5();
        let solution = solution.unwrap();
        assert_eq!(232792560, solution);
    }
}
