use std::time::SystemTime;

const PI: f64 = 3.141592654;
const RAND_MAX: f64 = 2147483647.0;

pub enum Style {
    PMrand,
    Gauss,
    BMrand,
    Marsaglia,
    Lazy,
    Crand,
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

    pub fn lazy_srand(&mut self) {
        let sys_time = SystemTime::now();
        let foo_string = format!("{:?}", sys_time);
        let mut seed: i64 = 0;
        let mut flag = false;
        for num in foo_string.chars() {
            match num {
                e @ '0'...'9' => {
                    flag = true;
                    seed = seed * 10 + (e as u8 - 48) as i64;
                    if seed >= i32::max_value() as i64 {
                        break;
                    }
                }
                _ => {
                    if flag {
                        break;
                    }
                }
            }
        }
        self.seed = seed;
    }

    pub fn get_rand(&mut self) -> f64 {
        match self.style {
            Style::PMrand => {
                self.seed = Rand::pmrand(self.seed, 48271);
                self.seed as f64 / RAND_MAX
            }
            Style::Gauss => Rand::gauss(&mut self.seed, 25) / RAND_MAX,
            Style::BMrand => Rand::bmrand(&mut self.seed, &mut self.attachment) / RAND_MAX,
            Style::Marsaglia => Rand::marsaglia(&mut self.seed, &mut self.attachment) / RAND_MAX,
            Style::Crand => Rand::crand(&mut self.seed) as f64 / RAND_MAX,
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
            x += *seed as f64 / RAND_MAX;
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

    fn marsaglia(seed: &mut i64, attachment: &mut Vec<f64>) -> f64 {
        // FIXME: value assigned to `x` is never read
        let mut x = 0.0;
        if attachment.len() == 0 {
            attachment.push(0.0);
            attachment.push(0.0);
            attachment.push(0.0);
            attachment.push(0.0);
        }
        if attachment[3] == 0.0 {
            loop {
                *seed = Rand::pmrand(*seed, 48271);
                let u1 = *seed as f64 / RAND_MAX;
                *seed = Rand::pmrand(*seed, 48271);
                let u2 = *seed as f64 / RAND_MAX;
                attachment[0] = 2.0 * u1 - 1.0;
                attachment[1] = 2.0 * u2 - 1.0;
                attachment[2] = attachment[0] * attachment[0] + attachment[1] * attachment[1];
                if attachment[2] < 1.0 && attachment[2] != 0.0 {
                    break;
                }
            }
            x = attachment[0] * (-2.0 * attachment[2].log(10.0) / attachment[2]);
        } else {
            x = attachment[1] * (-2.0 * attachment[2].log(10.0) / attachment[2]);
        }
        attachment[3] = 1.0 - attachment[3];
        x
    }

    fn lazy(seed: i64) -> i64 {
        Rand::pmrand(seed, 16807)
    }

    fn crand(seed: &mut i64) -> i64 {
        *seed = *seed * 1103515245 + 12345;
        (*seed >> 16) & (RAND_MAX as i64)
    }
}
