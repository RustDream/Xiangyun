extern crate xiangyun;

use xiangyun::Rand;

fn main() {
    let mut foo = Rand::new_multibase(10);
    println!("{}", foo.multirand() as usize % 100 + 1);
    println!("{}", foo.multirand() as usize % 100 + 1);
    println!("{}", foo.multirand() as usize % 100 + 1);
}
