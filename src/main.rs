use linux_embedded_hal::{I2cdev};
use std::thread;
use std::time::Duration;
mod lib;

fn main() {

    let i2c = I2cdev::new("/dev/i2c-31").unwrap();
    let mut apds = Apds9306::new(i2c, 0x52).unwrap();

    loop {
        let acceleration = apds.read_accel_bytes().unwrap();
        println!("{:?}", acceleration);
        thread::sleep(Duration::from_millis(200));
    }
}
