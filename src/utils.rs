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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn is_prime_works() {
        let primes = vec![2, 3, 5, 7, 11, 13, 17, 2521, 6089, 7919];
        for p in primes {
            assert!(is_prime(p));
        }

        let nonprimes = vec![0, 1, 4, 6, 8, 9, 27, 3973, 7249];
        for n in nonprimes {
            assert!(!is_prime(n));
        }
    }
}
