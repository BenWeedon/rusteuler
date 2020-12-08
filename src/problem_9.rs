//! > A Pythagorean triplet is a set of three natural numbers, `a < b < c`, for
//! which,
//! >
//! > <code>a<sup>2</sup> + b<sup>2</sup> = c<sup>2</sup></code>
//! >
//! > For example, <code>3<sup>2</sup> + 4<sup>2</sup> = 9 + 16 = 25 = 5<sup>2</sup></code>.
//! >
//! > There exists exactly one Pythagorean triplet for which `a + b + c =
//! 1000`. Find the product `abc`.
//!
//! We iterate over all values of <var>a</var> from 0 to 1000, covering all
//! natural numbers. Then, since <var>b</var> must be greater than <var>a</var>
//! and <var>a</var> plus <var>b</var> can't be more than 1000, we iterate
//! <var>b</var> from `a + 1` to `1000 - a`. Technically, we could also account
//! for <var>c</var> being bigger than <var>b</var> in this iteration and set
//! the top limit even lower, but that isn't done here.
//!
//! Once we have values of <var>a</var> and <var>b</var>, we can subtract them
//! from 1000 to get <var>c</var>. Then it's just a matter of checking that
//! <var>c</var> is greater than <var>b</var> and that they're a Pythagorean
//! triplet, and we can break out of all the loops with the solution.
//!
//! ```
//! # rusteuler::framework::run_solution(9, || {
//! const N: u64 = 1000;
//!
//! let mut solution = 0;
//! 'outer: for a in 0..N {
//!     for b in a + 1..N - a {
//!         let c = N - a - b;
//!         if c > b && a.pow(2) + b.pow(2) == c.pow(2) {
//!             solution = a * b * c;
//!             break 'outer;
//!         }
//!     }
//! }
//!
//! assert_eq!(solution, 31875000);
//! # Ok(())
//! # }).unwrap();
//! ```
//!
//! ====TIME====
