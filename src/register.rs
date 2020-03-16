#![allow(non_camel_case_types)]

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    LIGHT_L          = 0x0D,
    LIGHT_M          = 0x0E,
    LIGHT_H          = 0x0F,
    CTRL             = 0x00,
    WHOAMI           = 0x06,
}

impl Register {
    pub fn addr(self) -> u8 {
        self as u8
    }

    pub fn read_only(self) -> bool {
        match self {
            Register::WHOAMI |
            Register::LIGHT_L |
            Register::LIGHT_M |
            Register::LIGHT_H => true,
            _ => false,
        }
    }
}
