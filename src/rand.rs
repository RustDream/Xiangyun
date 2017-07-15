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

    pub fn srand(&mut self, seed: usize) {
        self.base.srand(seed);
    }

    pub fn lazy_srand(&mut self) {
        self.base.lazy_srand();
    }

    pub fn set_function(&mut self, style: &str) {
        self.base.set_function(style);
    }

    /// get a random number
    pub fn rand(&mut self) -> f64 {
        match self.style {
            Style::Normal => self.base.rand() as f64,
            Style::Gauss(nsum) => gauss(&mut self.base, nsum),
            Style::BMgauss(u, v, phase) => {
                let mut _u = u;
                let mut _v = v;
                let mut _phase = phase;
                let _return = bmgauss(&mut self.base, &mut _u, &mut _v, &mut _phase);
                self.style = Style::BMgauss(_u, _v, _phase);
                _return
            }
            Style::Marsaglia(v1, v2, s, phase) => {
                let mut _v1 = v1;
                let mut _v2 = v2;
                let mut _s = s;
                let mut _phase = phase;
                let _return = marsaglia(&mut self.base, &mut _v1, &mut _v2, &mut _s, &mut _phase);
                self.style = Style::Marsaglia(_v1, _v2, _s, _phase);
                _return
            }
        }
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

fn marsaglia(base: &mut BaseRand,
             v1: &mut f64,
             v2: &mut f64,
             s: &mut f64,
             phase: &mut bool)
             -> f64 {
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
