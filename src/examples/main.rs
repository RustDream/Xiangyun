extern crate xiangyun;

use xiangyun::rand::{Rand, Style};

fn main() {
    let mut foo = Rand::new(Style::Lazy);
    println!("Style::Lazy");
    println!("{}", foo.get_rand());
    println!("{}", foo.get_rand());
    let mut foo = Rand::new(Style::PMrand);
    println!("Style::PMrand");
    println!("{}", foo.get_rand());
    println!("{}", foo.get_rand());
    let mut foo = Rand::new(Style::Gauss);
    println!("Style::Gauss");
    println!("{}", foo.get_rand());
    println!("{}", foo.get_rand());
    let mut foo = Rand::new(Style::BMrand);
    println!("Style::BMrand");
    println!("{}", foo.get_rand());
    println!("{}", foo.get_rand());
}