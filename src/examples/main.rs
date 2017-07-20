#[macro_use]
extern crate xiangyun;

use xiangyun::{RandTrait};
use xiangyun::rand::base::{refresh_sys_seed, time_get};

fn main() {
    randomize!();
    println!("{}", f64::rand());
    println!("{}", f64::rand());
}
