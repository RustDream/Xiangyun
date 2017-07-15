pub enum Flag {
    on(usize),
    off,
}

impl Flag {
    pub fn new() -> Self {
        Flag::off
    }

    pub fn on(&mut self) {
        *self = Flag::on(0);
    }

    pub fn off(&mut self) {
        *self = Flag::off;
    }

    pub fn up(&mut self) {
        match *self {
            Flag::on(e) => *self = Flag::on(e + 1),
            _ => *self = Flag::on(0),
        }
    }

    pub fn down(&mut self) {
        match *self {
            Flag::on(e) => {
                if e == 0 {
                    *self = Flag::off
                } else {
                    *self = Flag::on(e - 1)
                }
            }
            _ => {}
        }
    }

    pub fn is_on(&self) -> bool {
        match *self {
            Flag::on(_) => true,
            _ => false,
        }
    }
}
