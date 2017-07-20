#[macro_use]
extern crate xiangyun;

use xiangyun::{Rand, RandTrait};

fn main() {
    randomize!(foo);
    println!("{}", foo());
    println!("{}", f64::rand()/f64::rand());
}
