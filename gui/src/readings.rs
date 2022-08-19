use rustodrive::state::{AxisState, InputMode, ControlMode};

#[derive(Clone)]
pub struct ODriveReadings {
    pub id: usize,
    pub current_state: AxisState,
    pub input_mode: InputMode,
    pub control_mode: ControlMode,
    // axis_error: AxisError
    // motor_error: MotorError,
    pub position_estimate: f32,
    pub velocity_estimate: f32,
    pub shadow_count: i32,
    pub encoder_count: i32,
    pub motor_temp: f32,
    pub inverter_temp: f32,
    pub bus_voltage: f32,
    pub bus_current: f32,
}

impl ODriveReadings {
    pub fn as_columns(&self) -> [String; 10] {
        [
            self.id.to_string(),
            self.current_state.to_string(),
            self.position_estimate.to_string(),
            self.velocity_estimate.to_string(),
            self.shadow_count.to_string(),
            self.encoder_count.to_string(),
            self.motor_temp.to_string(),
            self.inverter_temp.to_string(),
            self.bus_voltage.to_string(),
            self.bus_current.to_string(),
        ]
    }
}


#[derive(PartialEq, Eq, Hash)]
pub enum PlottableData {
    PosEstimate,
    VelEstimate,
    ShadowCount,
    EncoderCount,
    MotorTemp,
    InverterTemp,
    BusVoltage,
    BusCurrent
}
