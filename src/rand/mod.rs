mod flag;
pub mod base;

use self::base::{BaseRand, RAND_MAX};
use self::flag::Flag;
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

/// Rand is random number solver  
/// Please don't assume that the fields are any type  
pub struct Rand {
    base: Vec<BaseRand>,
    // TODO: Use Flag instead Option<usize>
    handle: Option<usize>,
    style: Style,
    jump: JumpStyle,
}

impl Rand {
    // TODO: Need a private function to get the handle base

    /// A lazy way to get a random solver
    pub fn new() -> Self {
        Rand {
            base: vec![BaseRand::new()],
            // TODO: Use Flag instead Option<usize>
            handle: Some(0),
            style: Style::Normal,
            jump: JumpStyle::Static,
        }
    }

    /// Insert a new base
    pub fn new_multibase(mul: usize) -> Self {
        let mut foo = Rand::new();
        let mut bar: Vec<usize> = Vec::new();
        for _ in 0..mul {
            bar.push(foo.base());
        }
        for _ in 1..mul {
            foo.new_base();
        }
        foo.multisrand(bar);
        foo.jump_style(JumpStyle::DoubleJump);
        foo
    }

    /// Set random solver style
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    /// Set the random seed for handle base
    pub fn srand(&mut self, handle: usize, seed: usize) {
        self.base[handle].srand(seed);
    }

    /// Get the random seed for handle base
    pub fn get_base_seed(&self, handle: usize) -> usize {
        self.base[handle].get_seed()
    }

    /// Set the random seed
    pub fn multisrand(&mut self, seed: Vec<usize>) {
        for i in 0..self.base.len() {
            self.srand(i, seed[i]);
        }
    }

    /// Lazy way to set the random seed
    pub fn lazy_srand(&mut self) {
        for i in 0..self.base.len() {
            self.base[i].lazy_srand();
        }
    }

    pub fn set_function(&mut self, handle: usize, style: &str) {
        self.base[handle].set_function(style);
    }

    /// Get a random number
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

    /// Get a random number for multibase
    pub fn multirand(&mut self) -> f64 {
        let _return = self.rand();
        self.jump();
        _return
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

    pub fn add_base(&mut self, base: BaseRand) {
        self.base.push(base);
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
                        let mut _gap = Flag::new();
                        _gap._on(self.base());
                        while _gap.is_on() {
                            let _jump = self.base();
                            self.handle = Some(_jump % self.base.len());
                            _gap.down();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

fn base64(base: &mut Rand, base_offset: usize, max_offset: usize) -> f64 {
    (base.base() + base_offset) as f64 / (RAND_MAX + base_offset + max_offset) as f64
}

fn gauss(base: &mut Rand, nsum: u32) -> f64 {
    let mut x = 0.0;
    for _ in 0..nsum {
        x += base64(base, 0, 0);
    }
    x -= nsum as f64 / 2.0;
    x /= (nsum as f64 / 12.0).sqrt();
    x
}

fn bmgauss(base: &mut Rand, u: &mut f64, v: &mut f64, phase: &mut bool) -> f64 {
    if *phase {
        *phase = false;
        *u = base64(base, 1, 1);
        *v = base64(base, 0, 1);
        (-2.0 * (*u).log10()).sqrt() * (2.0 * PI * (*v)).sin()
    } else {
        *phase = true;
        (-2.0 * (*u).log10()).sqrt() * (2.0 * PI * (*v)).cos()
    }
}

fn marsaglia(base: &mut Rand, v1: &mut f64, v2: &mut f64, s: &mut f64, phase: &mut bool) -> f64 {
    if *phase {
        *phase = false;
        let u1 = base64(base, 0, 0);
        let u2 = base64(base, 0, 0);
        *v1 = 2.0 * u1 - 1.0;
        *v2 = 2.0 * u2 - 1.0;
        *v1 * (-2.0 * (*s).log10() / (*s)).sqrt()
    } else {
        *phase = true;
        *v2 * (-2.0 * (*s).log10() / (*s)).sqrt()
    }
}

fn pythagoras(base: &mut Rand) -> f64 {
    let a = base64(base, 0, 1);
    let b = base64(base, 0, 1);
    let c_sqr = a * a + b * b;
    c_sqr.sqrt()
}
