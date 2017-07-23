pub mod system;

use std::time::SystemTime;
use super::flag::Flag;

/// RAND_MAX is a const  
/// Please don't assume that it is any value  
pub const RAND_MAX: usize = 32767;

#[derive(Copy, Clone, Debug)]
pub struct BaseRand {
    seed: usize,
    function: Flag,
}

impl BaseRand {
    pub fn new() -> Self {
        BaseRand {
            seed: time_get(),
            function: Flag::On(0),
        }
    }

    pub fn srand(&mut self, seed: usize) {
        self.seed = seed;
    }

    pub fn set_function(&mut self, style: &str) {
        match style {
            "pmrand" => self.function = Flag::On(1),
            "basic" => self.function = Flag::On(0),
            _ => self.function = Flag::new(),
        }
    }

    pub fn lazy_srand(&mut self) {
        self.srand(time_get());
    }

    pub fn rand(&mut self) -> usize {
        let func = self.function;
        let mut _seed = self.seed;
        let mut _return = 0;
        match func {
            Flag::On(e) => {
                match e {    
                    0 => _return = basic(&mut _seed),
                    1 => _return = pmrand(&mut _seed),
                    _ => _return = lazy(&mut _seed),
                }
            }
            Flag::Off => _return = lazy(&mut _seed),
        }
        self.srand(_seed);
        _return
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
    let mut _seed = *seed as u64;
    let m: u64 = 2147483647;
    let q = m / a;
    let r = m % a;
    let hi = _seed / q;
    let lo = _seed % q;
    let test = a * lo - r * hi;
    if test > 0 {
        _seed = test;
    } else {
        _seed = test + m;
    }
    _seed %= RAND_MAX as u64;
    *seed = _seed as usize;
    _seed as usize
}

fn time_get() -> usize {
    let foo_string = format!("{:?}", SystemTime::now());
    let mut seed: usize = 0;
    let mut flag = Flag::new();
    for num in foo_string.chars() {
        match num {
            e @ '0'...'9' => {
                flag.on();
                seed = seed * 10 + (e as u8 - 48) as usize;
                if seed >= usize::max_value() / 10 - 10 {
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
