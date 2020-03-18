use apds9306_rs::Apds9306;
use linux_embedded_hal::I2cdev;
use std::thread;
use std::time::Duration;

fn main() {
    let buses = vec!["/dev/i2c-32", "/dev/i2c-31", "/dev/i2c-30"];
    let mut apdses = vec![];
    for bus in buses {
        let i2c = I2cdev::new(bus).unwrap();
        let mut apds = Apds9306::new(i2c, 0x52).unwrap();
        apdses.push(apds);
    }

    loop {
        for apds in &mut apdses {
            let light = apds.read_light_bytes().unwrap();
            print!("{:6}  ", light);
        }
        println!();
        thread::sleep(Duration::from_millis(200));
    }
}
