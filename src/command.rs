//! I2C Commands for pH EZO Chip.
//!
use std::thread;
use std::time::Duration;

use errors::*;
use response::{
    CalibrationStatus,
    CompensationValue,
    DeviceInfo,
    DeviceStatus,
    Exported,
    ExportedInfo,
    LedStatus,
    ProbeSlope,
    ProtocolLockStatus,
    SensorReading,
};

use ezo_common::{
    BpsRate,
    ResponseCode,
    response_code,
    string_from_response_data,
    write_to_ezo,
};

use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;

/// Maximum ascii-character response size + 2
pub const MAX_DATA: usize = 401;

/// I2C command for the EZO chip.
pub trait Command {
    type Response;

    fn get_command_string (&self) -> String;
    fn get_delay (&self) -> u64;
    fn run(&self, dev: &mut LinuxI2CDevice) -> Result<Self::Response>;
}

define_command! {
    doc: "`Baud,n` command, where `n` is a variant belonging to `BpsRate`.",
    cmd: Baud(BpsRate), { format!("Baud,{}\0", cmd.parse() ) }, 0
}

define_command! {
    doc: "`Cal,mid,t` command, where `t` is of type `f64`.",
    cmd: CalibrationMid(f64), { format!("Cal,mid,{:.*}\0", 2, cmd) }, 900, Ack
}

define_command! {
    doc: "`Cal,low,t` command, where `t` is of type `f64`.",
    cmd: CalibrationLow(f64), { format!("Cal,low,{:.*}\0", 2, cmd) }, 900, Ack
}

define_command! {
    doc: "`Cal,high,t` command, where `t` is of type `f64`.",
    cmd: CalibrationHigh(f64), { format!("Cal,high,{:.*}\0", 2, cmd) }, 900, Ack
}

define_command! {
    doc: "`Cal,clear` command.",
    CalibrationClear, { "Cal,clear\0".to_string() }, 300, Ack
}

define_command! {
    doc: "`Cal,?` command. Returns a `CalibrationStatus` response.",
    CalibrationState, { "Cal,?\0".to_string() }, 300,
    resp: CalibrationStatus, { CalibrationStatus::parse(&resp) }
}

define_command! {
    doc: "`Export` command.",
    Export, { "Export\0".to_string() }, 300,
    resp: Exported, { Exported::parse(&resp) }
}

define_command! {
    doc: "`ExportInfo` command.",
    ExportInfo, { "Export,?\0".to_string() }, 300,
    resp: ExportedInfo, { ExportedInfo::parse(&resp) }
}

define_command! {
    doc: "`Import,n` command, where `n` is of type `String`.",
    cmd: Import(String), { format!("Import,{}\0", cmd) }, 300, Ack
}

define_command! {
    doc: "`Factory` command.",
    Factory, { "Factory\0".to_string() }, 0
}

define_command! {
    doc: "`Find` command.",
    Find, { "F\0".to_string() }, 300
}

define_command! {
    doc: "`I2C,n` command, where `n` is of type `u64`.",
    cmd: DeviceAddress(u16), { format!("I2C,{}\0", cmd) }, 300
}

define_command! {
    doc: "`I` command.",
    DeviceInformation, { "I\0".to_string() }, 300,
    resp: DeviceInfo, { DeviceInfo::parse(&resp) }
}

define_command! {
    doc: "`L,1` command.",
    LedOn, { "L,1\0".to_string() }, 300, Ack
}

define_command! {
    doc: "`L,0` command.",
    LedOff, { "L,0\0".to_string() }, 300, Ack
}

define_command! {
    doc: "`L,?` command.",
    LedState, { "L,?\0".to_string() }, 300,
    resp: LedStatus, { LedStatus::parse(&resp) }
}

define_command! {
    doc: "`Plock,1` command.",
    ProtocolLockEnable, { "Plock,1\0".to_string() }, 300, Ack
}

define_command! {
    doc: "`Plock,0` command.",
    ProtocolLockDisable, { "Plock,0\0".to_string() }, 300, Ack
}

define_command! {
    doc: "`Plock,?` command. Returns a `ProtocolLockStatus` response.",
    ProtocolLockState, { "Plock,?\0".to_string() }, 300,
    resp: ProtocolLockStatus, { ProtocolLockStatus::parse(&resp) }
}

define_command! {
    doc: "`R` command. Returns a `SensorReading` response.",
    Reading, { "R\0".to_string() }, 900,
    resp: SensorReading, { SensorReading::parse(&resp) }
}

define_command! {
    doc: "`Slope,?` command. Returns a `ProbeSlope` response.",
    Slope, { "Slope,?\0".to_string() }, 300,
    resp: ProbeSlope, { ProbeSlope::parse(&resp) }
}

define_command! {
    doc: "`Status` command. Returns a `DeviceStatus` response.",
    Status, { "Status\0".to_string() }, 300,
    resp: DeviceStatus, { DeviceStatus::parse(&resp) }
}

define_command! {
    doc: "`Sleep` command.",
    Sleep, { "Sleep\0".to_string() }, 0
}

define_command! {
    doc: "`T,t` command, where `t` is of type `f64`.",
    cmd: TemperatureCompensation(f64), { format!("T,{:.*}\0", 3, cmd) }, 300, Ack
}

define_command! {
    doc: "`T,?` command. Returns a `CompensationValue` response.",
    CompensatedTemperatureValue, { "T,?\0".to_string() }, 300,
    resp: CompensationValue, { CompensationValue::parse(&resp) }
}

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

    #[test]
    fn build_command_export() {
        let cmd = Export;
        assert_eq!(cmd.get_command_string(), "Export\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_export_info() {
        let cmd = ExportInfo;
        assert_eq!(cmd.get_command_string(), "Export,?\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = Import(calibration_string);
        assert_eq!(cmd.get_command_string(), "Import,ABCDEFGHIJKLMNO\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_factory() {
        let cmd = Factory;
        assert_eq!(cmd.get_command_string(), "Factory\0");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_find() {
        let cmd = Find;
        assert_eq!(cmd.get_command_string(), "F\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_device_information() {
        let cmd = DeviceInformation;
        assert_eq!(cmd.get_command_string(), "I\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_change_device_address() {
        let cmd = DeviceAddress(88);
        assert_eq!(cmd.get_command_string(), "I2C,88\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_led_on() {
        let cmd = LedOn;
        assert_eq!(cmd.get_command_string(), "L,1\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_led_off() {
        let cmd = LedOff;
        assert_eq!(cmd.get_command_string(), "L,0\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_led_state() {
        let cmd = LedState;
        assert_eq!(cmd.get_command_string(), "L,?\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_plock_enable() {
        let cmd = ProtocolLockEnable;
        assert_eq!(cmd.get_command_string(), "Plock,1\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_plock_disable() {
        let cmd = ProtocolLockDisable;
        assert_eq!(cmd.get_command_string(), "Plock,0\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_plock_status() {
        let cmd = ProtocolLockState;
        assert_eq!(cmd.get_command_string(), "Plock,?\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_reading() {
        let cmd = Reading;
        assert_eq!(cmd.get_command_string(), "R\0");
        assert_eq!(cmd.get_delay(), 900);
    }

    #[test]
    fn build_command_sleep_mode() {
        let cmd = Sleep;
        assert_eq!(cmd.get_command_string(), "Sleep\0");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn build_command_slope() {
        let cmd = Slope;
        assert_eq!(cmd.get_command_string(), "Slope,?\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_device_status() {
        let cmd = Status;
        assert_eq!(cmd.get_command_string(), "Status\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_temperature_compensation() {
        let cmd = TemperatureCompensation(19.5);
        assert_eq!(cmd.get_command_string(), "T,19.500\0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_temperature_compensation_value() {
        let cmd = CompensatedTemperatureValue;
        assert_eq!(cmd.get_command_string(), "T,?\0");
        assert_eq!(cmd.get_delay(), 300);
    }
}
