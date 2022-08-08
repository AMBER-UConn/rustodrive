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
        PhaseResistanceOFR = 0x1,
        PhaseInductanceOFR = 0x2,
        DRVFault = 0x8,
        ControlDeadlineMissed = 0x10,
        ModulationMagnitude = 0x80,
        CurrentSenseSaturation = 0x400,
        CurrentLimitViolation = 0x1000,
        ModulationIsNAN = 0x10000,
        MotorThermistorOverTemp = 0x20000,
        FetThermistorOverTemp = 0x40000,
        TimerUpdateMissed = 0x80000,
        CurrentMeasurementUnavailable = 0x100000,
        ControllerFailed = 0x200000,
        IBusOFR = 0x400000,
        BrakeResistorDisarmed = 0x800000,
        SystemLevel = 0x1000000,
        BadTiming = 0x2000000,
        UnknownPhaseEstimate = 0x4000000,
        UnknownPhaseVel = 0x8000000,
        UnknownTorque = 0x10000000,
        UnknownCurrentCommand = 0x20000000,
        UnknownCurrentMeasurement = 0x40000000,
        UnknownVBusVoltage = 0x80000000,
        UnknownVoltageCommand = 0x100000000,
        UnknownGains = 0x200000000,
        ControllerInitializing = 0x400000000,
        UnbalancedPhases = 0x800000000,
    }
}

back_to_enum! { u32,
    pub enum EncoderError {
        UnstableGain = 0x1,
        CPRPolepairsMismatch = 0x2,
        NoResponse = 0x4,
        UnsupportedEncoderMode = 0x8,
        IllegalHallState = 0x10,
        IndexNotFoundYet = 0x20,
        AbsSpiTimeout = 0x40,
        AbsSpiComFail = 0x80,
        AbsSpiNotReady = 0x100,
        HallNotCalibratedYet = 0x200,
    }
}

back_to_enum! { u32,
    pub enum SensorlessError {
        UnstableGain = 0x1,
        UnknownCurrentMeasurement = 0x2,
    }
}