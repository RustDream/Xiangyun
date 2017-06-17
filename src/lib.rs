pub mod rand {
    const PI: f64 = 3.141592654;
    const RAND_MAX: f64  = 2147483647.0;
    
    pub enum Style {
        PMrand,
        Gauss,
        BMrand,
        Marsaglia,
        Lazy,
    }

    pub struct Rand {
        style: Style,
        seed: i64,
        attachment: Vec<f64>,
    }

    impl Rand {
        pub fn new(style: Style) -> Self {
            Rand {
                style: style,
                seed: 1,
                attachment: Vec::new(),
            }
        }
        
        pub fn srand(&mut self, seed: i64) {
            self.seed = seed;
        }

        pub fn get_rand(&mut self) -> f64 {
            match self.style {
                Style::PMrand => {
                    self.seed = Rand::pmrand(self.seed, 48271);
                    self.seed as f64 / RAND_MAX
                }
                Style::Gauss => {
                    Rand::gauss(&mut self.seed, 25) / RAND_MAX
                }
                Style::BMrand => {
                    Rand::bmrand(&mut self.seed, &mut self.attachment) / RAND_MAX
                }
                _ => {
                    self.seed = Rand::lazy(self.seed);
                    self.seed as f64 / RAND_MAX
                }
            }
        }
        

        fn pmrand(seed: i64, a: i64) -> i64 {
            let m = RAND_MAX as i64;
            let q = m / a;
            let r = m % a;
            let hi = seed / q;
            let lo = seed % q;
            let test = a * lo - r * hi;
            if test > 0 { test } else { test + m }
        }
        
        fn gauss(seed: &mut i64, nsum: i64) -> f64 {
            let mut x = 0.0;
            for _ in 0..nsum {
                *seed = Rand::pmrand(*seed, 48271);
                x += *seed as f64/ RAND_MAX;
            }
            x -= nsum as f64 / 2.0;
            x /= (nsum as f64 / 12.0).sqrt();
            x
        }
        
        fn bmrand(seed: &mut i64, attachment: &mut Vec<f64>) -> f64 {
            // FIXME: value assigned to `z` is never read
            let mut z = 0.0;
            if attachment.len() == 0 {
                attachment.push(0.0);
                attachment.push(0.0);
                attachment.push(0.0);
            }
            if attachment[2] == 0.0 {
                *seed = Rand::pmrand(*seed, 48271);
                attachment[0] = (*seed as f64 + 1.0) / (RAND_MAX + 2.0);
                *seed = Rand::pmrand(*seed, 48271);
                attachment[1] = *seed as f64 / (RAND_MAX + 1.0);
                z = (2.0 * PI * attachment[1]).sin();
            } else {
                z = (2.0 * PI * attachment[1]).cos();
            }
            z *= (-2.0 * attachment[0].log(10.0)).sqrt();
            attachment[2] = 1.0 - attachment[2];
            z
        }
        
        fn marsaglia() -> f64 {
            let mut x: f64 = 0.0;
            
            x
        }

        fn lazy(seed: i64) -> i64 {
            Rand::pmrand(seed, 16807)
        }
    }
}
