//! Parses I2C responses from the PH EZO Chip.
//!
//! Code modified from "Federico Mena Quintero <federico@gnome.org>"'s original.
use std::fmt;
use std::str::FromStr;

use super::{ErrorKind, EzoError};
use failure::ResultExt;

pub use ezo_common::response::{
    DeviceInfo, DeviceStatus, Exported, ExportedInfo, LedStatus, ProtocolLockStatus,
    ResponseStatus, RestartReason,
};

/// Minimum possible pH reading, per pH probe data sheet.
pub const PROBE_LOWER_LIMIT: f64 = 0.0;

/// Maximum possible pH reading, per pH probe data sheet.
pub const PROBE_UPPER_LIMIT: f64 = 14.0;


/// Calibration status of the PH EZO chip.
#[derive(Copy, Clone, PartialEq)]
pub enum CalibrationStatus {
    OnePoint,
    TwoPoint,
    ThreePoint,
    NotCalibrated,
}

impl CalibrationStatus {
    /// Parses the result of the "Cal,?" command to query the device's
    /// calibration status.  Returns ...
    pub fn parse(response: &str) -> Result<CalibrationStatus, EzoError> {
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

impl fmt::Debug for CalibrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalibrationStatus::OnePoint => write!(f, "?CAL,1"),
            CalibrationStatus::TwoPoint => write!(f, "?CAL,2"),
            CalibrationStatus::ThreePoint => write!(f, "?CAL,3"),
            CalibrationStatus::NotCalibrated => write!(f, "?CAL,0"),
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

/// A temperature reading
#[derive(Copy, Clone, PartialEq)]
pub struct SensorReading(pub f64);

impl SensorReading {
    /// Parses the result of the "R" command to get a temperature reading.
    /// Note that the returned value has no known units. It is your
    /// responsibility to know the current `TemperatureScale` setting.
    pub fn parse(response: &str) -> Result<SensorReading, EzoError> {
        let val = f64::from_str(response).context(ErrorKind::ResponseParse)?;

        match val {
            v if (v >= PROBE_LOWER_LIMIT) && (v <= PROBE_UPPER_LIMIT) => Ok(SensorReading(v)),

            _ => Err(ErrorKind::InvalidReading.into()),
        }
    }
}

impl fmt::Debug for SensorReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*}", 3, self.0)
    }
}

impl fmt::Display for SensorReading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*}", 3, self.0)
    }
}

/// Slope-points for the current sensor probe
#[derive(Copy, Clone, PartialEq)]
pub struct ProbeSlope {
    pub acid_end: f64,
    pub base_end: f64,
}

impl ProbeSlope {
    /// Parses the result of the "Slope,?" command to get the device's status.
    pub fn parse(response: &str) -> Result<ProbeSlope, EzoError> {
        if response.starts_with("?SLOPE,") {
            let num_str = response.get(7..).unwrap();

            let mut split = num_str.split(",");

            let acid_end = if let Some(acid_str) = split.next() {
                f64::from_str(acid_str).context(ErrorKind::ResponseParse)?
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            let base_end = if let Some(base_str) = split.next() {
                f64::from_str(base_str).context(ErrorKind::ResponseParse)?
            } else {
                return Err(ErrorKind::ResponseParse.into());
            };

            if let Some(_) = split.next() {
                return Err(ErrorKind::ResponseParse.into());
            }

            Ok(ProbeSlope { acid_end, base_end })
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Debug for ProbeSlope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?SLOPE,{:.*},{:.*}", 3, self.acid_end, 3, self.base_end)
    }
}

impl fmt::Display for ProbeSlope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.*},{:.*}", 3, self.acid_end, 3, self.base_end)
    }
}

/// Current temperature value used for pH compensation.
#[derive(Copy, Clone, PartialEq)]
pub struct CompensationValue(pub f64);

impl CompensationValue {
    /// Parses the result of the "T,?" command to get the device's
    /// temperature compensation value.
    pub fn parse(response: &str) -> Result<CompensationValue, EzoError> {
        if response.starts_with("?T,") {
            let rest = response.get(3..).unwrap();
            let val = f64::from_str(rest).context(ErrorKind::ResponseParse)?;
            Ok(CompensationValue(val))
        } else {
            Err(ErrorKind::ResponseParse.into())
        }
    }
}

impl fmt::Debug for CompensationValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "?T,{:.*}", 3, self.0)
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
        assert_eq!(
            CalibrationStatus::parse(&response).unwrap(),
            CalibrationStatus::OnePoint
        );

        let response = "?CAL,2";
        assert_eq!(
            CalibrationStatus::parse(&response).unwrap(),
            CalibrationStatus::TwoPoint
        );

        let response = "?CAL,3";
        assert_eq!(
            CalibrationStatus::parse(&response).unwrap(),
            CalibrationStatus::ThreePoint
        );

        let response = "?CAL,0";
        assert_eq!(
            CalibrationStatus::parse(&response).unwrap(),
            CalibrationStatus::NotCalibrated
        );
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
    fn parses_sensor_reading() {
        let response = "0";
        assert_eq!(SensorReading::parse(response).unwrap(), SensorReading(0.00));

        let response = "12.5";
        assert_eq!(
            SensorReading::parse(response).unwrap(),
            SensorReading(12.50)
        );

        let response = "14.0";
        assert_eq!(
            SensorReading::parse(response).unwrap(),
            SensorReading(14.00)
        );
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
        assert_eq!(
            ProbeSlope::parse(response).unwrap(),
            ProbeSlope {
                acid_end: 99.7,
                base_end: -100.3
            }
        );

        let response = "?SLOPE,97,-99.3";
        assert_eq!(
            ProbeSlope::parse(response).unwrap(),
            ProbeSlope {
                acid_end: 97.0,
                base_end: -99.3
            }
        );
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
    fn parses_temperature_compensation_value() {
        let response = "?T,14.56";
        assert_eq!(
            CompensationValue::parse(response).unwrap(),
            CompensationValue(14.56)
        );
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
