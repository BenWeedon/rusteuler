//! > In the 20×20 grid below, four numbers along a diagonal line have been
//! marked in red.
//! >
//! > 08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08 \
//! 49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00 \
//! 81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65 \
//! 52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91 \
//! 22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80 \
//! 24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50 \
//! 32 98 81 28 64 23 67 10 <span class="red">26</span> 38 40 67 59 54 70 66 18 38 64 70 \
//! 67 26 20 68 02 62 12 20 95 <span class="red">63</span> 94 39 63 08 40 91 66 49 94 21 \
//! 24 55 58 05 66 73 99 26 97 17 <span class="red">78</span> 78 96 83 14 88 34 89 63 72 \
//! 21 36 23 09 75 00 76 44 20 45 35 <span class="red">14</span> 00 61 33 97 34 31 33 95 \
//! 78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92 \
//! 16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57 \
//! 86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58 \
//! 19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40 \
//! 04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66 \
//! 88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69 \
//! 04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36 \
//! 20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16 \
//! 20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54 \
//! 01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
//! >
//! > The product of these numbers is 26 × 63 × 78 × 14 = 1788696.
//! >
//! > What is the greatest product of four adjacent numbers in the same
//! direction (up, down, left, right, or diagonally) in the 20×20 grid?
//!
//! I may have gone a little overboard with this solution, but it was fun. I
//! tried to use best practices throughout, even though it made the code
//! longer.
//!
//! We define a couple types. First is `Direction`, which enumerates all
//! possible directions to get the product starting at a particular cell. We
//! only have 4 directions rather than 8 because half the directions encompass
//! the same lines as the other half. Second is `GridProductIter`, which is an
//! [`Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
//! implementation which returns products sequentially.
//!
//! The grid is stored in a 1D array, and indexes are converted back-and-forth
//! to/from x-y coordinates. This is a trick that makes the data structure
//! itself simpler, as well as improving memory locality (in theory?).
//!
//! We change the iterator to the next state by cycling through the directions
//! and at the end moving to the next index. To get the product for a given
//! direction, we pass in helper lambdas which translate from one coordinate
//! pair to another with a given increment.
//!
//! We do checked subtraction at one point because we want to be able to check
//! if <var>x</var> or <var>y</var> is less than 0 to see if it's still in the
//! grid. However, unsigned integers can't go below 0. So, we do a checked
//! subtraction where a return value of
//! [`None`](https://doc.rust-lang.org/std/option/enum.Option.html) means it
//! underflowed.
//!
//! ```
//! # rusteuler::framework::run_solution(11, || {
//! const WIDTH: usize = 20;
//! const HEIGHT: usize = 20;
//! const GRID: [u64; 400] = [
//!     08, 02, 22, 97, 38, 15, 00, 40, 00, 75, 04, 05, 07, 78, 52, 12, 50, 77, 91, 08, 49, 49, 99,
//!     40, 17, 81, 18, 57, 60, 87, 17, 40, 98, 43, 69, 48, 04, 56, 62, 00, 81, 49, 31, 73, 55, 79,
//!     14, 29, 93, 71, 40, 67, 53, 88, 30, 03, 49, 13, 36, 65, 52, 70, 95, 23, 04, 60, 11, 42, 69,
//!     24, 68, 56, 01, 32, 56, 71, 37, 02, 36, 91, 22, 31, 16, 71, 51, 67, 63, 89, 41, 92, 36, 54,
//!     22, 40, 40, 28, 66, 33, 13, 80, 24, 47, 32, 60, 99, 03, 45, 02, 44, 75, 33, 53, 78, 36, 84,
//!     20, 35, 17, 12, 50, 32, 98, 81, 28, 64, 23, 67, 10, 26, 38, 40, 67, 59, 54, 70, 66, 18, 38,
//!     64, 70, 67, 26, 20, 68, 02, 62, 12, 20, 95, 63, 94, 39, 63, 08, 40, 91, 66, 49, 94, 21, 24,
//!     55, 58, 05, 66, 73, 99, 26, 97, 17, 78, 78, 96, 83, 14, 88, 34, 89, 63, 72, 21, 36, 23, 09,
//!     75, 00, 76, 44, 20, 45, 35, 14, 00, 61, 33, 97, 34, 31, 33, 95, 78, 17, 53, 28, 22, 75, 31,
//!     67, 15, 94, 03, 80, 04, 62, 16, 14, 09, 53, 56, 92, 16, 39, 05, 42, 96, 35, 31, 47, 55, 58,
//!     88, 24, 00, 17, 54, 24, 36, 29, 85, 57, 86, 56, 00, 48, 35, 71, 89, 07, 05, 44, 44, 37, 44,
//!     60, 21, 58, 51, 54, 17, 58, 19, 80, 81, 68, 05, 94, 47, 69, 28, 73, 92, 13, 86, 52, 17, 77,
//!     04, 89, 55, 40, 04, 52, 08, 83, 97, 35, 99, 16, 07, 97, 57, 32, 16, 26, 26, 79, 33, 27, 98,
//!     66, 88, 36, 68, 87, 57, 62, 20, 72, 03, 46, 33, 67, 46, 55, 12, 32, 63, 93, 53, 69, 04, 42,
//!     16, 73, 38, 25, 39, 11, 24, 94, 72, 18, 08, 46, 29, 32, 40, 62, 76, 36, 20, 69, 36, 41, 72,
//!     30, 23, 88, 34, 62, 99, 69, 82, 67, 59, 85, 74, 04, 36, 16, 20, 73, 35, 29, 78, 31, 90, 01,
//!     74, 31, 49, 71, 48, 86, 81, 16, 23, 57, 05, 54, 01, 70, 54, 71, 83, 51, 54, 69, 16, 92, 33,
//!     48, 61, 43, 52, 01, 89, 19, 67, 48,
//! ];
//!
//! #[derive(Clone, Copy)]
//! enum Direction {
//!     Up,
//!     UpRight,
//!     Right,
//!     DownRight,
//! }
//!
//! struct GridProductIter {
//!     index: usize,
//!     direction: Direction,
//! }
//! impl GridProductIter {
//!     fn new() -> Self {
//!         GridProductIter {
//!             index: 0,
//!             direction: Direction::Up,
//!         }
//!     }
//!     fn do_product<F: Fn(usize, usize, usize) -> Option<u64>>(&self, f: F) -> Option<u64> {
//!         let (x, y) = (self.index % WIDTH, self.index / WIDTH);
//!         let a = GRID[self.index];
//!         if let Some(b) = f(1, x, y) {
//!             if let Some(c) = f(2, x, y) {
//!                 if let Some(d) = f(3, x, y) {
//!                     return Some(a * b * c * d);
//!                 }
//!             }
//!         }
//!         None
//!     }
//!     fn get_at(&self, x: Option<usize>, y: Option<usize>) -> Option<u64> {
//!         // If x or y are None, they underflowed, meaning they're out of bounds.
//!         if let Some(x) = x {
//!             if let Some(y) = y {
//!                 if x >= WIDTH || y >= HEIGHT {
//!                     return None;
//!                 } else {
//!                     let index = x + y * WIDTH;
//!                     return Some(GRID[index]);
//!                 }
//!             }
//!         }
//!         None
//!     }
//!     fn calculate_product(&self) -> Option<u64> {
//!         match self.direction {
//!             Direction::Up => {
//!                 self.do_product(|inc, x, y| self.get_at(Some(x), y.checked_sub(inc)))
//!             }
//!             Direction::UpRight => {
//!                 self.do_product(|inc, x, y| self.get_at(Some(x + inc), y.checked_sub(inc)))
//!             }
//!             Direction::Right => {
//!                 self.do_product(|inc, x, y| self.get_at(Some(x + inc), Some(y)))
//!             }
//!             Direction::DownRight => {
//!                 self.do_product(|inc, x, y| self.get_at(Some(x + inc), Some(y + inc)))
//!             }
//!         }
//!     }
//!     fn next_state(&mut self) {
//!         match self.direction {
//!             Direction::Up => self.direction = Direction::UpRight,
//!             Direction::UpRight => self.direction = Direction::Right,
//!             Direction::Right => self.direction = Direction::DownRight,
//!             Direction::DownRight => {
//!                 self.direction = Direction::Up;
//!                 self.index += 1;
//!             }
//!         }
//!     }
//! }
//! impl Iterator for GridProductIter {
//!     type Item = u64;
//!     fn next(&mut self) -> Option<Self::Item> {
//!         let product = loop {
//!             if self.index == GRID.len() {
//!                 return None;
//!             } else if let Some(p) = self.calculate_product() {
//!                 break p;
//!             } else {
//!                 self.next_state();
//!             }
//!         };
//!         self.next_state();
//!         Some(product)
//!     }
//! }
//!
//! let solution = GridProductIter::new().max().unwrap();
//! assert_eq!(solution, 70600674);
//! # Ok(())
//! # }).unwrap();
//! ```
//!
//! ====TIME====
