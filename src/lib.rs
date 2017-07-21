//! I2C Commands for pH EZO Chip, taken from their Datasheet.
//! This chip is used for pH measurement. It features calibration, sleep mode,
//! scale, etc.

#![feature(str_checked_slicing)]

#![feature(exclusive_range_pattern)]

#![feature(inclusive_range_syntax)]

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate ezo_common;
extern crate i2cdev;

// Use error-chain.
pub mod errors;

/// Issuable commands for the EZO RTD Chip.
pub mod command;

/// Parseable responses from the EZO RTD Chip.
pub mod response;

use errors::*;
use ezo_common::BpsRate;
use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;
use std::thread;
use std::time::Duration;

/// Maximum ascii-character response size + 2
pub const MAX_RESPONSE_LENGTH: usize = 401;

/// Useful for properly building I2C parameters from a command.
pub trait I2cCommand {
    fn build(&self) -> CommandOptions;
}

/// Commands for interacting with the PH EZO chip.
pub enum PhCommand {
    // 'Baud' command
    Baud(BpsRate),
    // 'Cal' command
    CalibrationClear,
    CalibrationMid(f64),
    CalibrationLow(f64),
    CalibrationHigh(f64),
    CalibrationState,
    // 'Export/Import' command
    Export,
    ExportInfo,
    Import(String),
    // 'Factory' command
    Factory,
    // 'Find' command
    Find,
    // 'I' command
    DeviceInformation,
    // 'I2C' command
    DeviceAddress(u8),
    // 'L' command
    LedOn,
    LedOff,
    LedState,
    // 'Plock' command
    ProtocolLockDisable,
    ProtocolLockEnable,
    ProtocolLockState,
    // 'R' command
    Reading,
    // 'Sleep' command
    Sleep,
    // 'Slope' command
    Slope,
    // 'Status' command
    Status,
    // 'T' command
    TemperatureCompensation(f64),
    TemperatureCompensationValue,
}

/// Command-related parameters used to build I2C write/read interactions.
#[derive(Clone,Debug,Default,PartialEq,Eq)]
pub struct CommandOptions {
    pub command: String,
    pub delay: Option<u64>,
    pub response: Option<CommandResponse>,
}

/// Allowed responses from I2C read interactions.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum CommandResponse {
    Ack,
    CalibrationState,
    CompensationValue,
    DeviceInformation,
    Export,
    ExportInfo,
    LedState,
    ProtocolLockState,
    Reading,
    Slope,
    Status,
}

/// Builds commands.
pub trait CommandBuilder {
    fn finish(&self) -> CommandOptions;
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<()>;
    fn set_command(&mut self, command_str: String) -> &mut CommandOptions;
    fn set_delay(&mut self, delay: u64) -> &mut CommandOptions;
    fn set_response(&mut self, response: CommandResponse) -> &mut CommandOptions;
}

impl CommandBuilder for CommandOptions {
    fn finish(&self) -> CommandOptions {
        self.clone()
    }
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<()> {
        let mut data_buffer = [0u8; MAX_RESPONSE_LENGTH];
        println!("COMMAND: {}", self.command);
        if let Err(_) = dev.write(self.command.as_bytes()) {
            thread::sleep(Duration::from_millis(300));
            dev.write(self.command.as_bytes())
                .chain_err(|| "Command could not be sent")?;
        };
        if let Some(delay) = self.delay {
            thread::sleep(Duration::from_millis(delay));
        }
        if let Some(_) = self.response {
            if let Err(_) = dev.read(&mut data_buffer) {
                thread::sleep(Duration::from_millis(300));
                dev.read(&mut data_buffer)
                    .chain_err(|| "Error reading from device")?;
            };
            match data_buffer[0] {
                255 => println!("No data expected."),
                254 => println!("Pending"),
                2 => println!("Error"),
                1 => {
                    let data: String = match data_buffer.into_iter().position(|&x| x == 0) {
                        Some(eol) => {
                            data_buffer[1..eol]
                                .into_iter()
                                .map(|c| (*c & !0x80) as char)
                                .collect()
                        }
                        _ => {
                            String::from_utf8(Vec::from(&data_buffer[1..]))
                                .chain_err(|| "Data is not readable")?
                        }
                    };
                    println!("RESPONSE: {}", data);
                }
                _ => println!("NO RESPONSE"),
            };
        }
        println!();
        Ok(())
    }
    /// Sets the ASCII string for the command to be sent
    fn set_command(&mut self, command_str: String) -> &mut CommandOptions {
        self.command = command_str;
        self
    }
    fn set_delay(&mut self, delay: u64) -> &mut CommandOptions {
        self.delay = Some(delay);
        self
    }
    fn set_response(&mut self, response: CommandResponse) -> &mut CommandOptions {
        self.response = Some(response);
        self
    }
}

impl I2cCommand for PhCommand {
    fn build(&self) -> CommandOptions {
        use self::PhCommand::*;
        let mut opts = CommandOptions::default();
        match *self {
            Baud(ref baud) => {
                let rate = match *baud {
                    BpsRate::Bps300 => BpsRate::Bps300 as u32,
                    BpsRate::Bps1200 => BpsRate::Bps1200 as u32,
                    BpsRate::Bps2400 => BpsRate::Bps2400 as u32,
                    BpsRate::Bps9600 => BpsRate::Bps9600 as u32,
                    BpsRate::Bps19200 => BpsRate::Bps19200 as u32,
                    BpsRate::Bps38400 => BpsRate::Bps38400 as u32,
                    BpsRate::Bps57600 => BpsRate::Bps57600 as u32,
                    BpsRate::Bps115200 => BpsRate::Bps115200 as u32,
                };
                opts.set_command(format!("Baud,{}\0", rate)).finish()
            }
            CalibrationMid(cal) => {
                opts.set_command(format!("Cal,mid,{:.*}\0", 2, cal))
                    .set_delay(900)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            CalibrationLow(cal) => {
                opts.set_command(format!("Cal,low,{:.*}\0", 2, cal))
                    .set_delay(900)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            CalibrationHigh(cal) => {
                opts.set_command(format!("Cal,high,{:.*}\0", 2, cal))
                    .set_delay(900)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            CalibrationClear => {
                opts.set_command("Cal,clear\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            CalibrationState => {
                opts.set_command("Cal,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::CalibrationState)
                    .finish()
            }
            Export => {
                opts.set_command("Export\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Export)
                    .finish()
            }
            ExportInfo => {
                opts.set_command("Export,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::ExportInfo)
                    .finish()
            }
            Import(ref calib) => {
                opts.set_command(format!("Import,{}\0", calib))
                    .set_delay(300)
                    .finish()
            }
            Factory => opts.set_command("Factory\0".to_string()).finish(),
            Find => {
                opts.set_command("F\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            DeviceInformation => {
                opts.set_command("I\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::DeviceInformation)
                    .finish()
            }
            DeviceAddress(addr) => {
                opts.set_command(format!("I2C,{}\0", addr))
                    .set_delay(300)
                    .finish()
            }
            LedOn => {
                opts.set_command("L,1\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            LedOff => {
                opts.set_command("L,0\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            LedState => {
                opts.set_command("L,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::LedState)
                    .finish()
            }
            ProtocolLockEnable => {
                opts.set_command("Plock,1\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            ProtocolLockDisable => {
                opts.set_command("Plock,0\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            ProtocolLockState => {
                opts.set_command("Plock,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::ProtocolLockState)
                    .finish()
            }
            Reading => {
                opts.set_command("R\0".to_string())
                    .set_delay(900)
                    .set_response(CommandResponse::Reading)
                    .finish()
            }
            Sleep => opts.set_command("Sleep\0".to_string()).finish(),
            Slope => {
                opts.set_command("Slope,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Slope)
                    .finish()
            }
            Status => {
                opts.set_command("Status\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::Status)
                    .finish()
            }
            TemperatureCompensation(temp) => {
                opts.set_command(format!("T,{}\0", temp))
                    .set_delay(300)
                    .set_response(CommandResponse::Ack)
                    .finish()
            }
            TemperatureCompensationValue => {
                opts.set_command("T,?\0".to_string())
                    .set_delay(300)
                    .set_response(CommandResponse::CompensationValue)
                    .finish()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::PhCommand::*;

    #[test]
    fn build_command_baud_300() {
        let cmd = Baud(BpsRate::Bps300).build();
        assert_eq!(cmd.command, "Baud,300\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_1200() {
        let cmd = Baud(BpsRate::Bps1200).build();
        assert_eq!(cmd.command, "Baud,1200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_2400() {
        let cmd = Baud(BpsRate::Bps2400).build();
        assert_eq!(cmd.command, "Baud,2400\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_9600() {
        let cmd = Baud(BpsRate::Bps9600).build();
        assert_eq!(cmd.command, "Baud,9600\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_19200() {
        let cmd = Baud(BpsRate::Bps19200).build();
        assert_eq!(cmd.command, "Baud,19200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_38400() {
        let cmd = Baud(BpsRate::Bps38400).build();
        assert_eq!(cmd.command, "Baud,38400\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_57600() {
        let cmd = Baud(BpsRate::Bps57600).build();
        assert_eq!(cmd.command, "Baud,57600\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_baud_115200() {
        let cmd = Baud(BpsRate::Bps115200).build();
        assert_eq!(cmd.command, "Baud,115200\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_calibration_mid() {
        let cmd = CalibrationMid(7.00).build();
        assert_eq!(cmd.command, "Cal,mid,7.00\0");
        assert_eq!(cmd.delay, Some(900));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_calibration_low() {
        let cmd = CalibrationLow(4.).build();
        assert_eq!(cmd.command, "Cal,low,4.00\0");
        assert_eq!(cmd.delay, Some(900));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_calibration_high() {
        let cmd = CalibrationHigh(10.056).build();
        assert_eq!(cmd.command, "Cal,high,10.06\0");
        assert_eq!(cmd.delay, Some(900));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_calibration_clear() {
        let cmd = CalibrationClear.build();
        assert_eq!(cmd.command, "Cal,clear\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_calibration_state() {
        let cmd = CalibrationState.build();
        assert_eq!(cmd.command, "Cal,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::CalibrationState));
    }

    #[test]
    fn build_command_export() {
        let cmd = Export.build();
        assert_eq!(cmd.command, "Export\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Export));
    }

    #[test]
    fn build_command_export_info() {
        let cmd = ExportInfo.build();
        assert_eq!(cmd.command, "Export,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::ExportInfo));
    }

    #[test]
    fn build_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = Import(calibration_string).build();
        assert_eq!(cmd.command, "Import,ABCDEFGHIJKLMNO\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_factory() {
        let cmd = Factory.build();
        assert_eq!(cmd.command, "Factory\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_find() {
        let cmd = Find.build();
        assert_eq!(cmd.command, "F\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_device_information() {
        let cmd = DeviceInformation.build();
        assert_eq!(cmd.command, "I\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::DeviceInformation));
    }

    #[test]
    fn build_command_change_device_address() {
        let cmd = DeviceAddress(88).build();
        assert_eq!(cmd.command, "I2C,88\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_led_on() {
        let cmd = LedOn.build();
        assert_eq!(cmd.command, "L,1\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_led_off() {
        let cmd = LedOff.build();
        assert_eq!(cmd.command, "L,0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_led_state() {
        let cmd = LedState.build();
        assert_eq!(cmd.command, "L,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::LedState));
    }

    #[test]
    fn build_command_plock_enable() {
        let cmd = ProtocolLockEnable.build();
        assert_eq!(cmd.command, "Plock,1\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_plock_disable() {
        let cmd = ProtocolLockDisable.build();
        assert_eq!(cmd.command, "Plock,0\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_plock_status() {
        let cmd = ProtocolLockState.build();
        assert_eq!(cmd.command, "Plock,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::ProtocolLockState));
    }

    #[test]
    fn build_command_reading() {
        let cmd = Reading.build();
        assert_eq!(cmd.command, "R\0");
        assert_eq!(cmd.delay, Some(900));
        assert_eq!(cmd.response, Some(CommandResponse::Reading));
    }

    #[test]
    fn build_command_sleep_mode() {
        let cmd = Sleep.build();
        assert_eq!(cmd.command, "Sleep\0");
        assert_eq!(cmd.delay, None);
        assert_eq!(cmd.response, None);
    }

    #[test]
    fn build_command_slope() {
        let cmd = Slope.build();
        assert_eq!(cmd.command, "Slope,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Slope));
    }

    #[test]
    fn build_command_device_status() {
        let cmd = Status.build();
        assert_eq!(cmd.command, "Status\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Status));
    }

    #[test]
    fn build_command_temperature_compensation() {
        let cmd = TemperatureCompensation(19.5).build();
        assert_eq!(cmd.command, "T,19.5\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::Ack));
    }

    #[test]
    fn build_command_temperature_compensation_value() {
        let cmd = TemperatureCompensationValue.build();
        assert_eq!(cmd.command, "T,?\0");
        assert_eq!(cmd.delay, Some(300));
        assert_eq!(cmd.response, Some(CommandResponse::CompensationValue));
    }
}
