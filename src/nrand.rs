use std::cell::Cell;

use super::rand::{RandBasic, pmrand, time_get};

//pub const NRAND_MAX: u32 = 2147483647;

pub enum NRand {
    PMrand(i64),
    Ryus(i64),
    Lazy(i64),
}

fn basic(seed: i64) {
    let s = Cell::new(seed);
    s.set(pmrand(s.get(), 48271));
}

impl RandBasic for NRand {
    type Style = String;
    type Seed = i64;
    type Attachment = u32;
    type Return = i64;
    fn new(style: String) -> Self {
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
	}
    fn get_rand(&mut self, attachment: Option<u32>) -> i64 {
        match *self {
            NRand::PMrand(seed) => pmrand(seed, 48271),
            NRand::Ryus(seed) => {}
            NRand::Lazy(seed) => pmrand(seed, 16807),
        }
    }
    // fn lazy_rand(&mut self, min: i64, max: i64) -> i64;
    // fn lazy_randf(&mut self, min: f64, max: f64) -> f64;
}

trait Like {
    fn like(&self, p: &str) -> bool;
}

impl Like for String {
    fn like(&self, p: &str) -> bool {
        if self.to_lowercase() == p.to_lowercase().to_string() {
            true
        } else {
            false
        }
    }
}
