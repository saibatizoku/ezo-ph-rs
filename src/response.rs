//! Parses I2C responses from the PH EZO Chip.
//!
//! Code modified from "Federico Mena Quintero <federico@gnome.org>"'s original.
use std::fmt;
use std::str::FromStr;

use errors::*;

/// Minimum possible pH reading, per pH probe data sheet.
pub const PROBE_LOWER_LIMIT: f64 = 0.0;

/// Maximum possible pH reading, per pH probe data sheet.
pub const PROBE_UPPER_LIMIT: f64 = 14.0;

/// Calibration status of the PH EZO chip.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CalibrationStatus {
    OnePoint,
    TwoPoint,
    ThreePoint,
    NotCalibrated,
}

impl CalibrationStatus {
    /// Parses the result of the "Cal,?" command to query the device's
    /// calibration status.  Returns ...
    pub fn parse(response: &str) -> Result<CalibrationStatus> {
        if response.starts_with("?CAL,") {
            let rest = response.get(5..).unwrap();
            let mut split = rest.split(',');

            let _calibration = match split.next() {
                Some("3") => Ok(CalibrationStatus::ThreePoint),
                Some("2") => Ok(CalibrationStatus::TwoPoint),
                Some("1") => Ok(CalibrationStatus::OnePoint),
                Some("0") => Ok(CalibrationStatus::NotCalibrated),
                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            match split.next() {
                None => _calibration,
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for CalibrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalibrationStatus::OnePoint => write!(f, "one-point"),
            CalibrationStatus::TwoPoint => write!(f, "two-point"),
            CalibrationStatus::ThreePoint => write!(f, "three-point"),
            CalibrationStatus::NotCalibrated => write!(f, "none"),
        }
    }
}

/// Exported calibration string of the PH EZO chip.
#[derive(Debug, Clone, PartialEq)]
pub enum Exported {
    ExportString(String),
    Done,
}

impl Exported {
    pub fn parse(response: &str) -> Result<Exported> {
        if response.starts_with("*") {
            match response {
                "*DONE" => Ok(Exported::Done),
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            match response.len() {
                1..13 => Ok(Exported::ExportString(response.to_string())),
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        }
    }
}

impl fmt::Display for Exported {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Exported::ExportString(ref s) => write!(f, "{}", s),
            Exported::Done => write!(f, "DONE"),
        }
    }
}

/// Export the current calibration settings of the PH EZO chip.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExportedInfo {
    pub lines: u16,
    pub total_bytes: u16,
}

impl ExportedInfo {
    pub fn parse(response: &str) -> Result<ExportedInfo> {
        if response.starts_with("?EXPORT,") {
            let num_str = response.get(8..).unwrap();

            let mut split = num_str.split(",");

            let lines = if let Some(lines_str) = split.next() {
                u16::from_str(lines_str)
                    .chain_err(|| ErrorKind::ResponseParse)?
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            let total_bytes = if let Some(totalbytes_str) = split.next() {
                u16::from_str(totalbytes_str)
                    .chain_err(|| ErrorKind::ResponseParse)?
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            if let Some(_) = split.next() {
                return Err(ErrorKind::ResponseParse.into());
            }

            Ok (ExportedInfo { lines, total_bytes } )
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for ExportedInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.lines, self.total_bytes)
    }
}

/// Current firmware settings of the PH EZO chip.
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceInfo {
    pub device: String,
    pub firmware: String,
}

impl DeviceInfo {
    pub fn parse(response: &str) -> Result<DeviceInfo> {
        if response.starts_with("?I,") {
            let rest = response.get(3..).unwrap();
            let mut split = rest.split(',');

            let device = if let Some(device_str) = split.next() {
                device_str.to_string()
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            let firmware = if let Some(firmware_str) = split.next() {
                firmware_str.to_string()
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            if let Some(_) = split.next() {
                return Err(ErrorKind::ResponseParse.into());
            }

            Ok (DeviceInfo { device, firmware } )

        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for DeviceInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.device, self.firmware)
    }
}

/// Status of PH EZO's LED.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LedStatus {
    Off,
    On,
}

impl LedStatus {
    pub fn parse(response: &str) -> Result<LedStatus> {
        if response.starts_with("?L,") {
            let rest = response.get(3..).unwrap();

            match rest {
                "1" => Ok(LedStatus::On),
                "0" => Ok(LedStatus::Off),
                _ => return Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for LedStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LedStatus::On => write!(f, "on"),
            LedStatus::Off => write!(f, "off"),
        }
    }
}

/// Status of I2C protocol lock.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProtocolLockStatus {
    Off,
    On,
}

impl ProtocolLockStatus {
    pub fn parse(response: &str) -> Result<ProtocolLockStatus> {
        if response.starts_with("?PLOCK,") {
            let rest = response.get(7..).unwrap();
            let mut split = rest.split(',');

            let _plock_status = match split.next() {
                Some("1") => Ok(ProtocolLockStatus::On),
                Some("0") => Ok(ProtocolLockStatus::Off),
                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            match split.next() {
                None => _plock_status,
                _ => Err(ErrorKind::ResponseParse.into()),
            }
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for ProtocolLockStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProtocolLockStatus::On => write!(f, "on"),
            ProtocolLockStatus::Off => write!(f, "off"),
        }
    }
}

/// A temperature reading
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SensorReading(pub f64);

impl SensorReading {
    /// Parses the result of the "R" command to get a temperature reading.
    /// Note that the returned value has no known units. It is your
    /// responsibility to know the current `TemperatureScale` setting.
    pub fn parse(response: &str) -> Result<SensorReading> {
        let val = f64::from_str(response).chain_err(|| ErrorKind::ResponseParse)?;

        match val {
            v if (v >= PROBE_LOWER_LIMIT) && (v <= PROBE_UPPER_LIMIT) => Ok ( SensorReading(v) ),

            _ => Err ( ErrorKind::InvalidReading.into() ),
        }
    }
}

impl fmt::Display for SensorReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*}", 3, self.0)
    }
}

/// Slope-points for the current sensor probe
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ProbeSlope {
    pub acid_end: f64,
    pub base_end: f64,
}

impl ProbeSlope {
    /// Parses the result of the "Slope,?" command to get the device's status.
    pub fn parse(response: &str) -> Result<ProbeSlope> {
        if response.starts_with("?SLOPE,") {
            let num_str = response.get(7..).unwrap();

            let mut split = num_str.split(",");

            let acid_end = if let Some(acid_str) = split.next() {
                f64::from_str(acid_str)
                    .chain_err(|| ErrorKind::ResponseParse)?
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            let base_end = if let Some(base_str) = split.next() {
                f64::from_str(base_str)
                    .chain_err(|| ErrorKind::ResponseParse)?
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            if let Some(_) = split.next() {
                return Err(ErrorKind::ResponseParse.into());
            }

            Ok ( ProbeSlope { acid_end, base_end } )
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for ProbeSlope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*},{:.*}", 3, self.acid_end, 3, self.base_end)
    }
}

/// Reason for which the device restarted, data sheet pp. 58
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum RestartReason {
    PoweredOff,
    SoftwareReset,
    BrownOut,
    Watchdog,
    Unknown,
}

impl fmt::Display for RestartReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RestartReason::PoweredOff => write!(f, "powered-off"),
            RestartReason::SoftwareReset => write!(f, "software-reset"),
            RestartReason::BrownOut => write!(f, "brown-out"),
            RestartReason::Watchdog => write!(f, "watchdog"),
            RestartReason::Unknown => write!(f, "unknown"),
        }
    }
}

/// Response from the "Status" command to get the device status
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DeviceStatus {
    pub restart_reason: RestartReason,
    pub vcc_voltage: f64,
}

impl DeviceStatus {
    /// Parses the result of the "Status" command to get the device's status.
    pub fn parse(response: &str) -> Result<DeviceStatus> {
        if response.starts_with("?STATUS,") {
            let rest = response.get(8..).unwrap();
            let mut split = rest.split(',');

            let restart_reason = match split.next() {
                Some("P") => RestartReason::PoweredOff,
                Some("S") => RestartReason::SoftwareReset,
                Some("B") => RestartReason::BrownOut,
                Some("W") => RestartReason::Watchdog,
                Some("U") => RestartReason::Unknown,
                _ => return Err(ErrorKind::ResponseParse.into()),
            };

            let voltage = if let Some(voltage_str) = split.next() {
                f64::from_str(voltage_str)
                    .chain_err(|| ErrorKind::ResponseParse)?
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            if let Some(_) = split.next() {
                return Err(ErrorKind::ResponseParse.into());
            }

            Ok(DeviceStatus {
                   restart_reason: restart_reason,
                   vcc_voltage: voltage,
               })
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{:.*}", self.restart_reason, 3, self.vcc_voltage)
    }
}

/// Current temperature value used for pH compensation.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CompensationValue(pub f64);

impl CompensationValue {
    /// Parses the result of the "T,?" command to get the device's
    /// temperature compensation value.
    pub fn parse(response: &str) -> Result<CompensationValue> {
        if response.starts_with("?T,") {
            let rest = response.get(3..).unwrap();
            let val = f64::from_str(rest).chain_err(|| ErrorKind::ResponseParse)?;
            Ok ( CompensationValue(val) )
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Display for CompensationValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*}", 3, self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_calibration_status() {
        let response = "?CAL,1";
        assert_eq!(CalibrationStatus::parse(&response).unwrap(),
                   CalibrationStatus::OnePoint);

        let response = "?CAL,2";
        assert_eq!(CalibrationStatus::parse(&response).unwrap(),
                   CalibrationStatus::TwoPoint);

        let response = "?CAL,3";
        assert_eq!(CalibrationStatus::parse(&response).unwrap(),
                   CalibrationStatus::ThreePoint);

        let response = "?CAL,0";
        assert_eq!(CalibrationStatus::parse(&response).unwrap(),
                   CalibrationStatus::NotCalibrated);
    }

    #[test]
    fn parsing_invalid_calibration_status_yields_error() {
        let response = "";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,2.";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,-1";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,4";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,b";
        assert!(CalibrationStatus::parse(&response).is_err());

        let response = "?CAL,1,";
        assert!(CalibrationStatus::parse(&response).is_err());
    }

    #[test]
    fn parses_data_export_string() {
        let response = "123456789012";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::ExportString("123456789012".to_string()));

        let response = "myresponse";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::ExportString("myresponse".to_string()));

        let response = "*DONE";
        assert_eq!(Exported::parse(response).unwrap(),
                   Exported::Done);
    }

    #[test]
    fn parsing_invalid_export_string_yields_error() {
        let response = "*";
        assert!(Exported::parse(response).is_err());

        let response = "*DONE*";
        assert!(Exported::parse(response).is_err());

        let response = "**DONE";
        assert!(Exported::parse(response).is_err());

        let response = "12345678901234567890";
        assert!(Exported::parse(response).is_err());
    }

    #[test]
    fn parses_export_info() {
        let response = "?EXPORT,0,0";
        assert_eq!(ExportedInfo::parse(response).unwrap(),
                   ExportedInfo { lines: 0, total_bytes: 0 } );
    }

    #[test]
    fn parsing_invalid_export_info_yields_error() {
        let response = "?EXPORT,11,120,10";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "?EXPORT,1012";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "10,*DON";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "12,";
        assert!(ExportedInfo::parse(response).is_err());

        let response = "";
        assert!(ExportedInfo::parse(response).is_err());
    }

    #[test]
    fn parses_device_information() {
        let response = "?I,PH,1.98";
        assert_eq!(DeviceInfo::parse(response).unwrap(),
                   DeviceInfo {
                       device: "PH".to_string(),
                       firmware: "1.98".to_string(),
                   } );

        let response = "?I,,";
        assert_eq!(DeviceInfo::parse(response).unwrap(),
                   DeviceInfo {
                       device: "".to_string(),
                       firmware: "".to_string(),
                   } );

    }

    #[test]
    fn parsing_invalid_device_info_yields_error() {
        let response = "";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "?I";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "?I,";
        assert!(DeviceInfo::parse(response).is_err());

        let response = "?I,a,b,c";
        assert!(DeviceInfo::parse(response).is_err());
    }

    #[test]
    fn parses_led_status() {
        let response = "?L,1";
        assert_eq!(LedStatus::parse(&response).unwrap(),
                   LedStatus::On);

        let response = "?L,0";
        assert_eq!(LedStatus::parse(&response).unwrap(),
                   LedStatus::Off);
    }

    #[test]
    fn parsing_invalid_led_status_yields_error() {
        let response = "";
        assert!(LedStatus::parse(&response).is_err());

        let response = "?L,";
        assert!(LedStatus::parse(&response).is_err());

        let response = "?L,b";
        assert!(LedStatus::parse(&response).is_err());

        let response = "?L,17";
        assert!(LedStatus::parse(&response).is_err());
    }

    #[test]
    fn parses_protocol_lock_status() {
        let response = "?PLOCK,1";
        assert_eq!(ProtocolLockStatus::parse(&response).unwrap(),
                   ProtocolLockStatus::On);

        let response = "?PLOCK,0";
        assert_eq!(ProtocolLockStatus::parse(&response).unwrap(),
                   ProtocolLockStatus::Off);
    }

    #[test]
    fn parsing_invalid_protocol_lock_status_yields_error() {
        let response = "";
        assert!(ProtocolLockStatus::parse(&response).is_err());

        let response = "?PLOCK,57";
        assert!(ProtocolLockStatus::parse(&response).is_err());

        let response = "?PLOCK,b";
        assert!(ProtocolLockStatus::parse(&response).is_err());

        let response = "?PLOCK,b,1";
        assert!(ProtocolLockStatus::parse(&response).is_err());
    }

    #[test]
    fn parses_sensor_reading() {
        let response = "0";
        assert_eq!(SensorReading::parse(response).unwrap(),
                   SensorReading(0.00));

        let response = "12.5";
        assert_eq!(SensorReading::parse(response).unwrap(),
                   SensorReading(12.50));

        let response = "14.0";
        assert_eq!(SensorReading::parse(response).unwrap(),
                   SensorReading(14.00));
    }

    #[test]
    fn parsing_invalid_sensor_reading_yields_error() {
        let response = "";
        assert!(SensorReading::parse(response).is_err());

        let response = "-x";
        assert!(SensorReading::parse(response).is_err());

        let response = "-0.5";
        assert!(SensorReading::parse(response).is_err());

        let response = "10.5.5";
        assert!(SensorReading::parse(response).is_err());

        let response = "14.1";
        assert!(SensorReading::parse(response).is_err());
    }

    #[test]
    fn parses_probe_slope() {
        let response = "?SLOPE,99.7,-100.3";
        assert_eq!(ProbeSlope::parse(response).unwrap(),
                   ProbeSlope { acid_end: 99.7, base_end: -100.3 });

        let response = "?SLOPE,97,-99.3";
        assert_eq!(ProbeSlope::parse(response).unwrap(),
                   ProbeSlope { acid_end: 97.0, base_end: -99.3 });
    }

    #[test]
    fn parsing_invalid_probe_slope_yields_error() {
        let response = "?SLOPE,";
        assert!(ProbeSlope::parse(response).is_err());

        let response = "?SLOPE,-1,";
        assert!(ProbeSlope::parse(response).is_err());

        let response = "?SLOPE,1,foo";
        assert!(ProbeSlope::parse(response).is_err());

        let response = "?SLOPE,1,foo,";
        assert!(ProbeSlope::parse(response).is_err());
    }

    #[test]
    fn parses_device_status() {
        let response = "?STATUS,P,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::PoweredOff,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,S,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::SoftwareReset,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,B,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::BrownOut,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,W,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::Watchdog,
                       vcc_voltage: 1.5,
                   });

        let response = "?STATUS,U,1.5";
        assert_eq!(DeviceStatus::parse(response).unwrap(),
                   DeviceStatus {
                       restart_reason: RestartReason::Unknown,
                       vcc_voltage: 1.5,
                   });
    }

    #[test]
    fn parsing_invalid_device_status_yields_error() {
        let response = "";
        assert!(DeviceStatus::parse(response).is_err());

        let response = "?STATUS,X,";
        assert!(DeviceStatus::parse(response).is_err());

        let response = "?STATUS,P,1.5,";
        assert!(DeviceStatus::parse(response).is_err());
    }

    #[test]
    fn parses_temperature_compensation_value() {
        let response = "?T,14.56";
        assert_eq!(CompensationValue::parse(response).unwrap(),
                   CompensationValue(14.56));
    }

    #[test]
    fn parsing_invalid_temperature_compensation_value_yields_error() {
        let response = "";
        assert!(CompensationValue::parse(response).is_err());

        let response = "?T,";
        assert!(CompensationValue::parse(response).is_err());

        let response = "?T,X.00";
        assert!(CompensationValue::parse(response).is_err());

        let response = "?T,1.2,43";
        assert!(CompensationValue::parse(response).is_err());
    }
}
