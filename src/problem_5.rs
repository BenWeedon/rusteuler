use crate::util;

pub fn problem_5() -> Result<u64, String> {
    const K: u64 = 20;
    let limit = (K as f64).sqrt();
    Ok(util::PrimeIter::new()
        .take_while(|p| p <= &K)
        .map(|p| {
            let a = if p as f64 <= limit {
                (K as f64).log(p as f64).floor() as u64
            } else {
                1
            };
            p.pow(a as u32)
        })
        .product())
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
