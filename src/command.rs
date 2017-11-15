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
    cmd: Baud(BpsRate), { format!("Baud,{}", cmd.parse() ) }, 0
}

define_command! {
    doc: "`Cal,mid,t` command, where `t` is of type `f64`.",
    cmd: CalibrationMid(f64), { format!("Cal,mid,{:.*}", 2, cmd) }, 900, Ack
}

define_command! {
    doc: "`Cal,low,t` command, where `t` is of type `f64`.",
    cmd: CalibrationLow(f64), { format!("Cal,low,{:.*}", 2, cmd) }, 900, Ack
}

define_command! {
    doc: "`Cal,high,t` command, where `t` is of type `f64`.",
    cmd: CalibrationHigh(f64), { format!("Cal,high,{:.*}", 2, cmd) }, 900, Ack
}

define_command! {
    doc: "`Cal,clear` command.",
    CalibrationClear, { "Cal,clear".to_string() }, 300, Ack
}

define_command! {
    doc: "`Cal,?` command. Returns a `CalibrationStatus` response.",
    CalibrationState, { "Cal,?".to_string() }, 300,
    resp: CalibrationStatus, { CalibrationStatus::parse(&resp) }
}

define_command! {
    doc: "`Export` command.",
    Export, { "Export".to_string() }, 300,
    resp: Exported, { Exported::parse(&resp) }
}

define_command! {
    doc: "`ExportInfo` command.",
    ExportInfo, { "Export,?".to_string() }, 300,
    resp: ExportedInfo, { ExportedInfo::parse(&resp) }
}

define_command! {
    doc: "`Import,n` command, where `n` is of type `String`.",
    cmd: Import(String), { format!("Import,{}", cmd) }, 300, Ack
}

define_command! {
    doc: "`Factory` command.",
    Factory, { "Factory".to_string() }, 0
}

define_command! {
    doc: "`Find` command.",
    Find, { "F".to_string() }, 300
}

define_command! {
    doc: "`I2C,n` command, where `n` is of type `u64`.",
    cmd: DeviceAddress(u16), { format!("I2C,{}", cmd) }, 300
}

define_command! {
    doc: "`I` command.",
    DeviceInformation, { "I".to_string() }, 300,
    resp: DeviceInfo, { DeviceInfo::parse(&resp) }
}

define_command! {
    doc: "`L,1` command.",
    LedOn, { "L,1".to_string() }, 300, Ack
}

define_command! {
    doc: "`L,0` command.",
    LedOff, { "L,0".to_string() }, 300, Ack
}

define_command! {
    doc: "`L,?` command.",
    LedState, { "L,?".to_string() }, 300,
    resp: LedStatus, { LedStatus::parse(&resp) }
}

define_command! {
    doc: "`Plock,1` command.",
    ProtocolLockEnable, { "Plock,1".to_string() }, 300, Ack
}

define_command! {
    doc: "`Plock,0` command.",
    ProtocolLockDisable, { "Plock,0".to_string() }, 300, Ack
}

define_command! {
    doc: "`Plock,?` command. Returns a `ProtocolLockStatus` response.",
    ProtocolLockState, { "Plock,?".to_string() }, 300,
    resp: ProtocolLockStatus, { ProtocolLockStatus::parse(&resp) }
}

define_command! {
    doc: "`R` command. Returns a `SensorReading` response.",
    Reading, { "R".to_string() }, 900,
    resp: SensorReading, { SensorReading::parse(&resp) }
}

define_command! {
    doc: "`Slope,?` command. Returns a `ProbeSlope` response.",
    Slope, { "Slope,?".to_string() }, 300,
    resp: ProbeSlope, { ProbeSlope::parse(&resp) }
}

define_command! {
    doc: "`Status` command. Returns a `DeviceStatus` response.",
    Status, { "Status".to_string() }, 300,
    resp: DeviceStatus, { DeviceStatus::parse(&resp) }
}

define_command! {
    doc: "`Sleep` command.",
    Sleep, { "Sleep".to_string() }, 0
}

define_command! {
    doc: "`T,t` command, where `t` is of type `f64`.",
    cmd: TemperatureCompensation(f64), { format!("T,{:.*}", 3, cmd) }, 300, Ack
}

define_command! {
    doc: "`T,?` command. Returns a `CompensationValue` response.",
    CompensatedTemperatureValue, { "T,?".to_string() }, 300,
    resp: CompensationValue, { CompensationValue::parse(&resp) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_command_uart_300() {
        let cmd = Baud(BpsRate::Bps300);
        assert_eq!(cmd.get_command_string(), "BAUD,300");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_300() {
        let cmd = "baud,300".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps300));

        let cmd = "BAUD,300".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps300));
    }

    #[test]
    fn build_command_baud_1200() {
        let cmd = Baud(BpsRate::Bps1200);
        assert_eq!(cmd.get_command_string(), "BAUD,1200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_1200() {
        let cmd = "baud,1200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps1200));

        let cmd = "BAUD,1200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps1200));
    }

    #[test]
    fn build_command_baud_2400() {
        let cmd = Baud(BpsRate::Bps2400);
        assert_eq!(cmd.get_command_string(), "BAUD,2400");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_2400() {
        let cmd = "baud,2400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps2400));

        let cmd = "BAUD,2400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps2400));
    }

    #[test]
    fn build_command_baud_9600() {
        let cmd = Baud(BpsRate::Bps9600);
        assert_eq!(cmd.get_command_string(), "BAUD,9600");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_9600() {
        let cmd = "baud,9600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps9600));

        let cmd = "BAUD,9600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps9600));
    }

    #[test]
    fn build_command_baud_19200() {
        let cmd = Baud(BpsRate::Bps19200);
        assert_eq!(cmd.get_command_string(), "BAUD,19200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_19200() {
        let cmd = "baud,19200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps19200));

        let cmd = "BAUD,19200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps19200));
    }

    #[test]
    fn build_command_baud_38400() {
        let cmd = Baud(BpsRate::Bps38400);
        assert_eq!(cmd.get_command_string(), "BAUD,38400");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_38400() {
        let cmd = "baud,38400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps38400));

        let cmd = "BAUD,38400".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps38400));
    }

    #[test]
    fn build_command_baud_57600() {
        let cmd = Baud(BpsRate::Bps57600);
        assert_eq!(cmd.get_command_string(), "BAUD,57600");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_57600() {
        let cmd = "baud,57600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps57600));

        let cmd = "BAUD,57600".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps57600));
    }

    #[test]
    fn build_command_baud_115200() {
        let cmd = Baud(BpsRate::Bps115200);
        assert_eq!(cmd.get_command_string(), "BAUD,115200");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_baud_115200() {
        let cmd = "baud,115200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps115200));

        let cmd = "BAUD,115200".parse::<Baud>().unwrap();
        assert_eq!(cmd, Baud(BpsRate::Bps115200));
    }

    #[test]
    fn build_command_calibration_mid() {
        let cmd = CalibrationMid(7.00);
        assert_eq!(cmd.get_command_string(), "CAL,MID,7.00");
        assert_eq!(cmd.get_delay(), 900);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_mid() {
        let cmd = "cal,mid,0".parse::<CalibrationMid>().unwrap();
        assert_eq!(cmd, CalibrationMid(0_f64));

        let cmd = "Cal,MID,121.43".parse::<CalibrationMid>().unwrap();
        assert_eq!(cmd, CalibrationMid(121.43));
    }

    #[test]
    fn build_command_calibration_low() {
        let cmd = CalibrationLow(4.);
        assert_eq!(cmd.get_command_string(), "CAL,LOW,4.00");
        assert_eq!(cmd.get_delay(), 900);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_low() {
        let cmd = "cal,low,0".parse::<CalibrationLow>().unwrap();
        assert_eq!(cmd, CalibrationLow(0_f64));

        let cmd = "Cal,loW,-121.43".parse::<CalibrationLow>().unwrap();
        assert_eq!(cmd, CalibrationLow(-121.43));
    }

    #[test]
    fn build_command_calibration_high() {
        let cmd = CalibrationHigh(10.056);
        assert_eq!(cmd.get_command_string(), "CAL,HIGH,10.06");
        assert_eq!(cmd.get_delay(), 900);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_high() {
        let cmd = "cal,high,0".parse::<CalibrationHigh>().unwrap();
        assert_eq!(cmd, CalibrationHigh(0_f64));

        let cmd = "Cal,HIGH,4121.43".parse::<CalibrationHigh>().unwrap();
        assert_eq!(cmd, CalibrationHigh(4121.43));
    }

    #[test]
    fn build_command_calibration_clear() {
        let cmd = CalibrationClear;
        assert_eq!(cmd.get_command_string(), "CAL,CLEAR");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_clear() {
        let cmd = "cal,clear".parse::<CalibrationClear>().unwrap();
        assert_eq!(cmd, CalibrationClear);

        let cmd = "Cal,CLEAR".parse::<CalibrationClear>().unwrap();
        assert_eq!(cmd, CalibrationClear);
    }

    #[test]
    fn build_command_calibration_state() {
        let cmd = CalibrationState;
        assert_eq!(cmd.get_command_string(), "CAL,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_calibration_state() {
        let cmd = "cal,?".parse::<CalibrationState>().unwrap();
        assert_eq!(cmd, CalibrationState);

        let cmd = "Cal,?".parse::<CalibrationState>().unwrap();
        assert_eq!(cmd, CalibrationState);
    }

    #[test]
    fn build_command_export() {
        let cmd = Export;
        assert_eq!(cmd.get_command_string(), "EXPORT");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_export() {
        let cmd = "export".parse::<Export>().unwrap();
        assert_eq!(cmd, Export);

        let cmd = "EXPORT".parse::<Export>().unwrap();
        assert_eq!(cmd, Export);
    }

    #[test]
    fn build_command_export_info() {
        let cmd = ExportInfo;
        assert_eq!(cmd.get_command_string(), "EXPORT,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_export_info() {
        let cmd = "export,?".parse::<ExportInfo>().unwrap();
        assert_eq!(cmd, ExportInfo);

        let cmd = "EXPORT,?".parse::<ExportInfo>().unwrap();
        assert_eq!(cmd, ExportInfo);
    }

    #[test]
    fn build_command_import() {
        let calibration_string = "ABCDEFGHIJKLMNO".to_string();
        let cmd = Import(calibration_string);
        assert_eq!(cmd.get_command_string(), "IMPORT,ABCDEFGHIJKLMNO");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_import() {
        let cmd = "import,1".parse::<Import>().unwrap();
        assert_eq!(cmd, Import("1".to_string()));

        let cmd = "IMPORT,abcdef".parse::<Import>().unwrap();
        assert_eq!(cmd, Import("ABCDEF".to_string()));
    }

    #[test]
    fn build_command_factory() {
        let cmd = Factory;
        assert_eq!(cmd.get_command_string(), "FACTORY");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_factory() {
        let cmd = "factory".parse::<Factory>().unwrap();
        assert_eq!(cmd, Factory);

        let cmd = "FACTORY".parse::<Factory>().unwrap();
        assert_eq!(cmd, Factory);
    }

    #[test]
    fn build_command_find() {
        let cmd = Find;
        assert_eq!(cmd.get_command_string(), "F");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_find() {
        let cmd = "f".parse::<Find>().unwrap();
        assert_eq!(cmd, Find);

        let cmd = "F".parse::<Find>().unwrap();
        assert_eq!(cmd, Find);
    }

    #[test]
    fn build_command_device_information() {
        let cmd = DeviceInformation;
        assert_eq!(cmd.get_command_string(), "I");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_device_information() {
        let cmd = "i".parse::<DeviceInformation>().unwrap();
        assert_eq!(cmd, DeviceInformation);

        let cmd = "I".parse::<DeviceInformation>().unwrap();
        assert_eq!(cmd, DeviceInformation);
    }

    #[test]
    fn build_command_change_device_address() {
        let cmd = DeviceAddress(88);
        assert_eq!(cmd.get_command_string(), "I2C,88");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_device_address() {
        let cmd = "i2c,1".parse::<DeviceAddress>().unwrap();
        assert_eq!(cmd, DeviceAddress(1));

        let cmd = "I2C,123".parse::<DeviceAddress>().unwrap();
        assert_eq!(cmd, DeviceAddress(123));
    }

    #[test]
    fn build_command_led_on() {
        let cmd = LedOn;
        assert_eq!(cmd.get_command_string(), "L,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_led_on() {
        let cmd = "l,1".parse::<LedOn>().unwrap();
        assert_eq!(cmd, LedOn);

        let cmd = "L,1".parse::<LedOn>().unwrap();
        assert_eq!(cmd, LedOn);
    }

    #[test]
    fn build_command_led_off() {
        let cmd = LedOff;
        assert_eq!(cmd.get_command_string(), "L,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_led_off() {
        let cmd = "l,0".parse::<LedOff>().unwrap();
        assert_eq!(cmd, LedOff);

        let cmd = "L,0".parse::<LedOff>().unwrap();
        assert_eq!(cmd, LedOff);
    }

    #[test]
    fn build_command_led_state() {
        let cmd = LedState;
        assert_eq!(cmd.get_command_string(), "L,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn build_command_plock_enable() {
        let cmd = ProtocolLockEnable;
        assert_eq!(cmd.get_command_string(), "PLOCK,1");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_plock_enable() {
        let cmd = "plock,1".parse::<ProtocolLockEnable>().unwrap();
        assert_eq!(cmd, ProtocolLockEnable);

        let cmd = "PLOCK,1".parse::<ProtocolLockEnable>().unwrap();
        assert_eq!(cmd, ProtocolLockEnable);
    }

    #[test]
    fn build_command_plock_disable() {
        let cmd = ProtocolLockDisable;
        assert_eq!(cmd.get_command_string(), "PLOCK,0");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_plock_disable() {
        let cmd = "plock,0".parse::<ProtocolLockDisable>().unwrap();
        assert_eq!(cmd, ProtocolLockDisable);

        let cmd = "PLOCK,0".parse::<ProtocolLockDisable>().unwrap();
        assert_eq!(cmd, ProtocolLockDisable);
    }

    #[test]
    fn build_command_plock_status() {
        let cmd = ProtocolLockState;
        assert_eq!(cmd.get_command_string(), "PLOCK,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_plock_status() {
        let cmd = "plock,?".parse::<ProtocolLockState>().unwrap();
        assert_eq!(cmd, ProtocolLockState);

        let cmd = "PLOCK,?".parse::<ProtocolLockState>().unwrap();
        assert_eq!(cmd, ProtocolLockState);
    }

    #[test]
    fn build_command_reading() {
        let cmd = Reading;
        assert_eq!(cmd.get_command_string(), "R");
        assert_eq!(cmd.get_delay(), 900);
    }

    #[test]
    fn parse_case_insensitive_command_reading() {
        let cmd = "r".parse::<Reading>().unwrap();
        assert_eq!(cmd, Reading);

        let cmd = "R".parse::<Reading>().unwrap();
        assert_eq!(cmd, Reading);
    }

    #[test]
    fn build_command_sleep_mode() {
        let cmd = Sleep;
        assert_eq!(cmd.get_command_string(), "SLEEP");
        assert_eq!(cmd.get_delay(), 0);
    }

    #[test]
    fn parse_case_insensitive_command_sleep() {
        let cmd = "Sleep".parse::<Sleep>().unwrap();
        assert_eq!(cmd, Sleep);

        let cmd = "SLEEP".parse::<Sleep>().unwrap();
        assert_eq!(cmd, Sleep);
    }

    #[test]
    fn build_command_slope() {
        let cmd = Slope;
        assert_eq!(cmd.get_command_string(), "SLOPE,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_slope() {
        let cmd = "Slope,?".parse::<Slope>().unwrap();
        assert_eq!(cmd, Slope);

        let cmd = "SLOPE,?".parse::<Slope>().unwrap();
        assert_eq!(cmd, Slope);
    }

    #[test]
    fn build_command_device_status() {
        let cmd = Status;
        assert_eq!(cmd.get_command_string(), "STATUS");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_device_status() {
        let cmd = "status".parse::<Status>().unwrap();
        assert_eq!(cmd, Status);

        let cmd = "STATUS".parse::<Status>().unwrap();
        assert_eq!(cmd, Status);
    }

    #[test]
    fn build_command_temperature_compensation() {
        let cmd = TemperatureCompensation(19.5);
        assert_eq!(cmd.get_command_string(), "T,19.500");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_temperature_compensation() {
        let cmd = "t,0".parse::<TemperatureCompensation>().unwrap();
        assert_eq!(cmd, TemperatureCompensation(0_f64));

        let cmd = "T,10.5".parse::<TemperatureCompensation>().unwrap();
        assert_eq!(cmd, TemperatureCompensation(10.5));
    }

    #[test]
    fn build_command_temperature_compensation_value() {
        let cmd = CompensatedTemperatureValue;
        assert_eq!(cmd.get_command_string(), "T,?");
        assert_eq!(cmd.get_delay(), 300);
    }

    #[test]
    fn parse_case_insensitive_command_temperature_compensation_value() {
        let cmd = "t,?".parse::<CompensatedTemperatureValue>().unwrap();
        assert_eq!(cmd, CompensatedTemperatureValue);

        let cmd = "T,?".parse::<CompensatedTemperatureValue>().unwrap();
        assert_eq!(cmd, CompensatedTemperatureValue);
    }
}
