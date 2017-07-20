//! I2C Commands for pH EZO Chip.
//!
use std::thread;
use std::time::Duration;

use errors::*;

use ezo_common::BpsRate;

use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;

/// Maximum ascii-character response size + 2
pub const MAX_RESPONSE_LENGTH: usize = 401;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_command_uart_300() {
        let cmd = Baud(BpsRate::Bps300);
        assert_eq!(cmd.get_command_string(), "Baud,300\0");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_1200() {
        let cmd = Baud(BpsRate::Bps1200);
        assert_eq!(cmd.get_command_string(), "Baud,1200\0");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_2400() {
        let cmd = Baud(BpsRate::Bps2400);
        assert_eq!(cmd.get_command_string(), "Baud,2400\0");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_9600() {
        let cmd = Baud(BpsRate::Bps9600);
        assert_eq!(cmd.get_command_string(), "Baud,9600\0");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_19200() {
        let cmd = Baud(BpsRate::Bps19200);
        assert_eq!(cmd.get_command_string(), "Baud,19200\0");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_38400() {
        let cmd = Baud(BpsRate::Bps38400);
        assert_eq!(cmd.get_command_string(), "Baud,38400\0");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_57600() {
        let cmd = Baud(BpsRate::Bps57600);
        assert_eq!(cmd.get_command_string(), "Baud,57600\0");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_uart_115200() {
        let cmd = Baud(BpsRate::Bps115200);
        assert_eq!(cmd.get_command_string(), "Baud,115200\0");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_calibration_mid() {
        let cmd = CalibrationMid(7.00);
        assert_eq!(cmd.get_command_string(), "Cal,mid,7.00\0");
        assert_eq!(cmd.get_delay(), 900);
    }

    #[test]
    fn build_command_calibration_low() {
        let cmd = CalibrationLow(4.);
        assert_eq!(cmd.get_command_string(), "Cal,low,4.00\0");
        assert_eq!(cmd.get_delay(), 900);
    }

    #[test]
    fn build_command_calibration_high() {
        let cmd = CalibrationHigh(10.056);
        assert_eq!(cmd.get_command_string(), "Cal,high,10.06\0");
        assert_eq!(cmd.get_delay(), 900);
    }

    #[test]
    fn build_command_calibration_clear() {
        let cmd = CalibrationClear;
        assert_eq!(cmd.get_command_string(), "Cal,clear\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_calibration_state() {
        let cmd = CalibrationState;
        assert_eq!(cmd.get_command_string(), "Cal,?\0");
        assert_eq!(cmd.get_delay(), 300);
    }
}
