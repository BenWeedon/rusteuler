use {crate::util, macros::answer};

#[answer(6857)]
pub fn run() -> Result<u64, String> {
    const N: u64 = 600851475143;
    for n in 1..N / 2 {
        if N % n == 0 {
            let factor = N / n;
            if util::is_prime(factor) {
                return Ok(factor);
            }
        }
    }
    Err(format!("Largest prime factor not found for number {}.", N))
}
