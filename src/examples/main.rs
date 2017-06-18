#[macro_use]
extern crate xiangyun;

use xiangyun::{Rand, Style};

fn main() {
    println!("Style::Lazy");
    let foo = rand!(2);
	println!("{}, {}", foo[0], foo[1]);
    println!("Style::PMrand");
    let foo = rand!(Style::PMrand, 2);
	println!("{}, {}", foo[0], foo[1]);
    println!("Style::Gauss");
    let foo = rand!(Style::Gauss, 2);
	println!("{}, {}", foo[0], foo[1]);
    println!("Style::BMrand");
    let foo = rand!(Style::BMrand, 2);
	println!("{}, {}", foo[0], foo[1]);
    println!("Style::Marsaglia");
    let foo = rand!(Style::Marsaglia, 2);
	println!("{}, {}", foo[0], foo[1]);
    println!("Style::Crand");
    let foo = rand!(Style::Crand, 2);
	println!("{}, {}", foo[0], foo[1]);
}
