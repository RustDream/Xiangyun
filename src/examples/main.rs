extern crate xiangyun;

use xiangyun::{Rand, Style, RandBasic, NRand};

fn main() {
    let mut foo = Rand::new(Style::Dalton);
    foo.lazy_srand();
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    let mut foo = NRand::new("PMrand");
    println!("PMrand: {}", foo.lazy_randf(1.0, 100.0));
}
