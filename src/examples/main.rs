// cargo build --features "clippy"

extern crate xiangyun;

use xiangyun::{system, Chaos};

fn main() {
    system::randomize();
    let mut foo = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    foo.chaos();
    println!("{:?}", foo);
}
