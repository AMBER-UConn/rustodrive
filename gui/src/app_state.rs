use std::collections::HashMap;

use rustodrive::{
    axis::AxisID,
    state::{ControlMode, InputMode, ODriveAxisState},
};

use crate::{readings::ODriveReadings, views::{control_panel::ControlPanel, detail::{ODriveUIState, ODriveDetail}}};

pub struct StateParam {
    pub ui: UIState,
    pub app: AppState,
}

pub struct UIState {
    pub control_panel: ControlPanel,
    pub details: HashMap<AxisID, ODriveDetail>,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            control_panel: ControlPanel { open: true, odrives: ODriveUIState::default()},
            details: HashMap::new(),
        }
    }

    pub fn detail_view(&mut self, odrive: &ODriveReadings) {
        let control_mode_val = match odrive.control_mode {
            ControlMode::VoltageControl => odrive.bus_voltage,
            ControlMode::TorqueControl => unimplemented!("Torque readings not currently supported by CAN"),
            ControlMode::VelocityControl => odrive.velocity_estimate,
            ControlMode::PositionControl => odrive.position_estimate,
        };

        let odrive_ui_state = ODriveUIState {
            axis_state: odrive.current_state,
            control_mode: odrive.control_mode,
            input_mode: odrive.input_mode,
            control_mode_val: control_mode_val,
        };

        self.details.insert(odrive.id, ODriveDetail { open: true, odrive: odrive_ui_state });
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
                id: i,
                current_state: ODriveAxisState::Idle,
                position_estimate: 3141f32,
                velocity_estimate: 21f32,
                shadow_count: 23414,
                encoder_count: 123,
                motor_temp: 31f32,
                inverter_temp: 30f32,
                bus_voltage: 22.31,
                bus_current: 3.12,
                input_mode: InputMode::VelRamp,
                control_mode: ControlMode::VelocityControl,
            });
        }

        Self {
            odrive_data: fake_odrive_data,
            battery: 0.2,
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

    pub fn set_control_val(&mut self, val: &f32) {
        println!("{}", val.to_string());
    }
}
