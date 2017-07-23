//! Xiangyun is the crate for random with Rust  

pub mod rand;
pub mod num;

pub use rand::{Rand, Style, JumpStyle};
pub use rand::base::{system, RAND_MAX};
pub use num::RandTrait;
