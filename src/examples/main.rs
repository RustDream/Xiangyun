extern crate xiangyun;

use xiangyun::rand::{Rand, Style};

fn main() {
    let mut foo = Rand::new(Style::Lazy);
    println!("{}", foo.get_rand());
    println!("{}", foo.get_rand());
	let mut foo = Rand::new(Style::PMrand);
	println!("{}", foo.get_rand());
    println!("{}", foo.get_rand());
}