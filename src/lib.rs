//! I2C Commands for pH EZO Chip, taken from their Datasheet.
//! This chip is used for pH measurement. It features calibration, sleep mode,
//! scale, etc.
#![recursion_limit = "1024"]
#[macro_use]
extern crate error_chain;
extern crate i2cdev;

/// Use error-chain.
pub mod errors { error_chain! {} }

use errors::*;
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

/// Allowable baudrates used when changing the chip to UART mode.
#[derive(Debug)]
pub enum BpsRate {
    Bps300 = 300,
    Bps1200 = 1200,
    Bps2400 = 2400,
    Bps9600 = 9600,
    Bps19200 = 19200,
    Bps38400 = 38400,
    Bps57600 = 57600,
    Bps115200 = 115200,
}

pub enum PhCommand {
    // 'Baud' command
    Baud(BpsRate),
    // 'Cal' command
    CalibrationClear,
    CalibrationSingleMidPoint(f64),
    CalibrationThreePointAtHigh(f64),
    CalibrationTwoPointAtLow(f64),
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
    ProtocolLockStatus,
    // 'R' command
    Reading,
    // 'Sleep' command
    Sleep,
    // 'Slope' command
    Slope,
    // 'Status' command
    Status,
    // 'T' command
    TemperatureCompensation,
    TemperatureCompensatedValue,
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
    OutputState,
    ProtocolLockState,
    ProbeTypeState,
    Reading,
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
        unimplemented!();
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
        let cmd = CalibrationHigh(10.).build();
        assert_eq!(cmd.command, "Cal,high,10.0\0");
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
}
