#![no_std]
#![allow(non_camel_case_types)]

use core::fmt::Debug;
use embedded_hal::blocking::i2c::{WriteRead, Write};

use crate::register::Register;

#[derive(Debug)]
pub enum Error<E> {
    /// I²C bus error
    I2C(E),
    /// Invalid input data.
    WrongAddress,
    WriteToReadOnly,
    InvalidDataRate,
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

    pub fn read_light_bytes(&mut self) -> Result<[u8;6], Error<E>> {
        let mut data = [0u8;6];
        self.i2c
            .write_read(self.address, &[Register::LIGHT_L.addr() | 0x80], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data))
    }

    pub fn write_register(&mut self, register: Register, value: u8) -> Result<(), Error<E>> {
        if register.read_only() {
            return Err(Error::WriteToReadOnly);
        }
        self.i2c.write(self.address, &[register.addr(), value]).map_err(Error::I2C)
    }
}
