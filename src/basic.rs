use std::time::SystemTime;

pub trait Basic {
    type Style;
    type Seed;
    type Attachment;
    type Return;
    fn new(style: Self::Style) -> Self;
    fn srand(&mut self, seed: Self::Seed);
    fn lazy_srand(&mut self);
    fn get_rand(&mut self, attachment: Option<Self::Attachment>) -> Self::Return;
    fn lazy_rand(&mut self, min: i64, max: i64) -> i64;
    fn lazy_randf(&mut self, min: f64, max: f64) -> f64;
}

#[inline]
pub fn pmrand(seed: i64, a: i64) -> i64 {
    let m = 2147483647;
    let q = m / a;
    let r = m % a;
    let hi = seed / q;
    let lo = seed % q;
    let test = a * lo - r * hi;
    if test > 0 { test } else { test + m }
}

pub fn rand(seed: &mut i64) -> i64 {
    let foo = *seed * 1103515245 + 12345;
    if foo > i32::max_value() as i64 {
        *seed = foo - i32::max_value() as i64 + i32::min_value() as i64;
    } else {
        *seed = foo;
    }
    (foo >> 16) & 2147483647
}

pub fn time_get() -> i64 {
    let sys_time = SystemTime::now();
    let foo_string = format!("{:?}", sys_time);
    let mut seed: i64 = 0;
    let mut flag = false;
    for num in foo_string.chars() {
        match num {
            e @ '0'...'9' => {
                flag = true;
                seed = seed * 10 + (e as u8 - 48) as i64;
                if seed >= i32::max_value() as i64 {
                    break;
                }
            }
            _ => {
                if flag {
                    break;
                }
            }
        }
    }
    seed
}
