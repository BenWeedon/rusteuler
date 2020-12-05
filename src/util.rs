//! Utilities that can be shared across the solutions.

/// If `n` is less than or equal to 1, it isn't prime. If it's in the range
/// `(1, 3]`, it is prime. If it's divisible by 2 or 3, it's not prime.
///
/// Finally, if none of those checks tell us anything, we iterate from 5 to
/// `sqrt(n)` in increments of 6 (since the 2/3 divisibility rule mentioned
/// above means primes can only occur once every 6 numbers). If `n` is
/// divisible by the number, or it's divisible by the number plus 2, we return
/// false. If we make it through the whole iteration without finding any
/// divisors, we return true.
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        false
    } else if n <= 3 {
        true
    } else if n % 2 == 0 || n % 3 == 0 {
        false
    } else {
        let max = (n as f64).sqrt().floor() as u64;
        for i in (5..=max).step_by(6) {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
        }
        true
    }
}

/// We use log base 10 to determine the number of digits in the number. We then
/// iterate over the first half of the digits. We get the current digit as well
/// as the corresponding mirrored, digit, and check if they're equal.
pub fn is_palindromic(n: u64) -> bool {
    let n: f64 = n as f64;
    let num_digits = n.log10().floor() + 1.0;
    let middle = (num_digits / 2.0).floor() as u64;
    for i in 0..middle {
        let i = i as f64;
        let first_digit = ((n / 10_f64.powf(num_digits - 1.0 - i)) as u64 % 10) as u64;
        let last_digit = ((n / 10_f64.powf(i)) as u64 % 10) as u64;
        if first_digit != last_digit {
            return false;
        }
    }
    true
}

/// An iterator which iterates over all primes by simply incrementing an
/// internal count and calling [`is_prime`] to check if it should be returned.
pub struct PrimeIter {
    curr_prime: u64,
}
impl PrimeIter {
    pub fn new() -> PrimeIter {
        PrimeIter { curr_prime: 1 }
    }
}
impl Iterator for PrimeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while !is_prime(self.curr_prime) {
            self.curr_prime += 1;
        }
        let result = Some(self.curr_prime);
        self.curr_prime += 1;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_works() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 2521, 6089, 7919];
        for n in primes {
            assert!(is_prime(n));
        }

        let non_primes = vec![0, 1, 4, 6, 8, 9, 27, 3973, 7249];
        for n in non_primes {
            assert!(!is_prime(n));
        }
    }

    #[test]
    fn is_palindromic_works() {
        let palindromes = vec![
            0, 1, 9, 11, 55, 101, 111, 232, 5555, 5005, 6116, 70207, 10001,
        ];
        for n in palindromes {
            assert!(is_palindromic(n));
        }

        let non_palindromes = vec![12, 56, 72, 122, 311, 123, 1234, 5455, 71207];
        for n in non_palindromes {
            assert!(!is_palindromic(n));
        }
    }

    #[test]
    fn prime_iter_works() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        let p_iter = PrimeIter::new();
        for (p1, p2) in p_iter.zip(primes) {
            println!("{}, {}", p1, p2);
            assert_eq!(p1, p2);
        }
    }
}
