use super::rand::base::{get_sys_seed, refresh_sys_seed, sys_srand};
use super::rand::Rand;

pub fn randomize() {
    sys_srand();
}

pub trait RandTrait {
    fn rand() -> Self;
}

macro_rules! impl_rand_trait {
    ($($t: ty)*) => (
        $(
            impl RandTrait for $t {
                fn rand() -> Self {
                    let mut solver = Rand::new();
                    // FIXME
                    solver.get_base(0).unwrap().srand(get_sys_seed());
                    let _return = solver.rand();
                    refresh_sys_seed(solver.get_base(0).unwrap().get_seed());                   
                    _return as $t
                }
            }
        ) *
    )
}

impl_rand_trait!{u8 u16 u32 u64 usize}
impl_rand_trait!{i8 i16 i32 i64 isize}
impl_rand_trait!{f32 f64}
