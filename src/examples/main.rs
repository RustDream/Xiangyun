// cargo build --features "clippy"

extern crate xiangyun;

use xiangyun::{system, Chaos, XFloat};

fn main() {
    system::randomize();
    let mut foo = vec!["刘一", "陈二", "张三", "李四", "王五", "赵六", "孙七", "周八", "吴九", "郑十"];
    foo.chaos();
    println!("{:?}", foo);
    let mut bar = [1, 2, 3, 4, 5];
    bar.chaos();
    println!("{:?}", bar);
    let mut mystr = "测试字符串".to_string();
    mystr.chaos();
    println!("{:?}", mystr);
    println!("{}", 5.5.get_fractional_part());
    println!("{}", 5.5.get_fractional_digit());
}
