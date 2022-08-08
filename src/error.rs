use crate::back_to_enum;

// See documentation: https://docs.odriverobotics.com/v/latest/fibre_types/com_odriverobotics_ODrive.html?highlight=error#ODrive.Error
back_to_enum! { u32,
    pub enum AxisError { 
        Initializing = 0x1,
        SystemLevel = 0x2,
        TimingError = 0x4,
        MissingEstimate = 0x8,
        BadConfig = 0x10,
        DrvFault = 0x20,
        DCBusOverVoltage = 0x100,
        DCBusUnderVoltage = 0x200,
        DCBusOverCurrent = 0x400,
        DCBusOverRegenCurrent = 0x800,
        CurrentLimitViolation = 0x1000,
        MotorOverTemp = 0x2000,
        InverterOverTemp = 0x4000,
        VelocityLimitViolation = 0x8000,
        PositionLimitViolation = 0x10000,
        WatchdogTimerExpired = 0x1000000,
        EStopRequested = 0x2000000,
        SpinoutDetected = 0x4000000,
        OtherDeviceFailed = 0x8000000,
    }
}


back_to_enum! { u64,
    pub enum MotorError {
    }
}

back_to_enum! { u32,
    pub enum EncoderError {

    }
}

back_to_enum! { u32,
    pub enum SensorlessError {

    }
}