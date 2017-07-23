// cargo build --features "clippy"

extern crate xiangyun;

use xiangyun::{Rand, system};

fn main() {
    system::randomize();
    println!("{}", f64::rand());
    println!("{}", f64::rand());
    println!("{}", f64::rand());
    println!("{}", f64::rand());
}
