use crate::canframe::CANResponse;
use crate::error::{AxisError, EncoderError, MotorError, SensorlessError};
use crate::state::{AxisState, ODriveCommand, ReadComm};
use crate::utils::ResponseManip;

pub struct Heartbeat {
    axis_error: AxisError,
    current_state: AxisState,
} // Not including controller status due to lack of docs

pub struct EncoderEstimates {
    position: f32,
    velocity: f32,
}
pub struct EncoderCount {
    shadow_count: i32,
    cpr_count: i32,
}
pub struct IQ {
    setpoint: f32,
    measured: f32,
}
pub struct Temperature {
    inverter: f32,
    motor: f32,
}
pub struct Bus {
    voltage: f32,
    current: f32,
}

impl From<CANResponse> for Heartbeat {
    fn from(response: CANResponse) -> Self {
        // Odrive CAN Signal: (0 1 2 3 4 5 6 7)
        // Axis Error [32-bit]: (0 1 2 3)
        // Current State [8-bit]: (start bit 4)
        // Controller Status [8-bit]: (start bit 7)

        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::Heartbeat) {
            panic!("Cannot cast cmd {:?} into type Heartbeat", response.cmd)
        }

        let axis_err_bin: [u8; 4] = response.data[0..4].try_into().unwrap();
        let axis_state_bin: [u8; 1] = response.data[4..5].try_into().unwrap();

        let axis_err = u32::from_le_bytes(axis_err_bin);
        let current_state = u8::from_le_bytes(axis_state_bin);

        return Heartbeat {
            axis_error: axis_err
                .try_into()
                .expect("Failed to convert bytes to AxisError"),
            current_state: current_state
                .try_into()
                .expect("Failed to convert bytes into AxisState"),
        };
    }
}

impl From<CANResponse> for EncoderEstimates {
    fn from(response: CANResponse) -> Self {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::GetEncoderEstimates) {
            panic!("Cannot cast cmd {:?} into type Estimates", response.cmd)
        }

        let (position_bytes, velocity_bytes) = ResponseManip::split_32(response.data);

        return EncoderEstimates { 
                            position: f32::from_le_bytes(position_bytes), 
                            velocity: f32::from_le_bytes(velocity_bytes) 
                        };
    }
}

impl From<CANResponse> for EncoderCount {
    fn from(response: CANResponse) -> Self {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::GetEncoderCount) {
            panic!("Cannot cast cmd {:?} into type Counts", response.cmd)
        }

        let (shadow_bytes, cpr_bytes) = ResponseManip::split_32(response.data);

        return EncoderCount { 
                                shadow_count: i32::from_le_bytes(shadow_bytes), 
                                cpr_count: i32::from_le_bytes(cpr_bytes)
                            };
    }
}

impl From<CANResponse> for IQ {
    fn from(response: CANResponse) -> Self {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::GetIQSetpoint) {
            panic!("Cannot cast cmd {:?} into type IQ", response.cmd)
        }

        let (setpoint_bytes, measured_bytes) = ResponseManip::split_32(response.data);
        return Self {
            setpoint: f32::from_le_bytes(setpoint_bytes),
            measured: f32::from_le_bytes(measured_bytes),
        };
    }
}

impl From<CANResponse> for Temperature {
    fn from(response: CANResponse) -> Self {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::GetTemperature) {
            panic!("Cannot cast cmd {:?} into type Temperature", response.cmd)
        }

        let (inverter_temp_bytes, motor_temp_bytes) = ResponseManip::split_32(response.data);
        let inverter = f32::from_le_bytes(inverter_temp_bytes);
        let motor = f32::from_le_bytes(motor_temp_bytes);

        return Temperature { inverter, motor };
    }
}

impl From<CANResponse> for Bus {
    fn from(response: CANResponse) -> Self {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::GetVBusVoltage) {
            panic!("Cannot cast cmd {:?} into type Bus", response.cmd)
        }

        let (voltage_bytes, current_bytes) = ResponseManip::split_32(response.data);
        let voltage = f32::from_le_bytes(voltage_bytes);
        let current = f32::from_le_bytes(current_bytes);

        return Bus { voltage, current };
    }
}

impl From<CANResponse> for MotorError {
    fn from(response: CANResponse) -> Self {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::MotorError) {
            panic!("Cannot cast cmd {:?} into type MotorError", response.cmd)
        }

        return u64::from_le_bytes(response.data)
            .try_into()
            .expect("Failed to convert bytes to MotorError");
    }
}

impl From<CANResponse> for EncoderError {
    fn from(response: CANResponse) -> Self {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::EncoderError) {
            panic!("Cannot cast cmd {:?} into type EncoderError", response.cmd)
        }

        let (encoder_error_bytes, _) = ResponseManip::split_32(response.data);

        return u32::from_le_bytes(encoder_error_bytes)
            .try_into()
            .expect("Failed to convert bytes to EncoderError");
    }
}

impl From<CANResponse> for SensorlessError {
    fn from(response: CANResponse) -> Self {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::MotorError) {
            panic!( "Cannot cast cmd {:?} into type SensorlessError", response.cmd);
        }

        let (encoder_error_bytes, _) = ResponseManip::split_32(response.data);

        return u32::from_le_bytes(encoder_error_bytes)
            .try_into()
            .expect("Failed to convert bytes to SensorlessError");
    }
}

impl From<CANResponse> for () {
    fn from(response: CANResponse) -> Self {
        if let ODriveCommand::Read(_cmd) = response.cmd{
            panic!("Can only cast a Write command into () since it contains no response, not Read({:?})", response.cmd);
        }
        return ();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_to_heartbeat() {
        
    }

    #[test]
    fn test_to_encoder_estimate() {

    }

    #[test]
    fn test_encoder_count() {

    }

    #[test]
    fn test_IQ() {

    }
}
