/// I2C Commands for pH EZO Chip, taken from their Datasheet.
/// This chip is used for pH measurement. It features calibration, sleep mode,
/// scale, etc.
enum PhEzoCommand {
    Baud(u16),
    CalibrationClear,
    CalibrationSingleMidPoint(f64),
    CalibrationThreePointAtHigh(f64),
    CalibrationTwoPointAtLow(f64),
    DeviceAddress(u8),
    DeviceInformation,
    Export,
    ExportInfo,
    Factory,
    Find,
    Import(String),
    LedOn,
    LedOff,
    LedState,
    ProtocolLockDisable,
    ProtocolLockEnable,
    ProtocolLockStatus,
    Reading,
    Sleep,
    Slope,
    Status,
    TemperatureCompensation,
    TemperatureCompensatedValue,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
