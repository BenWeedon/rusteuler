//! Utilities that can be shared across the solutions.

use std::convert::TryInto;

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
pub struct PrimeIterTrial {
    curr_prime: u64,
}
impl PrimeIterTrial {
    pub fn new() -> Self {
        Self { curr_prime: 1 }
    }
}
impl Default for PrimeIterTrial {
    fn default() -> Self {
        Self::new()
    }
}
impl Iterator for PrimeIterTrial {
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

/// An iterator which iterates over all primes under a given max by using the
/// Sieve of Eratosthenes: https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
pub struct PrimeIterEratosthenes {
    index: usize,
    table: Vec<bool>,
    first_run: bool,
}
impl PrimeIterEratosthenes {
    pub fn new(max_prime: usize) -> Self {
        Self {
            index: 0,
            table: vec![false; Self::p_to_i(max_prime) + 1],
            first_run: true,
        }
    }

    const fn p_to_i(prime: usize) -> usize {
        prime - 2
    }

    const fn i_to_p(index: usize) -> usize {
        index + 2
    }

    fn step_index(&mut self) -> Option<bool> {
        self.index += 1;
        if self.index == self.table.len() {
            None
        } else {
            Some(self.table[self.index])
        }
    }

    fn get_next_prime(&mut self) -> Option<usize> {
        if self.first_run {
            self.first_run = false;
            Some(2)
        } else {
            loop {
                match self.step_index() {
                    Some(is_composite) => {
                        if !is_composite {
                            break Some(Self::i_to_p(self.index));
                        }
                    }
                    None => break None,
                }
            }
        }
    }

    fn mark_multiples(&mut self) {
        let mut i = self.index;
        let initial_p = Self::i_to_p(i);
        while i < self.table.len() {
            self.table[i] = true;
            let p = Self::i_to_p(i);
            let new_p = p + initial_p;
            i = Self::p_to_i(new_p);
        }
    }
}
impl Iterator for PrimeIterEratosthenes {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.get_next_prime() {
            Some(p) => {
                self.mark_multiples();
                Some(p.try_into().unwrap())
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_test() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 2521, 6089, 7919];
        for n in primes {
            println!("{}", n);
            assert!(is_prime(n));
        }

        let non_primes = vec![0, 1, 4, 6, 8, 9, 27, 3973, 7249];
        for n in non_primes {
            println!("{}", n);
            assert!(!is_prime(n));
        }
    }

    #[test]
    fn is_palindromic_test() {
        let palindromes = vec![
            0, 1, 9, 11, 55, 101, 111, 232, 5555, 5005, 6116, 70207, 10001,
        ];
        for n in palindromes {
            println!("{}", n);
            assert!(is_palindromic(n));
        }

        let non_palindromes = vec![12, 56, 72, 122, 311, 123, 1234, 5455, 71207];
        for n in non_palindromes {
            println!("{}", n);
            assert!(!is_palindromic(n));
        }
    }

    fn prime_iter_test(iter: impl Iterator<Item = u64>) {
        const PRIMES: [u64; 15] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
        for (p1, p2) in iter.zip(&PRIMES) {
            println!("{}, {}", p1, p2);
            assert_eq!(p1, *p2);
        }
    }

    #[test]
    fn prime_iter_trial_test() {
        prime_iter_test(PrimeIterTrial::new());
    }

    #[test]
    fn prime_iter_eratosthenes_test() {
        prime_iter_test(PrimeIterEratosthenes::new(100));
        prime_iter_test(PrimeIterEratosthenes::new(10));
        prime_iter_test(PrimeIterEratosthenes::new(47));

        assert_eq!(PrimeIterEratosthenes::new(10).last().unwrap(), 7);
        assert_eq!(PrimeIterEratosthenes::new(47).last().unwrap(), 47);
        assert_eq!(PrimeIterEratosthenes::new(48).last().unwrap(), 47);
    }
}
