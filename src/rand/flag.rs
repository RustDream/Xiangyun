#[derive(Copy, Clone, Debug)]
pub enum Flag {
    On(usize),
    Off,
}

impl Flag {
    pub fn new() -> Self {
        Flag::Off
    }

    pub fn on(&mut self, num: Option<usize>) {
        match num {
            Some(e) => *self = Flag::On(e),
            _ => *self = Flag::On(0),
        }
    }

    // #[allow(dead_code)]
    // pub fn off(&mut self) {
    //     *self = Flag::Off;
    // }

    // #[allow(dead_code)]
    // pub fn up(&mut self) {
    //     match *self {
    //         Flag::On(e) => *self = Flag::On(e + 1),
    //         _ => *self = Flag::On(0),
    //     }
    // }

    pub fn down(&mut self) {
        if let Flag::On(e) = *self {
            if e == 0 {
                *self = Flag::Off
            } else {
                *self = Flag::On(e - 1)
            }
        }
    }

    pub fn is_on(&self) -> bool {
        if let Flag::On(_) = *self {
            true
        } else {
            false
        }
    }
}
