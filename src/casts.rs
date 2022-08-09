use crate::canframe::CANResponse;
use crate::error::{AxisError, EncoderError, MotorError, SensorlessError};
use crate::response::ODriveError;
use crate::state::{AxisState, ODriveCommand, ReadComm};
use crate::utils::ResponseManip;


#[derive(Debug, PartialEq)]
pub struct Heartbeat {
    axis_error: AxisError,
    current_state: AxisState,
} // Not including controller status due to lack of docs

#[derive(Debug, PartialEq)]
pub struct EncoderEstimates {
    position: f32,
    velocity: f32,
}

#[derive(Debug, PartialEq)]
pub struct EncoderCount {
    shadow_count: i32,
    cpr_count: i32,
}

#[derive(Debug, PartialEq)]
pub struct IQ {
    setpoint: f32,
    measured: f32,
}

#[derive(Debug, PartialEq)]
pub struct Temperature {
    inverter: f32,
    motor: f32,
}

#[derive(Debug, PartialEq)]
pub struct Bus {
    voltage: f32,
    current: f32,
}


impl TryFrom<CANResponse> for Heartbeat {
    type Error = ODriveError;

    fn try_from(response: CANResponse) -> Result<Self, Self::Error> {
        // Odrive CAN Signal: (0 1 2 3 4 5 6 7)
        // AxisError [32-bit]: (0 1 2 3)
        // Current State [8-bit]: (start bit 4)
        // Controller Status [8-bit]: (start bit 7)

        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::GetHeartbeat) {
            panic!("Cannot cast cmd {:?} into type Heartbeat", response.cmd)
        }

        let axis_err_bin: [u8; 4] = response.data[0..4].try_into().unwrap();
        let axis_state_bin: [u8; 1] = response.data[4..5].try_into().unwrap();

        // Try to convert the bytes. If it's bad data, return an error
        let axis_error = u32::from_le_bytes(axis_err_bin);
        let axis_error = match axis_error.try_into() {
            Ok(val) => val,
            Err(_) => return Err(ODriveError::ConvertedBadData),
        };

        // Try to convert the bytes. If it's bad data, return an error
        let current_state = u8::from_le_bytes(axis_state_bin);
        let current_state = match current_state.try_into() {
            Ok(val) => val,
            Err(_) => return Err(ODriveError::ConvertedBadData),
        };

        return Ok(Heartbeat { axis_error, current_state });
    }
}

impl TryFrom<CANResponse> for EncoderEstimates {
    type Error = ODriveError;

    fn try_from(response: CANResponse) -> Result<Self, Self::Error> {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::GetEncoderEstimates) {
            panic!("Cannot cast cmd {:?} into type Estimates", response.cmd)
        }

        let (position_bytes, velocity_bytes) = ResponseManip::split_32(response.data);
        
        return Ok(EncoderEstimates { 
            position: f32::from_le_bytes(position_bytes), 
            velocity: f32::from_le_bytes(velocity_bytes) 
        });
    }
}

impl TryFrom<CANResponse> for EncoderCount {
    type Error = ODriveError;

    fn try_from(response: CANResponse) -> Result<Self, Self::Error> {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::GetEncoderCount) {
            panic!("Cannot cast cmd {:?} into type Counts", response.cmd)
        }

        let (shadow_bytes, cpr_bytes) = ResponseManip::split_32(response.data);

        return Ok(EncoderCount { 
            shadow_count: i32::from_le_bytes(shadow_bytes), 
            cpr_count: i32::from_le_bytes(cpr_bytes)
        });
    }
}

impl TryFrom<CANResponse> for IQ {
    type Error = ODriveError;

    fn try_from(response: CANResponse) -> Result<Self, Self::Error> {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::GetIQ) {
            panic!("Cannot cast cmd {:?} into type IQ", response.cmd)
        }

        let (setpoint_bytes, measured_bytes) = ResponseManip::split_32(response.data);
        return Ok(Self {
            setpoint: f32::from_le_bytes(setpoint_bytes),
            measured: f32::from_le_bytes(measured_bytes),
        });
    }

}

impl TryFrom<CANResponse> for Temperature {
    type Error = ODriveError;

    fn try_from(response: CANResponse) -> Result<Self, Self::Error> {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::GetTemperature) {
            panic!("Cannot cast cmd {:?} into type Temperature", response.cmd)
        }

        let (inverter_temp_bytes, motor_temp_bytes) = ResponseManip::split_32(response.data);
        let inverter = f32::from_le_bytes(inverter_temp_bytes);
        let motor = f32::from_le_bytes(motor_temp_bytes);

        return Ok(Temperature { inverter, motor });
    }
}

impl TryFrom<CANResponse> for Bus {
    type Error = ODriveError;

    fn try_from(response: CANResponse) -> Result<Self, Self::Error> {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::GetVBusVoltage) {
            panic!("Cannot cast cmd {:?} into type Bus", response.cmd)
        }

        let (voltage_bytes, current_bytes) = ResponseManip::split_32(response.data);
        let voltage = f32::from_le_bytes(voltage_bytes);
        let current = f32::from_le_bytes(current_bytes);

        return Ok(Bus { voltage, current });
    }
}

impl TryFrom<CANResponse> for MotorError {
    type Error = ODriveError;

    fn try_from(response: CANResponse) -> Result<Self, Self::Error> {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::MotorError) {
            panic!("Cannot cast cmd {:?} into type MotorError", response.cmd)
        }

        // Try to convert the bytes. If it's bad data, return an error
        let motor_error = u64::from_le_bytes(response.data);
        return match motor_error.try_into() {
            Ok(val) => Ok(val),
            Err(_) => Err(ODriveError::ConvertedBadData),
        }
    }
}

impl TryFrom<CANResponse> for EncoderError {
    type Error = ODriveError;

    fn try_from(response: CANResponse) -> Result<Self, Self::Error> {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::EncoderError) {
            panic!("Cannot cast cmd {:?} into type EncoderError", response.cmd)
        }

        let (encoder_error_bytes, _) = ResponseManip::split_32(response.data);

        // Try to convert the bytes. If it's bad data, return an error
        let encoder_error = u32::from_le_bytes(encoder_error_bytes);
        return match encoder_error.try_into() {
            Ok(val) => Ok(val),
            Err(_) => Err(ODriveError::ConvertedBadData),
        };
    }
}

impl TryFrom<CANResponse> for SensorlessError {
    type Error = ODriveError;

    fn try_from(response: CANResponse) -> Result<Self, Self::Error> {
        // Check that the command can be converted into the proper type
        if response.cmd != ODriveCommand::Read(ReadComm::SensorlessError) {
            panic!( "Cannot cast cmd {:?} into type SensorlessError", response.cmd);
        }

        let (sensorless_error_bytes, _) = ResponseManip::split_32(response.data);
        // Try to convert the bytes. If it's bad data, return an error
        let sensorless_error = u32::from_le_bytes(sensorless_error_bytes);
        return match sensorless_error.try_into() {
            Ok(val) => Ok(val),
            Err(_) => Err(ODriveError::ConvertedBadData),
        };
    }
}

impl TryFrom<CANResponse> for () {
    type Error = ODriveError;

    fn try_from(response: CANResponse) -> Result<Self, ODriveError> {
        if let ODriveCommand::Read(_cmd) = response.cmd{
            panic!("Can only cast a Write command into () since it contains no response, not Read({:?})", response.cmd);
        }
        return Ok(());
    }
}

// TODO test remaining panics for improper data conversion
#[cfg(test)]
mod tests {
    use std::panic;

    use crate::{error::AxisError, state::{AxisState, ReadComm, ODriveCommand}, utils::ResponseManip, canframe::CANResponse};

    use super::{Heartbeat, EncoderEstimates, IQ, EncoderCount};

    #[test]
    fn test_to_heartbeat() {
        // Test that it successfully converts the CANResponse if it is the proper command
        let axis_error_bytes = u32::to_le_bytes(AxisError::SystemLevel as u32);
        let axis_state = u32::to_le_bytes(AxisState::ClosedLoop as u32);
        
        let combined = ResponseManip::combine_32(axis_error_bytes, axis_state);

        let fake_response = CANResponse {
            axis: 1,
            cmd: ODriveCommand::Read(ReadComm::GetHeartbeat),
            data: combined,
        };

        let expected = Heartbeat { axis_error: AxisError::SystemLevel, current_state: AxisState::ClosedLoop };

        assert_eq!(TryInto::<Heartbeat>::try_into(fake_response).unwrap(), expected); 
    }

    #[test]
    fn test_to_heartbeat_panic() {
        // Test that it fails to convert to heartbeat if it is the wrong type
        let result_wrong_type = panic::catch_unwind(|| {
            let axis_error_bytes = u32::to_le_bytes(AxisError::SystemLevel as u32);
            let axis_state = u32::to_le_bytes(AxisState::ClosedLoop as u32);
            
            let combined = ResponseManip::combine_32(axis_error_bytes, axis_state);
    
            let fake_response = CANResponse {
                axis: 1,
                cmd: ODriveCommand::Read(ReadComm::GetHeartbeat),
                data: combined,
            };
    
            TryInto::<EncoderCount>::try_into(fake_response).unwrap();

        });

        assert!(result_wrong_type.is_err());
    }

    #[test]
    fn test_to_encoder_estimate() {

        let encoder_pos_est = f32::to_le_bytes(0.0);
        let encoder_vel_est = f32::to_le_bytes(10.0);

        let dat = ResponseManip::combine_32(encoder_pos_est, encoder_vel_est);

        let fake_response = CANResponse {
            axis: 1,
            cmd: ODriveCommand::Read(ReadComm::GetEncoderEstimates),
            data: dat,
        };
        
        let expected = EncoderEstimates { position: 0.0, velocity: 10.0 };

        assert_eq!(TryInto::<EncoderEstimates>::try_into(fake_response).unwrap(), expected); 

    }

    #[test]
    fn test_encoder_count() {

        let s_c = 69420;
        let cpr = 8192;
        let shadow_count = i32::to_le_bytes(s_c);
        let cpr_count = i32::to_le_bytes(s_c / cpr);

        let dat = ResponseManip::combine_32(shadow_count, cpr_count);

        let fake_response = CANResponse {
            axis: 1,
            cmd: ODriveCommand::Read(ReadComm::GetEncoderCount),
            data: dat,
        };

        let expected = EncoderCount { shadow_count: s_c, cpr_count: s_c/cpr };

        assert_eq!(TryInto::<EncoderCount>::try_into(fake_response).unwrap(), expected); 
    }

    #[test]
    fn test_IQ() {

        let iq_setpoint = f32::to_le_bytes(100.0);
        let iq_measured = f32::to_le_bytes(0.0);

        let dat = ResponseManip::combine_32(iq_setpoint, iq_measured);

        let fake_response = CANResponse {
            axis: 1,
            cmd: ODriveCommand::Read(ReadComm::GetIQ),
            data: dat,
        };
        
        let expected = IQ { setpoint: 100.0, measured: 0.0 };

        assert_eq!(TryInto::<IQ>::try_into(fake_response).unwrap(), expected); 

    }

}
