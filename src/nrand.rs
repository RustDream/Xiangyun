pub const NRAND_MAX: u32 = 2147483647;

pub enum NStyle {
    PMrand,
    Gauss,
    BMrand,
    Marsaglia,
    Lazy,
    Crand,
    Ryus,
    Dalton,
}

pub struct NRand {
    style: NStyle,
    seed: i64,
    attachment: Vec<f64>,
}
