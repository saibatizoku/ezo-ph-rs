//! An example that takes readings from the PH EZO chip in a loop.
//!

#![recursion_limit = "1024"]

extern crate chrono;
extern crate ezo_ph;
extern crate i2cdev;

use std::thread;
use std::time::Duration;

use chrono::{DateTime, Utc};

use ezo_ph::errors::*;
use ezo_ph::command::{Command, Reading, Sleep};
use ezo_ph::response::{SensorReading};

use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 99; // could be specified as 0x63

fn run() -> Result<()> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);

    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .chain_err(|| "Could not open I2C device")?;

    loop {

        let ph_value = Reading.run(&mut dev)?;

        let _out = _print_response(ph_value)?;

        let _sleep = Sleep.run(&mut dev)?;

        // Ideally, every 10 seconds, fine-tune this to your hardware.
        thread::sleep(Duration::from_millis(9_100));
    }
}

fn _print_response(reading: SensorReading) -> Result<()> {
    let dt: DateTime<Utc> = Utc::now();
    println!("{:?},{:?}",
             dt,
             reading,
             );
    Ok(())
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
