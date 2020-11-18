use {crate::util, macros::answer};

#[answer(906609)]
pub fn run() -> Result<u64, String> {
    let mut largest = 0;
    for x in 100..1000 {
        for y in 100..1000 {
            let product = x * y;
            if product > largest && util::is_palindromic(product) {
                largest = product;
            }
        }
    }
    Ok(largest)
}
