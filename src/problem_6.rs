//! > The sum of the squares of the first ten natural numbers is,
//! >
//! > ![1 squared plus 2 squared plus ellipsis plus 10 squared equals
//! 385](problem_6_1.svg)
//! >
//! > The square of the sum of the first ten natural numbers is,
//! >
//! > ![left-parenthesis 1 plus 2 plus ellipsis plus 10 right-parenthesis
//! squared equals 55 squared equals 3025](problem_6_2.svg)
//! >
//! > Hence the difference between the sum of the squares of the first ten
//! natural numbers and the square of the sum is ![3025 minus 385 equals
//! 2640](problem_6_3.svg).
//! >
//! > Find the difference between the sum of the squares of the first one
//! hundred natural numbers and the square of the sum.
//!
//! For this problem, I simply calculate the sum of squares from 1 to 100 and
//! the square of the sum from 1 to 100, and subtract them.
//!
//! ```
//! # rusteuler::framework::run_solution(|| {
//! let sum_of_squares = (1..=100).map(|n: u64| n.pow(2)).sum::<u64>();
//! let square_of_sum = (1..=100).sum::<u64>().pow(2);
//! assert_eq!(square_of_sum - sum_of_squares, 25164150);
//! # Ok(6)
//! # }).unwrap();
//! ```
//!
//! ====TIME====
