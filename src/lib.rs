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
}
