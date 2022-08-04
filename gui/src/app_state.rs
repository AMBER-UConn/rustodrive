use std::{collections::HashMap, cell::{Cell}};

use rustodrive::{axis::AxisID, state::ODriveAxisState};

use crate::readings::ODriveReadings;


pub struct StateParam {
    pub ui: UIState,
    pub app: AppState
}

pub struct UIState {
    pub control_panel: bool,
    pub odrive_details: HashMap<AxisID, bool>,
    pub all_odrive_state: ODriveAxisState,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            control_panel: false,
            odrive_details: HashMap::new(),
            all_odrive_state: ODriveAxisState::Idle
        }
    }

    pub fn toggle_ctrl_panel(&mut self) {
        // let is_displayed = self.control_panel.get();
        // self.control_panel.replace(!is_displayed);
        self.control_panel = !self.control_panel;
    }

    pub fn detail_view(&mut self, odrive: &AxisID) {
        self.odrive_details.insert(*odrive, true);
    }
}

pub struct AppState {
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
            odrive_data: fake_odrive_data,
        }
    }
    pub fn set_all_states(&mut self, odrive_state: &ODriveAxisState) {
        println!("{}", odrive_state.to_string());
    }
}
