use std::time::SystemTime;
use super::flag::Flag;

/// RAND_MAX is a const  
/// Please don't assume that it is any value  
pub const RAND_MAX: usize = 32767;

static mut SYS_SEED: usize = 1;

pub fn get_sys_seed() -> usize {
    unsafe {
        SYS_SEED
    }
}

pub fn refresh_sys_seed(_seed: usize) {
    unsafe {
        SYS_SEED = _seed;
    }
}

pub struct BaseRand {
    seed: usize,
    // TODO: Use Flag instead fn(&mut usize) -> usize
    function: fn(&mut usize) -> usize,
}

impl BaseRand {
    pub fn new() -> Self {
        BaseRand {
            seed: time_get() as usize,
            function: basic,
        }
    }

    pub fn srand(&mut self, seed: usize) {
        self.seed = seed;
    }

    pub fn set_function(&mut self, style: &str) {
        match style {
            "pmrand" => self.function = pmrand,
            "basic" => self.function = basic,
            _ => self.function = lazy,
        }
    }

    pub fn lazy_srand(&mut self) {
        self.srand(time_get() as usize);
    }

    pub fn rand(&mut self) -> usize {
        let func = self.function;
        func(&mut self.seed)
    }

    pub fn get_seed(&self) -> usize {
        self.seed
    }
}

fn basic(seed: &mut usize) -> usize {
    let mut _seed = *seed as u32;
    _seed = (((_seed as u64 * 1103515245) as u32) as u64 + 12345) as u32;
    *seed = _seed as usize;
    (_seed as usize) >> 16 & RAND_MAX
}

fn lazy(seed: &mut usize) -> usize {
    _pmrand(seed, 16807)
}

fn pmrand(seed: &mut usize) -> usize {
    _pmrand(seed, 48271)
}

fn _pmrand(seed: &mut usize, a: u64) -> usize {
    let m: u64 = 2147483647;
    let q = m / a as u64;
    let r = m % a as u64;
    let hi = *seed as u64 / q;
    let lo = *seed as u64 % q;
    let test = a as u64 * lo - r * hi;
    if test > 0 {
        *seed = test as usize;
    } else {
        *seed = (test + m) as usize;
    }
    *seed
}

pub fn time_get() -> isize {
    let sys_time = SystemTime::now();
    let foo_string = format!("{:?}", sys_time);
    let mut seed: isize = 0;
    let mut flag = Flag::new();
    for num in foo_string.chars() {
        match num {
            e @ '0'...'9' => {
                flag.on();
                seed = seed * 10 + (e as u8 - 48) as isize;
                if seed >= (isize::max_value() / 10 - 10) as isize {
                    break;
                }
            }
            _ => {
                if flag.is_on() {
                    break;
                }
            }
        }
    }
    seed
}