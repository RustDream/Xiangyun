extern crate xiangyun;

use xiangyun::Rand;

fn main() {
    let mut foo = Rand::new();
    println!("{}, {}, {}, {}, {}",
             foo.rand(),
             foo.rand(),
             foo.rand(),
             foo.rand(),
             foo.rand());
}
