use super::base::{BaseRand, RAND_MAX};
use std::f64::consts::PI;



pub enum Style {
    Normal,
    Gauss(u32),
    BMgauss(f64, f64, bool),
    Marsaglia(f64, f64, f64, bool),
}


pub struct Rand {
    base: BaseRand,
    style: Style,
}

impl Rand {
    /// a lazy way to get a random solver
    pub fn new() -> Self {
        Rand {
            base: BaseRand::new(),
            style: Style::Normal,
        }
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    /// get a random number
    pub fn rand(&mut self) -> usize {
        //self.rand(&mut self.seed)
        0
    }
}

fn gauss(base: &mut BaseRand, nsum: u32) -> f64 {
    let mut x = 0.0;
    for _ in 0..nsum {
        x += base.rand() as f64 / RAND_MAX as f64;
    }
    x -= nsum as f64 / 2.0;
    x /= (nsum as f64 / 12.0).sqrt();
    x
}

fn bmgauss(base: &mut BaseRand, u: &mut f64, v: &mut f64, phase: &mut bool) -> f64 {
    if *phase {
        *phase = false;
        *u = (base.rand() + 1) as f64 / (RAND_MAX + 2) as f64;
        *v = base.rand() as f64 / (RAND_MAX + 1) as f64;
        (-2.0 * (*u).log10()).sqrt() * (2.0 * PI * (*v)).sin()
    } else {
        *phase = true;
        (-2.0 * (*u).log10()).sqrt() * (2.0 * PI * (*v)).cos()
    }
}

fn marsaglia(base: &mut BaseRand, v1: &mut f64, v2: &mut f64, s: &mut f64, phase: &mut bool) -> f64 {
    if *phase {
        *phase = false;
        let u1 = base.rand() as f64 / RAND_MAX as f64;
        let u2 = base.rand() as f64 / RAND_MAX as f64;
        *v1 = 2.0 * u1 - 1.0;
        *v2 = 2.0 * u2 - 1.0;
        *v1 * (-2.0 * (*s).log10() / (*s)).sqrt()
    } else {
        *phase = true;
        *v2 * (-2.0 * (*s).log10() / (*s)).sqrt()
    }
}
