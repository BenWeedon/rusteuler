#![doc(test(attr(deny(warnings))))]

use macros::declare_problem_mods;

pub mod framework;
pub mod util;

declare_problem_mods!();
