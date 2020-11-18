use macros::answer;

#[answer(4613732)]
pub fn run() -> Result<u64, String> {
    const MAX_VAL: u64 = 4_000_000;
    let mut n1 = 1;
    let mut n2 = 2;
    let mut sum = n2;
    loop {
        let n3 = n1 + n2;
        if n3 > MAX_VAL {
            break;
        }
        sum += if n3 % 2 == 0 { n3 } else { 0 };
        n1 = n2;
        n2 = n3;
    }
    Ok(sum)
}
