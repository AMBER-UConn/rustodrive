use std::{collections::HashMap};

use rustodrive::{axis::AxisID, state::{ODriveAxisState, ControlMode, InputMode}};

use crate::readings::ODriveReadings;


pub struct StateParam {
    pub ui: UIState,
    pub app: AppState
}

pub struct UIState {
    pub control_panel: bool,
    pub odrive_details: HashMap<AxisID, bool>,
    pub odrives_state: ODriveAxisState,
    pub odrives_control_mode: ControlMode,
    pub odrives_input_mode: InputMode,
    pub odrives_velocity: f32,
    pub odrives_position: f32,
    pub odrives_torque: f32,
    pub odrives_voltage: f32
}

impl UIState {
    pub fn new() -> Self {
        Self {
            control_panel: false,
            odrive_details: HashMap::new(),
            odrives_state: ODriveAxisState::Idle,
            odrives_control_mode: ControlMode::VelocityControl,
            odrives_input_mode: InputMode::Inactive,
            odrives_velocity: 0.0,
            odrives_position: 0.0,
            odrives_torque: 0.0,
            odrives_voltage: 0.0,
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
    pub battery: f32,
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
            battery: 0.2
        }
    }
    pub fn set_all_states(&mut self, odrive_state: &ODriveAxisState) {
        println!("{}", odrive_state.to_string());
    }

    pub fn set_control_mode(&mut self, control_mode: &ControlMode) {
        println!("{}", control_mode.to_string());
    }

    pub fn set_input_mode(&mut self, input_mode: &InputMode) {
        println!("{}", input_mode.to_string());
    }
}
