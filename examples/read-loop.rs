#![recursion_limit = "1024"]
//! An example that takes readings from the PH EZO chip in a loop.
//!
extern crate ezo_ph;
extern crate i2cdev;

use ezo_ph::errors::*;
use ezo_ph::{CommandBuilder, I2cCommand, PhCommand};
use i2cdev::linux::LinuxI2CDevice;
use std::thread;
use std::time::Duration;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 99; // could be specified as 0x63

fn run() -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;
    loop {
        PhCommand::Reading.build().run(&mut dev)?;
        PhCommand::Sleep.build().run(&mut dev)?;
        thread::sleep(Duration::from_millis(9400));
    }
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }
        ::std::process::exit(1);
    }
}
