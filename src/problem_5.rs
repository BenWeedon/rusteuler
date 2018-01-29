pub fn problem_5() -> Result<u64, String> {
    #[allow(unreachable_code)]
    (20..)
        .find(|n| (2..21).all(|d| n % d == 0))
        .ok_or(String::from("This should never be reached."))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let solution = problem_5();
        let solution = solution.unwrap_or_else(|err| panic!("{}", err));
        assert_eq!(232792560, solution);
    }
}
