// This mod will move to Granny crate, sooner or later

pub trait XFloat {
     fn get_fractional_part(&self) -> Self;
     fn get_integer_part(&self) -> Self;
     fn get_fractional_digit(&self) -> u8;
}

impl XFloat for f32 {
    fn get_fractional_part(&self) -> Self {
        let _self_integer_part = self.get_integer_part();
        *self - _self_integer_part
    }

    fn get_integer_part(&self) -> Self {
        *self as i64 as Self
    }

    fn get_fractional_digit(&self) -> u8 {
        let _self_fractional_string = format!("{}", self.get_fractional_part());
        _self_fractional_string.len() - 2
    }
}
