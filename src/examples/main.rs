extern crate xiangyun;

use xiangyun::{RandTrait, system};

fn main() {
    system::randomize();
    println!("{}", f64::rand());
    println!("{}", f64::rand());
    println!("{}", f64::rand());
    println!("{}", f64::rand());
}
