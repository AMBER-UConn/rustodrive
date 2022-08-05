#![allow(dead_code)]
#![allow(unused_variables)]

use crate::{
    canframe::{ticket, CANRequest},
    state::{
        ControlMode, InputMode, ODriveAxisState,
        ODriveCommand::{Read, Write},
        ReadComm::*,
        WriteComm::*,
    },
    utils::ResponseManip as RData,
};

pub type AxisID = usize;

/// This struct contains methods that can generate common `ODriveCANFrame` configurations.
/// The [`Motor`] and [`Encoder`] objects are publicly accessible and define their own
/// frame-generating methods.
pub struct Axis<'a> {
    id: &'a AxisID,
    pub motor: Motor<'a>,
    pub encoder: Encoder<'a>,
}

impl<'a> Axis<'a> {
    pub fn new(id: &'a AxisID) -> Self {
        Axis {
            id,
            motor: Motor::new(id),
            encoder: Encoder::new(id),
        }
    }


    pub fn get_heartbeat(&self) -> (u32, u8) {
        let tk =
        ticket(
            *self.id, 
            Read(Heartbeat), 
            [0; 8],
        );
        
        let (axis_error_bin, cstate_controller_bin) = RData::split_32(tk.data);
        let (current_state_bin, controller_status_bin) = RData::split_16(cstate_controller_bin);

        // Odrive CAN Signal: (0 1 2 3 4 5 6 7)
        // Axis Error [32-bit]: (0 1 2 3)
        // Current State [8-bit]: (4)
        // Controller Status [8-bit]: (7)

        return (
            u32::from_le_bytes(axis_error_bin),
            u8::from_le_bytes(RData::split_8(current_state_bin).0),
            // TODO - Implement Controller Status Bitfield Return
        );
    }
    pub fn get_axis_error(&self) -> u32 {self.get_heartbeat().0}
    pub fn get_current_state(&self) -> u8 {self.get_heartbeat().1}

    /// This generates the command to set the state for the `Axis` object in question
    pub fn set_state(&self, state: ODriveAxisState) -> CANRequest {
        ticket(
            *self.id,
            Write(SetAxisRequestedState),
            [state as u8, 0, 0, 0, 0, 0, 0, 0],
        )
    }

    pub fn get_temperatures(&self) -> (f32, f32) {
        let tk =
        ticket(
            *self.id,
            Read(GetTemperature),
            [0; 8],
        );

        let (inverter_temp_bin, motor_temp_bin) = RData::split_32(tk.data);

        return (
            f32::from_le_bytes(inverter_temp_bin),
            f32::from_le_bytes(motor_temp_bin),
        )
    }
    pub fn get_inverter_temperature(&self) -> f32 {self.get_temperatures().0}
    pub fn get_motor_temperature(&self) -> f32 {self.get_temperatures().1}
    
    //pub fn set_control_mode
}

pub struct Encoder<'a> {
    id: &'a AxisID,
}
impl<'a> Encoder<'a> {
    pub fn new(id: &'a AxisID) -> Self {
        Encoder { id }
    }
    pub fn get_error(&self) -> u32 {
        let tk =
        ticket(
            *self.id,
            Read(EncoderError),
            [0; 8],
        );

        u32::from_le_bytes(RData::split_32(tk.data).0)
    }

    pub fn get_count(&self) -> (i32, i32) {
        let tk =
        ticket(
            *self.id, 
            Read(GetEncoderCount), 
            [0; 8],
        );
        let (shadow_count_bin, cpr_count_bin) = RData::split_32(tk.data);

        return (
            i32::from_le_bytes(shadow_count_bin),
            i32::from_le_bytes(shadow_count_bin),
        )
    }
    pub fn get_shadow_count(&self) -> i32 {self.get_count().0}
    pub fn get_cpr_count(&self) -> i32 {self.get_count().1}

    pub fn get_estimate(&self) -> (f32, f32) {
        let tk = 
        ticket(
            *self.id,
            Read(GetEncoderEstimates),
            [0; 8],
        );
        let (pos_est_bin, vel_est_bin) = RData::split_32(tk.data);


        return (
            f32::from_le_bytes(pos_est_bin),
            f32::from_le_bytes(vel_est_bin),
        )
    }
    pub fn get_pos_estimate(&self) -> f32 {self.get_estimate().0}
    pub fn get_vel_estimate(&self) -> f32 {self.get_estimate().1}

    fn set_linear_count() {
        unimplemented!()
    }
}

struct Trajectory;
impl Trajectory {
    fn set_traj_vel_limit() {
        unimplemented!()
    }
    fn set_traj_accel_limit() {
        unimplemented!()
    }
    fn set_traj_inertia() {
        unimplemented!()
    }
}

pub struct Motor<'a> {
    id: &'a AxisID,
}
impl<'a> Motor<'a> {
    pub fn new(id: &'a AxisID) -> Self {
        Motor { id }
    }

    pub fn get_error(&self) -> u64 {
        let tk =
        ticket(
            *self.id,
            Read(MotorError),
            [0; 8],
        );

        u64::from_le_bytes(tk.data)
    }
    pub fn get_sensorless_error(&self) -> u32 {
        let tk =
        ticket(
            *self.id,
            Read(SensorlessError),
            [0; 8],
        );

        u32::from_le_bytes(RData::split_32(tk.data).0)
    }

    fn set_node_id() {
        unimplemented!()
    }
    pub fn set_control_mode(&self, control: ControlMode, input: InputMode) -> CANRequest {
        ticket(
            *self.id,
            Write(SetControllerMode),
            [control as u8, 0, 0, 0, input as u8, 0, 0, 0],
        )
    }

    pub fn set_input_pos(&self, rot: f32) -> CANRequest {
        let data = RData::combine_32(rot.to_le_bytes(), [0; 4]);
        ticket(
            *self.id, 
            Write(SetInputPosition), 
            data
        )
    }
    pub fn set_input_vel(&self, speed: f32) -> CANRequest {
        let data = RData::combine_32(speed.to_le_bytes(), [0; 4]);
        ticket(
            *self.id, 
            Write(SetInputVelocity), 
            data
        )
    }
    fn set_input_torque() {
        unimplemented!()
    }

    fn set_limits() {
        unimplemented!()
    } // velocity and current limit

    fn get_iq_setpoint() {
        unimplemented!()
    }

    fn set_position_gain() {
        unimplemented!()
    }
    fn set_vel_gain() {
        unimplemented!()
    }

    fn get_sensorless_estimates() {
        unimplemented!()
    }
}
