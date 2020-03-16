use linux_embedded_hal::{I2cdev};
use std::thread;
use std::time::Duration;
use apds9306_rs;


fn main() {

    let i2c = I2cdev::new("/dev/i2c-31").unwrap();
    let mut apds = lib::Apds9306::new(i2c, 0x52).unwrap();

    loop {
        let light = apds.read_light_bytes().unwrap();
        println!("{:?}", light);
        thread::sleep(Duration::from_millis(200));
    }
}
