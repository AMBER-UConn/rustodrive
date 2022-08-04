use std::collections::{HashMap};

use rustodrive::{axis::AxisID, state::ODriveAxisState};

use crate::readings::ODriveReadings;
pub struct AppState {
    pub control_panel: bool,
    pub odrive_details: HashMap<AxisID, bool>,
    pub odrive_data: Vec<ODriveReadings>,
}

impl AppState {
    pub fn new() -> Self {
        let mut fake_odrive_data = vec![];
        for i in 0..4 {
            fake_odrive_data.push(ODriveReadings {
                can_id: i,
                current_state: ODriveAxisState::Idle,
                position_estimate: 3141f32,
                velocity_estimate: 21f32,
                shadow_count: 23414,
                encoder_count: 123,
                motor_temp: 31f32,
                inverter_temp: 30f32,
                bus_voltage: 22.31,
                bus_current: 3.12,
            });
        }

        Self {
            control_panel: false,
            odrive_details: HashMap::new(),
            odrive_data: fake_odrive_data,
        }
    }

    pub fn toggle_ctrl_panel(&mut self) {
        self.control_panel = !self.control_panel;
    }

    pub fn detail_view(&mut self, odrive: &AxisID) {
        self.odrive_details.insert(*odrive, true);
    }
}
