extern crate xiangyun;

use xiangyun::Rand;

fn main() {
    let mut foo = Rand::Basic(1);
    println!("{}, {}, {}, {}, {}",
             foo.rand(),
             foo.rand(),
             foo.rand(),
             foo.rand(),
             foo.rand());
}
