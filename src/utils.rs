pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn is_palindromic(n: u64) -> bool {
    let n: f64 = n as f64;
    let num_digits = n.log10().floor() + 1.0;
    let mut i = 0_f64;
    while i < num_digits / 2.0 {
        let first_digit = ((n / 10_f64.powf(num_digits - 1.0 - i)) as u64 % 10) as f64;
        let last_digit = ((n / 10_f64.powf(i)) as u64 % 10) as f64;
        if first_digit != last_digit {
            return false;
        }
        i += 1.0;
    }
    true
}

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
        loop {
            self.curr_prime += 1;
            if is_prime(self.curr_prime) {
                return Some(self.curr_prime);
            }
        }
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
            0, 1, 9, 11, 55, 101, 111, 232, 5555, 5005, 6116, 70207, 10001
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
