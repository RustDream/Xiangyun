pub mod rand;

#[macro_use]
pub mod num;

pub use rand::{Rand, Style, JumpStyle};
pub use rand::base::{get_sys_seed, refresh_sys_seed};
pub use num::RandTrait;