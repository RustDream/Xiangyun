use std::cell::Cell;

use super::basic::{Basic, pmrand, time_get};

pub const NRAND_MAX: u32 = 2147483647;

pub enum NRand {
    PMrand(i64),
    Ryus(i64),
    Lazy(i64),
}

fn basic(seed: &Cell<i64>) -> i64 {
	println!("{}", seed.get());
    while seed.get() > i32::max_value() as i64 {
        seed.set(seed.get() - i32::max_value() as i64);
    }
    let foo = seed.get() * 1103515245 + 12345;
    while seed.get() > i32::max_value() as i64 {
        seed.set(seed.get() - i32::max_value() as i64);
    }
    seed.set(foo);
	println!("{}", seed.get());
    (foo >> 16) & 2147483647
}

fn basicf(seed: &Cell<i64>) -> f64 {
    basic(seed) as f64
}

impl Basic for NRand {
    type Style = &'static str;
    type Seed = i64;
    type Attachment = u32;
    type Return = i64;
    fn new(style: &str) -> Self {
        if style.like("PMrand") {
            NRand::PMrand(1)
        } else if style.like("Ryus") {
            NRand::Ryus(1)
        } else {
            NRand::Lazy(1)
        }
    }

    fn srand(&mut self, seed: i64) {
        match *self {
            NRand::PMrand(_) => *self = NRand::PMrand(seed),
            NRand::Ryus(_) => *self = NRand::Ryus(seed),
            _ => *self = NRand::Lazy(seed),
        }
    }
    fn lazy_srand(&mut self) {
        let seed = time_get();
        self.srand(seed);
    }
    fn get_rand(&mut self, attachment: Option<u32>) -> i64 {
        match *self {
            NRand::PMrand(seed) => {
                let foo = pmrand(seed, 48271);
                NRand::PMrand(foo);
                foo
            }
            NRand::Ryus(seed) => NRand::ryus(seed), // FIXME
            NRand::Lazy(seed) => {
                let foo = pmrand(seed, 16807);
                NRand::PMrand(foo);
                foo
            }
        }
    }
    fn lazy_rand(&mut self, min: i64, max: i64) -> i64 {
        let gap = max - min + 1;
        self.get_rand(None) / NRAND_MAX as i64 * gap + min
    }
    fn lazy_randf(&mut self, min: f64, max: f64) -> f64 {
        let gap = max - min + 1.0;
        self.get_rand(None) as f64 / NRAND_MAX as f64 * gap + min
    }
}

impl NRand {
    fn ryus(seed: i64) -> i64 {
        let s = Cell::new(seed);
        let i = basicf(&s);
        let j = basicf(&s);
        let k = basicf(&s) / NRAND_MAX as f64;
        (i * k + j * (1.0 - k)) as i64
    }
}

trait Like {
    fn like(&self, p: &str) -> bool;
}

impl Like for str {
    fn like(&self, p: &str) -> bool {
        if self.to_lowercase() == p.to_lowercase() {
            true
        } else {
            false
        }
    }
}
