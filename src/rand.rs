use std::time::SystemTime;

const PI: f64 = 3.141592654;

/// RAND_MAX is a const
/// Please don't assume that it is any value
pub const RAND_MAX: u32 = 32767;

pub enum Rand {
    Basic(u32),
    PMrand(u32, u32),
    Gauss(u32, u32),
    BMgauss(u32, f64, f64, bool),
    Marsaglia(u32, f64, f64, f64, bool),
}

impl Rand {
    /// a lazy way to get a random solver
    pub fn new() -> Self {
        Rand::Basic(time_get() as u32)
    }

    /// set seed of solver
    pub fn srand(&mut self, seed: u32) {
        match *self {
            Rand::Basic(_) => *self = Rand::Basic(seed),
            Rand::PMrand(_, a) => *self = Rand::PMrand(seed, a),
            Rand::Gauss(_, nsum) => *self = Rand::Gauss(seed, nsum),
            Rand::BMgauss(_, u, v, phase) => *self = Rand::BMgauss(seed, u, v, phase),
            Rand::Marsaglia(_, v1, v2, s, phase) => *self = Rand::Marsaglia(seed, v1, v2, s, phase),
        }
    }

    pub fn lazy_srand(&mut self) {
        self.srand(time_get() as u32);
    }

    /// get a random number
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
                let bar = bmgauss(&mut foo_seed, &mut foo_u, &mut foo_v, &mut foo_phase);
                *self = Rand::BMgauss(foo_seed, foo_u, foo_v, foo_phase);
                bar
            }
            Rand::Marsaglia(seed, v1, v2, s, phase) => {
                let mut foo_seed = seed;
                let mut foo_v1 = v1;
                let mut foo_v2 = v2;
                let mut foo_s = s;
                let mut foo_phase = phase;
                let bar = marsaglia(&mut foo_seed,
                                    &mut foo_v1,
                                    &mut foo_v2,
                                    &mut foo_s,
                                    &mut foo_phase);
                *self = Rand::Marsaglia(foo_seed, foo_v1, foo_v2, foo_s, foo_phase);
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

/// basic is a random function
/// # Describe Code
/// ```BASIC
/// function basic(seed as u32) as u32
///     seed = seed * 1103515245 + 12345
///     basic = seed >> 16 & RAND_MAX
/// end function
/// ```
fn basic(seed: &mut u32) -> u32 {
    *seed = (((*seed as u64 * 1103515245) as u32) as u64 + 12345) as u32;
    *seed >> 16 & RAND_MAX
}

/// pmrand is a random function by Park and Miller
/// # Describe Code
/// ```BASIC
/// function pmrand(seed as u32, byval a as u32) as u32
///     m = 2147483647
///     q = m / a
///     r = m mod as
///     hi = seed / q
///     lo = seed mod q
///     test = a * lo - r * hi
///     if test > 0 then
///         seed = test
///     else
///         seed = test + m
///     end if
///     pmrand = seed
/// end function
/// ```
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

/// gauss is a normal random function
/// # Describe Code
/// ```BASIC
/// function gauss(seed as u32, byval nsum as u32) as f64
///     x = 0
///     for i = 1 to nsum
///         x = x + basic(seed) / RAND_MAX
///     next i
///     x = x - nsum / 2
///     x = x / ((nsum / 12) ^ 0.5)
///     gauss = x
/// end function
/// ```
fn gauss(seed: &mut u32, nsum: u32) -> f64 {
    let mut x = 0.0;
    for _ in 0..nsum {
        x += basic(seed) as f64 / RAND_MAX as f64;
    }
    x -= nsum as f64 / 2.0;
    x /= (nsum as f64 / 12.0).sqrt();
    x
}

/// bmgauss is a normal random function by Box and Muller
/// # Describe Code
/// ```BASIC
/// function bmgauss(seed as u32, u as f64, v as f64, phase as bool) as f64
///     if phase then
///         phase = false
///         u = (basic(seed) + 1) / (RAND_MAX + 2)
///         v = basic(seed) / (RAND_MAX + 1)
///         bmgauss = sin(2 * PI * v)
///     else
///         phase = true
///         bmgauss = cos(2 * PI * v)
///     end if
///     bmgauss = bmgauss * (-2 * (log(u) ^ 0.5))
/// end function
/// ```
fn bmgauss(seed: &mut u32, u: &mut f64, v: &mut f64, phase: &mut bool) -> f64 {
    if *phase {
        *phase = false;
        *u = (basic(seed) + 1) as f64 / (RAND_MAX + 2) as f64;
        *v = basic(seed) as f64 / (RAND_MAX + 1) as f64;
        (-2.0 * (*u).log10()).sqrt() * (2.0 * PI * (*v)).sin()
    } else {
        *phase = true;
        (-2.0 * (*u).log10()).sqrt() * (2.0 * PI * (*v)).cos()
    }
}

/// marsaglia is a normal random function by Marsaglia
/// # Describe Code
/// ```BASIC
/// function marsaglia (seed as u32, v1 as f64, v2 as f64, s as f64, phase as bool) as f64
///     if phase then
///         phase = false
///         u1 = basic(seed) / RAND_MAX
///         u2 = basic(seed) / RAND_MAX
///         v1 = 2 * u1 - 1
///         v2 = 2 * u2 - 1
///         marsaglia = v1
///     else
///         phase = true
///         marsaglia = v2
///     end if
///     marsaglia = marsaglia * ((-2 * log(s) / s) ^ 0.5)
/// end function
/// ```
fn marsaglia(seed: &mut u32, v1: &mut f64, v2: &mut f64, s: &mut f64, phase: &mut bool) -> f64 {
    if *phase {
        *phase = false;
        let u1 = basic(seed) as f64 / RAND_MAX as f64;
        let u2 = basic(seed) as f64 / RAND_MAX as f64;
        *v1 = 2.0 * u1 - 1.0;
        *v2 = 2.0 * u2 - 1.0;
        *v1 * (-2.0 * (*s).log10() / (*s)).sqrt()
    } else {
        *phase = true;
        *v2 * (-2.0 * (*s).log10() / (*s)).sqrt()
    }
}
