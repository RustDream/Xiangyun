use std::time::SystemTime;

pub trait RandBasic {
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


const PI: f64 = 3.141592654;
pub const RAND_MAX: f64 = 2147483647.0;

/// #Example
///
/// ```Rust
/// extern crate xiangyun;
///
/// use xiangyun::{Rand, Style};
/// use std::{thread, time};
///
/// fn main() {
///     let millis = time::Duration::from_millis(1000);
///     let mut foo = Rand::new(Style::Ryus);
///     foo.lazy_srand();
///     println!("Ryus: {}", foo.lazy_rand(1, 100));
///     thread::sleep(millis);
///     foo.lazy_srand();
///     println!("Ryus: {}", foo.lazy_rand(1, 100));
///     println!("-------------------------------------------------------");
///     let mut foo = Rand::new(Style::PMrand);
///     foo.lazy_srand();
///     println!("PMrand: {}", foo.lazy_rand(1, 100));
///     thread::sleep(millis);
///     foo.lazy_srand();
///     println!("PMrand: {}", foo.lazy_rand(1, 100));
/// }
/// ```
pub enum Style {
    PMrand,
    Gauss,
    BMrand,
    Marsaglia,
    Lazy,
    Crand,
    Ryus,
    Dalton,
}

pub struct Rand {
    style: Style,
    seed: i64,
    attachment: Vec<f64>,
}

impl RandBasic for Rand {
    type Style = Style;
    type Seed = i64;
    type Attachment = u64;
    type Return = f64;

    /// Is used to create a new random sequence core
    fn new(style: Style) -> Self {
        Rand {
            style: style,
            seed: 1,
            attachment: Vec::new(),
        }
    }

    /// Is used to generate a fixed sequence
    fn srand(&mut self, seed: i64) {
        self.seed = seed;
    }

    /// Is used to set random seed
    fn lazy_srand(&mut self) {
        let mut seed: i64 = time_get();
        if seed > 0 {
            match self.style {
                Style::Crand => self.seed = 1,
                _ => self.seed = seed,
            }
        } else {
            self.seed = 1;
        }
    }

    fn get_rand(&mut self, attachment: Option<u64>) -> f64 {
        let mut a = 25;
        match attachment {
            Some(e) => a = e,
            _ => {}
        }
        match self.style {
            Style::PMrand => {
                self.seed = pmrand(self.seed, a as i64);
                (self.seed as f64 / RAND_MAX).abs()
            }
            Style::Gauss => (Rand::gauss(&mut self.seed, a) / RAND_MAX).abs(),
            Style::BMrand => (Rand::bmrand(&mut self.seed, &mut self.attachment) / RAND_MAX).abs(),
            Style::Marsaglia => {
                (Rand::marsaglia(&mut self.seed, &mut self.attachment) / RAND_MAX).abs()
            }
            Style::Crand => (Rand::crand(&mut self.seed) as f64 / RAND_MAX).abs(),
            Style::Ryus => (Rand::ryus(&mut self.seed) as f64 / RAND_MAX).abs(),
            Style::Dalton => (Rand::dalton(&mut self.seed, a) / RAND_MAX).abs(),
            _ => {
                self.seed = Rand::lazy(self.seed);
                (self.seed as f64 / RAND_MAX).abs()
            }
        }
    }

    fn lazy_rand(&mut self, min: i64, max: i64) -> i64 {
        let gap = max - min + 1;
        match self.style {
            Style::PMrand => min + (gap as f64 * self.get_rand(Some(48271))) as i64,
            Style::Gauss | Style::Dalton => {
                min + (gap as f64 * self.get_rand(Some((gap as u64)))) as i64
            }
            _ => min + (gap as f64 * self.get_rand(None)) as i64,
        }
    }

    fn lazy_randf(&mut self, min: f64, max: f64) -> f64 {
        let gap = max - min + 1.0;
        match self.style {
            Style::PMrand => min + self.get_rand(Some(48271)) * gap,
            Style::Gauss | Style::Dalton => min + self.get_rand(Some((gap as u64))) * gap,
            _ => min + self.get_rand(None) * gap,
        }
    }
}

#[inline]
pub fn pmrand(seed: i64, a: i64) -> i64 {
    let m = RAND_MAX as i64;
    let q = m / a;
    let r = m % a;
    let hi = seed / q;
    let lo = seed % q;
    let test = a * lo - r * hi;
    if test > 0 { test } else { test + m }
}

#[inline]
fn basic(seed: &mut i64) -> i64 {
    *seed = pmrand(*seed, 48271);
    *seed
}

#[inline]
fn basicf(seed: &mut i64) -> f64 {
    basic(seed) as f64
}

impl Rand {
    fn gauss(seed: &mut i64, nsum: u64) -> f64 {
        let mut x = 0.0;
        for _ in 0..nsum {
            x += basicf(seed) / RAND_MAX;
        }
        x -= nsum as f64 / 2.0;
        x /= (nsum as f64 / 12.0).sqrt();
        x
    }

    fn bmrand(seed: &mut i64, attachment: &mut Vec<f64>) -> f64 {
        if attachment.len() == 0 {
            attachment.push(0.0);
            attachment.push(0.0);
            attachment.push(0.0);
        }
        attachment[2] = 1.0 - attachment[2];
        if attachment[2] != 0.0 {
            attachment[0] = (basicf(seed) + 1.0) / (RAND_MAX + 2.0);
            attachment[1] = basicf(seed) / (RAND_MAX + 1.0);
            (2.0 * PI * attachment[1]).sin() * (-2.0 * attachment[0].log(10.0)).sqrt()
        } else {
            (2.0 * PI * attachment[1]).cos() * (-2.0 * attachment[0].log(10.0)).sqrt()
        }
    }

    fn marsaglia(seed: &mut i64, attachment: &mut Vec<f64>) -> f64 {
        if attachment.len() == 0 {
            attachment.push(0.0);
            attachment.push(0.0);
            attachment.push(0.0);
            attachment.push(0.0);
        }
        attachment[3] = 1.0 - attachment[3];
        if attachment[3] != 0.0 {
            loop {
                let u1 = basicf(seed) / RAND_MAX;
                let u2 = basicf(seed) / RAND_MAX;
                attachment[0] = 2.0 * u1 - 1.0;
                attachment[1] = 2.0 * u2 - 1.0;
                attachment[2] = attachment[0] * attachment[0] + attachment[1] * attachment[1];
                if attachment[2] < 1.0 && attachment[2] != 0.0 {
                    break;
                }
            }
            attachment[0] * (-2.0 * attachment[2].log(10.0) / attachment[2])
        } else {
            attachment[1] * (-2.0 * attachment[2].log(10.0) / attachment[2])
        }
    }

    fn lazy(seed: i64) -> i64 {
        pmrand(seed, 16807)
    }

    fn crand(seed: &mut i64) -> i64 {
        *seed = *seed * 1103515245 + 12345;
        (*seed >> 16) & (RAND_MAX as i64)
    }

    fn ryus(seed: &mut i64) -> i64 {
        let i = basicf(seed);
        let j = basicf(seed);
        let k = (basicf(seed) / RAND_MAX).abs();
        (i * k + j * (1.0 - k)) as i64
    }

    fn dalton(seed: &mut i64, nsum: u64) -> f64 {
        let mut x = 0.0;
        for _ in 0..nsum {
            *seed = Rand::ryus(seed);
            x += *seed as f64 / RAND_MAX;
        }
        x -= nsum as f64 / 2.0;
        x /= (nsum as f64 / 12.0).sqrt();
        x
    }
}

/// A macro that generates random numbers
///
/// #Example
///
/// ```Rust
/// #[macro_use]
/// extern crate xiangyun;
/// use xiangyun::{Rand, Style};
/// fn main() {
///     println!("Style::Lazy");
///     let foo = rand!(2);
///     println!("{}, {}", foo[0], foo[1]);
///     println!("Style::PMrand");
///     let foo = rand!(Style::PMrand, 2);
///     println!("{}, {}", foo[0], foo[1]);
///     println!("Style::Gauss");
///     let foo = rand!(Style::Gauss, 2);
///     println!("{}, {}", foo[0], foo[1]);
/// }
/// ```
#[macro_export]
macro_rules! rand {
    () => {{
        let mut foo = Rand::new(Style::Lazy);
        foo.lazy_srand();
        foo.get_rand(None)
    }};
    ($num: expr) => {{
        let mut foo = Rand::new(Style::Lazy);
        let mut bar: Vec<f64> = Vec::new();
        foo.lazy_srand();
        for _ in 0..$num {
            bar.push(foo.get_rand(None));
        }
        bar
    }};
    ($style: path) => {{
        let mut foo = Rand::new($style);
        foo.lazy_srand();
        foo.get_rand(None)
    }};
    ($style: path, $num: expr) => {{
        let mut foo = Rand::new($style);
        let mut bar: Vec<f64> = Vec::new();
        foo.lazy_srand();
        for _ in 0..$num {
            bar.push(foo.get_rand(None));
        }
        bar
    }};
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
