#![no_std]
#![allow(non_camel_case_types)]

use core::fmt::Debug;
use embedded_hal::blocking::i2c::{WriteRead, Write};


#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data.
    WrongAddress,
    WriteToReadOnly,
}


pub struct Apds9306<I2C> {
    i2c: I2C,
    address: u8,
}

impl<I2C, E> Apds9306<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E>
{
    pub fn new(i2c: I2C, address: u8) -> Result<Self, Error<E>> {
        let mut apds9306 = Apds9306 { i2c, address };

        let buf = apds9306.read_register(Register::WHOAMI)?;

        if buf != 0xB3 {
            return Err(Error::WrongAddress)
        }
        // Enable sensor reading
        apds9306.write_register(Register::CTRL, 0x02)?;
        Ok(apds9306)
    }

    pub fn read_register(&mut self, register: Register) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[register.addr()], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }

    pub fn read_light_bytes(&mut self) -> Result<u32, Error<E>> {
        let mut data = [0u8;3];
        self.i2c
            .write_read(self.address, &[Register::LIGHT_L.addr()], &mut data)
            .map_err(Error::I2C)?;
        Ok(data[0] as u32 | (data[1] as u32) << 8 | (data[2] as u32) << 16)
    }

    pub fn write_register(&mut self, register: Register, value: u8) -> Result<(), Error<E>> {
        if register.read_only() {
            return Err(Error::WriteToReadOnly);
        }
        self.i2c.write(self.address, &[register.addr(), value]).map_err(Error::I2C)
    }
}

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
