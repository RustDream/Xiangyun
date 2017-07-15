use std::f64::consts::PI;

/// RAND_MAX is a const
/// Please don't assume that it is any value
pub const RAND_MAX: u32 = 32767;

pub enum Style {
    Basic,
    PMrand,
    Lazy,
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
            seed: vec![time_get() as usize],
            style: Style::Basic,
            rand: ,
        }
    }

    /// set seed of solver
    pub fn srand(&mut self, seed: usize) {
        *self.seed = seed;
    }

    pub fn lazy_srand(&mut self) {
        self.srand(time_get() as usize);
    }

    /// get a random number
    pub fn rand(&mut self) -> usize {
        self.rand(&mut self.seed)
    }
}

fn gauss(seed: &mut usize, nsum: u32) -> f64 {
    let mut x = 0.0;
    for _ in 0..nsum {
        x += basic(seed) as f64 / RAND_MAX as f64;
    }
    x -= nsum as f64 / 2.0;
    x /= (nsum as f64 / 12.0).sqrt();
    x
}

fn bmgauss(seed: &mut usize, u: &mut f64, v: &mut f64, phase: &mut bool) -> f64 {
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

fn marsaglia(seed: &mut usize, v1: &mut f64, v2: &mut f64, s: &mut f64, phase: &mut bool) -> f64 {
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
