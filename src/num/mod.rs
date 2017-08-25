pub mod xfloat;

use super::rand::base::system::{get_seed, refresh_seed};
use super::rand::Solver;
use super::rand::base::RAND_MAX;

pub trait Rand<T=Self> {
    fn rand() -> Self;
    fn range(min: T, max: T) -> Self;
}

macro_rules! impl_rand_trait {
    ($($t: ty)*) => (
        $(
            impl Rand for $t {
                fn rand() -> Self {
                    let mut solver = Solver::new();
                    let mut _base = solver.get_base(0).unwrap();
                    _base.srand(get_seed());
                    solver.set_base(0, _base);
                    let _return = solver.rand();
                    refresh_seed(solver.get_base(0).unwrap().get_seed());                   
                    _return as $t
                }

                fn range(min: Self, max: Self) -> Self {
                    let result = f64::rand() * ((max as f64 - min as f64 + 1.0) / RAND_MAX as f64);
                    result as $t + min
                }
            }
        ) *
    )
}

impl_rand_trait!{u8 u16 u32 u64 usize}
impl_rand_trait!{i8 i16 i32 i64 isize}
impl_rand_trait!{f32 f64}

#[cfg(feature = "i128_support")]
impl_rand_trait!{u128 i128}

impl Rand for char {
    fn rand() -> Self {
        (u8::rand() % 128) as char
    }

    fn range(min: Self, max: Self) -> Self {
        let result = f64::rand() * (((max as u8 - min as u8) as f64 + 1.0) / RAND_MAX as f64);
        (result as u8 + min as u8) as char
    }
}

impl Rand<usize> for String {
    fn rand() -> Self {
        let mut result = "".to_string();
        loop {
            let _char = char::rand();
            if _char == '\0' {
                break result;
            }
            result += &_char.to_string();
        }
    }
    
    fn range(min: usize, max: usize) -> Self {
        let long = usize::range(min, max);
        let mut result = "".to_string();
        for _ in 0..long {
            let _char = char::range(1 as char, 127 as char);
            result += &_char.to_string();
        }
        result
    }
}