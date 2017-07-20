extern crate xiangyun;

use xiangyun::{RandTrait, randomize};

fn main() {
    randomize();
    println!("{}", f64::rand());
    println!("{}", f64::rand());
}
