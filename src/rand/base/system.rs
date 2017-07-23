use super::time_get;

static mut SYS_SEED: usize = 1;

pub fn get_seed() -> usize {
    unsafe {
        SYS_SEED
    }
}

pub fn refresh_seed(_seed: usize) {
    unsafe {
        SYS_SEED = _seed;
    }
}

pub fn randomize() {
    refresh_seed(time_get());
}
