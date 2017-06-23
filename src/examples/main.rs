extern crate xiangyun;

use xiangyun::{Rand, Style};
use std::{thread, time};

fn main() {
    let millis = time::Duration::from_millis(1000);
    let mut foo = Rand::new(Style::Ryus);
    foo.lazy_srand();
    println!("Ryus: {}", foo.lazy_rand(1, 100));
    thread::sleep(millis);
    foo.lazy_srand();
    println!("Ryus: {}", foo.lazy_rand(1, 100));
    println!("-------------------------------------------------------");
    let mut foo = Rand::new(Style::PMrand);
    foo.lazy_srand();
    println!("PMrand: {}", foo.lazy_rand(1, 100));
    thread::sleep(millis);
    foo.lazy_srand();
    println!("PMrand: {}", foo.lazy_rand(1, 100));
}
