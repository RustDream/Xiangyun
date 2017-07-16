pub enum Flag {
    On(usize),
    Off,
}

impl Flag {
    pub fn new() -> Self {
        Flag::Off
    }

    pub fn on(&mut self) {
        self._on(0);
    }

    pub fn _on(&mut self, num: usize) {
        *self = Flag::On(num);
    }

    pub fn off(&mut self) {
        *self = Flag::Off;
    }

    pub fn up(&mut self) {
        match *self {
            Flag::On(e) => *self = Flag::On(e + 1),
            _ => *self = Flag::On(0),
        }
    }

    pub fn down(&mut self) {
        match *self {
            Flag::On(e) => {
                if e == 0 {
                    *self = Flag::Off
                } else {
                    *self = Flag::On(e - 1)
                }
            }
            _ => {}
        }
    }

    pub fn is_on(&self) -> bool {
        match *self {
            Flag::On(_) => true,
            _ => false,
        }
    }
}
