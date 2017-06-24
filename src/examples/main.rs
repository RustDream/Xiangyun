extern crate xiangyun;

use xiangyun::{Rand, Style, NRand, Basic};

fn main() {
    let mut foo = Rand::new(Style::Dalton);
    foo.lazy_srand();
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    println!("Dalton: {}", foo.lazy_randf(1.0, 100.0));
    let mut foo = NRand::new("Ryus");
    foo.lazy_srand();
    println!("Ryus: {}", foo.lazy_randf(1.0, 100.0));
	println!("Ryus: {}", foo.lazy_randf(1.0, 100.0));
}
