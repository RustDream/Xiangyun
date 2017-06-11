pub mod rand {
pub enum Style {
    PMrand,
    Lazy,
}

pub struct Rand {
    style: Style,
    seed: i64,
}

impl Rand {
    pub fn new(style: Style) -> Self {
        match style {
            Style::PMrand => {
                Rand {
                    style: Style::PMrand,
                    seed: 1,
                }
            }
            _ => {
                Rand {
                    style: Style::Lazy,
                    seed: 1,
                }
            }
        }
    }

    pub fn get_rand(&mut self) -> f64 {
        match self.style {
            Style::PMrand => {
                self.seed = Rand::pmrand(self.seed, 48271);
                self.seed as f64 / 2_147_483_647.0
            }
            _ => {
                self.seed = Rand::lazy(self.seed);
                self.seed as f64 / 2_147_483_647.0
            }
        }
    }

    pub fn pmrand(seed: i64, a: i64) -> i64 {
        let m = 2_147_483_647;
        let q = m / a;
        let r = m % a;
        let hi = seed / q;
        let lo = seed % q;
        let test = a * lo - r * hi;
        if test > 0 { test } else { test + m }
    }

    pub fn lazy(seed: i64) -> i64 {
        Rand::pmrand(seed, 16807)
    }
}
}