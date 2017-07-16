use super::base::{BaseRand, RAND_MAX};
use std::f64::consts::PI;

pub enum Style {
    Normal,
    Pythagoras,
    Gauss(u32),
    BMgauss(f64, f64, bool),
    Marsaglia(f64, f64, f64, bool),
}

pub enum JumpStyle {
    Static,
    Next,
    Jump,
    DoubleJump,
}

pub struct Rand {
    base: Vec<BaseRand>,
    handle: Option<usize>,
    style: Style,
    jump: JumpStyle,
}

impl Rand {
    /// a lazy way to get a random solver
    pub fn new() -> Self {
        Rand {
            base: vec![BaseRand::new()],
            handle: Some(0),
            style: Style::Normal,
            jump: JumpStyle::Static,
        }
    }

    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    pub fn srand(&mut self, seed: Vec<usize>) {
        for i in 0..self.base.len() {
            self.base[i].srand(seed[i]);
        }
    }

    pub fn lazy_srand(&mut self) {
        for i in 0..self.base.len() {
            self.base[i].lazy_srand();
        }
    }

    pub fn set_function(&mut self, handle: usize, style: &str) {
        self.base[handle].set_function(style);
    }

    /// get a random number
    pub fn rand(&mut self) -> f64 {
        match self.style {
            Style::Normal => self.base() as f64,
            Style::Pythagoras => pythagoras(self),
            Style::Gauss(nsum) => gauss(self, nsum),
            Style::BMgauss(u, v, phase) => {
                let mut _u = u;
                let mut _v = v;
                let mut _phase = phase;
                let _return = bmgauss(self, &mut _u, &mut _v, &mut _phase);
                self.style = Style::BMgauss(_u, _v, _phase);
                _return
            }
            Style::Marsaglia(v1, v2, s, phase) => {
                let mut _v1 = v1;
                let mut _v2 = v2;
                let mut _s = s;
                let mut _phase = phase;
                let _return = marsaglia(self, &mut _v1, &mut _v2, &mut _s, &mut _phase);
                self.style = Style::Marsaglia(_v1, _v2, _s, _phase);
                _return
            }
        }
    }

    pub fn base(&mut self) -> usize {
        match self.handle {
            Some(e) => self.base[e].rand(),
            _ => {
                let mut _base = 0;
                for i in 0..self.base.len() {
                    _base += self.base[i].rand() / self.base.len();
                }
                _base
            }
        }
    }

    pub fn new_base(&mut self) {
        self.base.push(BaseRand::new());
    }

    pub fn del_base(&mut self) -> Option<BaseRand> {
        self.base.pop()
    }

    pub fn jump_style(&mut self, style: JumpStyle) {
        self.jump = style;
    }

    pub fn jump(&mut self) {
        match self.handle {
            Some(e) => {
                match self.jump {
                    JumpStyle::Next => self.handle = Some((e + 1) % self.base.len()),
                    JumpStyle::Jump => {
                        let _jump = self.base();
                        self.handle = Some(_jump % self.base.len());
                    }
                    JumpStyle::DoubleJump => {
                        let _gap = self.base();
                        for _ in 0.._gap {
                            let _jump = self.base();
                            self.handle = Some(_jump % self.base.len());
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn gauss(base: &mut Rand, nsum: u32) -> f64 {
    let mut x = 0.0;
    for _ in 0..nsum {
        x += base.base() as f64 / RAND_MAX as f64;
    }
    x -= nsum as f64 / 2.0;
    x /= (nsum as f64 / 12.0).sqrt();
    x
}

fn bmgauss(base: &mut Rand, u: &mut f64, v: &mut f64, phase: &mut bool) -> f64 {
    if *phase {
        *phase = false;
        *u = (base.base() + 1) as f64 / (RAND_MAX + 2) as f64;
        *v = base.base() as f64 / (RAND_MAX + 1) as f64;
        (-2.0 * (*u).log10()).sqrt() * (2.0 * PI * (*v)).sin()
    } else {
        *phase = true;
        (-2.0 * (*u).log10()).sqrt() * (2.0 * PI * (*v)).cos()
    }
}

fn marsaglia(base: &mut Rand, v1: &mut f64, v2: &mut f64, s: &mut f64, phase: &mut bool) -> f64 {
    if *phase {
        *phase = false;
        let u1 = base.base() as f64 / RAND_MAX as f64;
        let u2 = base.base() as f64 / RAND_MAX as f64;
        *v1 = 2.0 * u1 - 1.0;
        *v2 = 2.0 * u2 - 1.0;
        *v1 * (-2.0 * (*s).log10() / (*s)).sqrt()
    } else {
        *phase = true;
        *v2 * (-2.0 * (*s).log10() / (*s)).sqrt()
    }
}

fn pythagoras(base: &mut Rand) -> f64 {
    let a = base.base() as f64 / (RAND_MAX + 1) as f64;
    let b = base.base() as f64 / (RAND_MAX + 1) as f64;
    let c_sqr = a * a + b * b;
    c_sqr.sqrt()
}
