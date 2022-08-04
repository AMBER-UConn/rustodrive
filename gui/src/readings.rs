use rustodrive::state::ODriveAxisState;

#[derive(Clone)]
pub struct ODriveReadings {
    pub can_id: usize,
    pub current_state: ODriveAxisState,
    // controller_status: ControllerStatus,
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
            self.can_id.to_string(),
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
