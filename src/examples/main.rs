extern crate xiangyun;

use xiangyun::{Rand, JumpStyle};

fn main() {
    let mut foo = Rand::new();
    for _ in 0..9 {
        foo.new_base();
    }
    foo.jump_style(JumpStyle::DoubleJump);
    println!("{}", foo.rand());
    foo.jump();
    println!("{}", foo.rand());
    foo.jump();
    println!("{}", foo.rand());
}
