//! Xiangyun is the crate for random with Rust  

// TODO: remove clippy before publish 
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

pub mod rand;
pub mod num;

pub use rand::{Solver, Style, JumpStyle};
pub use rand::base::{system, RAND_MAX};
pub use num::Rand;
