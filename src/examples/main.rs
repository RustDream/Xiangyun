#[macro_use]
extern crate xiangyun;

use xiangyun::Rand;

fn main() {
    randomize!(foo);
    println!("{}", foo());
}
