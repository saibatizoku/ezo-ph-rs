//! An example that retrieves the current settings of the PH EZO chip.
//!
#![recursion_limit = "1024"]
#![feature(inclusive_range_syntax)]
extern crate ezo_ph;
extern crate failure;
extern crate i2cdev;

use ezo_ph::{
    command::{Command, DeviceInformation, CalibrationState, LedState, Reading, Slope, Sleep, Status},
    response::{CalibrationStatus, DeviceInfo, DeviceStatus, LedStatus, SensorReading, ProbeSlope},
};
use failure::{Error, ResultExt};
use i2cdev::linux::LinuxI2CDevice;

const I2C_BUS_ID: u8 = 1;
const EZO_SENSOR_ADDR: u16 = 99; // could be specified as 0x63

fn run() -> Result<(), Error> {
    let device_path = format!("/dev/i2c-{}", I2C_BUS_ID);
    let mut dev = LinuxI2CDevice::new(&device_path, EZO_SENSOR_ADDR)
        .context("Could not open I2C device")?;

    let info: DeviceInfo = DeviceInformation.run(&mut dev)?;
    println!("{:?}", info);

    let status: DeviceStatus = Status.run(&mut dev)?;
    println!("DeviceStatus: {:?}", status);

    let calibration: CalibrationStatus = CalibrationState.run(&mut dev)?;
    println!("CalibrationState: {:?}", calibration);

    let led_status: LedStatus = LedState.run(&mut dev)?;
    println!("LedState: {:?}", led_status);

    let ph_value: SensorReading = Reading.run(&mut dev)?;
    println!("{:?}", ph_value);

    let probe_slope: ProbeSlope = Slope.run(&mut dev)?;
    println!("{:?}", probe_slope);

    let _sleep = Sleep.run(&mut dev)?;
    println!("Sleeping....");

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);
        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        let backtrace = e.backtrace();
        println!("backtrace: {:?}", backtrace);
        ::std::process::exit(1);
    }
}
