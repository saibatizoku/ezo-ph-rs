//! I2C Commands for pH EZO Chip, taken from their Datasheet.
//! This chip is used for pH measurement. It features calibration, sleep mode,
//! scale, etc.

pub enum PhCommand {
    // 'Baud' command
    Baud(u16),
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
