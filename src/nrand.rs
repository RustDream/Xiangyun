use super::rand::RandBasic;

//pub const NRAND_MAX: u32 = 2147483647;

pub enum NRand {
    PMrand(i64),
    Ryus(i64),
    Lazy(i64),
}

impl NRand {
    pub fn new(style: &str) -> Self {
        let s = style.to_lowercase();
        if s == "pmrand" {
            NRand::PMrand(1)
        } else if s == "ryus" {
            NRand::Ryus(1)
        } else {
            NRand::Lazy(1)
        }
    }

    pub fn srand(&mut self, seed: i64) {
        match *self {
            NRand::PMrand(_) => *self = NRand::PMrand(seed),
            NRand::Ryus(_) => *self = NRand::Ryus(seed),
            _ => *self = NRand::Lazy(seed),
        }
    }
    // fn lazy_srand(&mut self);
    pub fn get_rand(&mut self, attachment: Option<u32>) -> i64 {
        0
    }
    // fn lazy_rand(&mut self, min: i64, max: i64) -> i64;
    // fn lazy_randf(&mut self, min: f64, max: f64) -> f64;
}
