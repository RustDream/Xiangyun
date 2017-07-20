use super::rand::base::{get_sys_seed, refresh_sys_seed, time_get};
use super::rand::Rand;

pub fn randomize() {
    refresh_sys_seed(time_get() as usize);
}

pub trait RandTrait {
    fn rand() -> Self;
}

macro_rules! impl_rand_trait {
    ($($t: ty)*) => (
        $(
            impl RandTrait for $t {
                fn rand() -> Self {
                    let mut _seed = get_sys_seed();
                    let mut solver = Rand::new();
                    solver.srand(0, _seed);
                    let _return = solver.rand();
                    _seed = solver.get_base_seed(0);
                    refresh_sys_seed(_seed);
                    _return as $t
                }
            }
        ) *
    )
}

impl_rand_trait!{u8 u16 u32 u64 usize}
impl_rand_trait!{i8 i16 i32 i64 isize}
impl_rand_trait!{f32 f64}