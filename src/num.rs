#[macro_export]
macro_rules! randomize {
    ($rand: ident) => {
        let mut sys_random_solver = Rand::new();
        let mut $rand = | | sys_random_solver.rand();
    };
}

