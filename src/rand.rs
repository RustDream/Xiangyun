use std::time::SystemTime;

const PI: f64 = 3.141592654;
pub const RAND_MAX: u32 = 32767;

pub enum Rand {
    Basic(u32),
    PMrand(u32, u32),
    Gauss(u32, u32),
    BMgauss(u32, f64, f64, bool),
}

impl Rand {
    pub fn new() -> Self {
        Rand::Basic(time_get() as u32)
    }
    pub fn srand(&mut self, seed: u32) {
        match *self {
            Rand::Basic(_) => *self = Rand::Basic(seed),
            Rand::PMrand(_, a) => *self = Rand::PMrand(seed, a),
            Rand::Gauss(_, nsum) => *self = Rand::Gauss(seed, nsum),
            Rand::BMgauss(_, u, v, phase) => *self = Rand::BMgauss(seed, u, v, phase),
        }
    }
    pub fn rand(&mut self) -> f64 {
        match *self {
            Rand::Basic(seed) => {
                let mut foo = seed;
                let bar = basic(&mut foo);
                self.srand(foo);
                bar as f64
            }
            Rand::PMrand(seed, a) => {
                let mut foo = seed;
                let bar = pmrand(&mut foo, a);
                self.srand(foo);
                bar as f64
            }
            Rand::Gauss(seed, nsum) => {
                let mut foo = seed;
                let bar = gauss(&mut foo, nsum);
                self.srand(foo);
                bar
            }
            Rand::BMgauss(seed, u, v, phase) => {
                let mut foo_seed = seed;
                let mut foo_u = u;
                let mut foo_v = v;
                let mut foo_phase = phase;
                let bar = bmgauss(foo_seed, foo_u, foo_v, foo_phase);
                *self = Rand::BMgauss(foo_seed, foo_u, foo_v, foo_phase);
                bar
            }
        }
    }
}

fn time_get() -> i64 {
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

fn basic(seed: &mut u32) -> u32 {
    *seed = (((*seed as u64 * 1103515245) as u32) as u64 + 12345) as u32;
    *seed >> 16 & RAND_MAX
}

fn pmrand(seed: &mut u32, a: u32) -> u32 {
    let m: u64 = 2147483647;
    let q = m / a as u64;
    let r = m % a as u64;
    let hi = *seed as u64 / q;
    let lo = *seed as u64 % q;
    let test = a as u64 * lo - r * hi;
    if test > 0 {
        *seed = test as u32;
    } else {
        *seed = (test + m) as u32;
    }
    *seed
}

fn gauss(seed: &mut u32, nsum: u32) -> f64 {
    let mut x = 0.0;
    for _ in 0..nsum {
        x += basic(seed) as f64 / RAND_MAX as f64;
    }
    x -= nsum as f64 / 2.0;
    x /= (nsum as f64 / 12.0).sqrt();
    x
}

fn bmgauss(seed: &mut u32, u: &mut f64, v: &mut f64, phase: &mut bool) -> f64 {
    let mut z = 0.0;
    if phase {
        phase = false * u = (basic(seed) + 1) as f64 / (RAND_MAX + 2) as f64;
        *v = basic(seed) as f64 / (RAND_MAX + 1) as f64;
        (-2.0 * (*u).log()).sqrt() * (2.0 * PI * v).sin()
    } else {
        phase = true(-2.0 * (*u).log()).sqrt() * (2.0 * PI * v).cos()
    }
}
