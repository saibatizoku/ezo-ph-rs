//! I2C Commands for pH EZO Chip.
//!
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use errors::*;
use response::{
    CalibrationStatus,
    CompensationValue,
    ProbeSlope,
    SensorReading,
};

use ezo_common::{
    ResponseCode,
    response_code,
    string_from_response_data,
    write_to_ezo,
};
use ezo_common::response::ResponseStatus;

use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CDevice;

/// Maximum ascii-character response size + 2
pub const MAX_DATA: usize = 401;

/// I2C command for the EZO chip.
pub use ezo_common::Command;
pub use ezo_common::command::*;


define_command! {
    doc: "`CAL,MID,t` command, where `t` is of type `f64`.",
    cmd: CalibrationMid(f64), { format!("CAL,MID,{:.*}", 2, cmd) }, 900, Ack
}

impl FromStr for CalibrationMid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("CAL,MID,") {
            let rest = supper.get(8..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) => {
                    n.parse::<f64>()
                        .chain_err(|| ErrorKind::CommandParse)?
                }
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(CalibrationMid(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`CAL,LOW,t` command, where `t` is of type `f64`.",
    cmd: CalibrationLow(f64), { format!("CAL,LOW,{:.*}", 2, cmd) }, 900, Ack
}

impl FromStr for CalibrationLow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("CAL,LOW,") {
            let rest = supper.get(8..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) => {
                    n.parse::<f64>()
                        .chain_err(|| ErrorKind::CommandParse)?
                }
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(CalibrationLow(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`CAL,HIGH,t` command, where `t` is of type `f64`.",
    cmd: CalibrationHigh(f64), { format!("CAL,HIGH,{:.*}", 2, cmd) }, 900, Ack
}

impl FromStr for CalibrationHigh {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("CAL,HIGH,") {
            let rest = supper.get(9..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) => {
                    n.parse::<f64>()
                        .chain_err(|| ErrorKind::CommandParse)?
                }
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(CalibrationHigh(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`CAL,?` command. Returns a `CalibrationStatus` response.",
    CalibrationState, { "CAL,?".to_string() }, 300,
    resp: CalibrationStatus, { CalibrationStatus::parse(&resp) }
}

impl FromStr for CalibrationState {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "CAL,?" => Ok(CalibrationState),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`R` command. Returns a `SensorReading` response.",
    Reading, { "R".to_string() }, 900,
    resp: SensorReading, { SensorReading::parse(&resp) }
}

impl FromStr for Reading {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "R" => Ok(Reading),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`SLOPE,?` command. Returns a `ProbeSlope` response.",
    Slope, { "SLOPE,?".to_string() }, 300,
    resp: ProbeSlope, { ProbeSlope::parse(&resp) }
}

impl FromStr for Slope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "SLOPE,?" => Ok(Slope),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

define_command! {
    doc: "`T,t` command, where `t` is of type `f64`.",
    cmd: TemperatureCompensation(f64), { format!("T,{:.*}", 3, cmd) }, 300, Ack
}

impl FromStr for TemperatureCompensation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        if supper.starts_with("T,") {
            let rest = supper.get(2..).unwrap();
            let mut split = rest.split(',');
            let value = match split.next() {
                Some(n) => {
                    n.parse::<f64>()
                        .chain_err(|| ErrorKind::CommandParse)?
                }
                _ => bail!(ErrorKind::CommandParse),
            };
            match split.next() {
                None => return Ok(TemperatureCompensation(value)),
                _ => bail!(ErrorKind::CommandParse),
            }
        } else {
            bail!(ErrorKind::CommandParse);
        }
    }
}

define_command! {
    doc: "`T,?` command. Returns a `CompensationValue` response.",
    CompensatedTemperatureValue, { "T,?".to_string() }, 300,
    resp: CompensationValue, { CompensationValue::parse(&resp) }
}

impl FromStr for CompensatedTemperatureValue {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let supper = s.to_uppercase();
        match supper.as_ref() {
            "T,?" => Ok(CompensatedTemperatureValue),
            _ => bail!(ErrorKind::CommandParse),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn parse_invalid_command_calibration_mid_yields_err() {
        let cmd = "cal,mid,".parse::<CalibrationMid>();
        assert!(cmd.is_err());

        let cmd = "CAL,MID,1a21.43".parse::<CalibrationMid>();
        assert!(cmd.is_err());
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
    fn parse_invalid_command_calibration_low_yields_err() {
        let cmd = "cal,low,".parse::<CalibrationLow>();
        assert!(cmd.is_err());

        let cmd = "CAL,LOW,1a21.43".parse::<CalibrationLow>();
        assert!(cmd.is_err());
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
    fn parse_invalid_command_calibration_high_yields_err() {
        let cmd = "cal,high,".parse::<CalibrationHigh>();
        assert!(cmd.is_err());

        let cmd = "CAL,High,1a21.43".parse::<CalibrationHigh>();
        assert!(cmd.is_err());
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
    fn parse_case_insensitive_command_temperature_compensation() {
        let cmd = "t,0".parse::<TemperatureCompensation>().unwrap();
        assert_eq!(cmd, TemperatureCompensation(0_f64));

        let cmd = "T,10.5".parse::<TemperatureCompensation>().unwrap();
        assert_eq!(cmd, TemperatureCompensation(10.5));
    }

    #[test]
    fn parse_invalid_command_temperature_compensation_yields_err() {
        let cmd = "T,".parse::<TemperatureCompensation>();
        assert!(cmd.is_err());

        let cmd = "T,$".parse::<TemperatureCompensation>();
        assert!(cmd.is_err());

        let cmd = "T,1a21.43".parse::<TemperatureCompensation>();
        assert!(cmd.is_err());
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
