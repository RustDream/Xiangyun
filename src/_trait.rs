pub trait RandBasic {
    type Style;
    type Seed;
    type Attachment;
    type Return;
    fn new(style: Self::Style) -> Self;
    fn srand(&mut self, seed: Self::Seed);
    fn lazy_srand(&mut self);
    fn get_rand(&mut self, attachment: Option<Self::Attachment>) -> Self::Return;
    fn lazy_rand(&mut self, min: i64, max: i64) -> i64;
    fn lazy_randf(&mut self, min: f64, max: f64) -> f64;
}
