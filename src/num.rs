use super::rand::base::system::{get_seed, refresh_seed};
use super::rand::Solver;

pub trait Rand {
    fn rand() -> Self;
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
            }
        ) *
    )
}

impl_rand_trait!{u8 u16 u32 u64 usize}
impl_rand_trait!{i8 i16 i32 i64 isize}
impl_rand_trait!{f32 f64}
