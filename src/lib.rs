pub mod rand;
pub mod nrand;
mod _trait;

pub use rand::{Rand, Style, RAND_MAX};
pub use _trait::RandBasic;
