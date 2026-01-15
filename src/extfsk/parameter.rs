#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtfskParameter {
    pub baud: u16,
    pub stop_bit: ExtfskStopbit,
    pub length: u8,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExtfskStopbit {
    One = 0,
    OneHalf = 1,
    Two = 2,
}

impl ExtfskParameter {
    pub fn parse(parameter: u32) -> ExtfskParameter {
        let baud = (parameter >> 16) as u16;
        let stop_bit = match parameter & 0b11 {
            0 => ExtfskStopbit::One,
            1 => ExtfskStopbit::OneHalf,
            2 => ExtfskStopbit::Two,
            _ => ExtfskStopbit::One,
        };
        let length = (parameter >> 2 & 0b1111) as u8;

        ExtfskParameter {
            baud,
            stop_bit,
            length,
        }
    }
}
