use std::collections::{HashMap, VecDeque};

use rand::Rng;
use rustodrive::{
    axis::AxisID,
    state::{ControlMode, InputMode, AxisState},
};

use crate::{readings::{ODriveReadings, PlottableData::*}, views::{control_panel::ControlPanel, detail::{ODriveDetailState, ODriveDetail}}};

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
        let control_panel_state = ODriveDetailState {
            plottable_values: HashMap::from([
                (MotorTemp, false),
                (InverterTemp, false),
                (BusVoltage, true),
                (BusCurrent, true),
            ]),
            ..Default::default()
        };
        Self {
            control_panel: ControlPanel { open: true, odrives: control_panel_state},
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

        let odrive_ui_state = ODriveDetailState {
            axis_state: odrive.current_state.clone(),
            control_mode: odrive.control_mode.clone(),
            input_mode: odrive.input_mode.clone(),
            control_mode_val: control_mode_val,
            ..Default::default()
        };

        self.details.insert(odrive.id, ODriveDetail { open: true, odrive: odrive_ui_state });
    }
}

pub struct AppState {
    pub odrive_data: HashMap<AxisID, VecDeque<ODriveReadings>>,
    pub battery: f32,
}



impl AppState {
    pub fn new(odrive_ids: &[usize]) -> Self {
        let mut odrive_data = HashMap::new();
        for id in odrive_ids {
            odrive_data.insert(*id, VecDeque::with_capacity(2000));
        } 

        Self {
            odrive_data: odrive_data,
            battery: rand::thread_rng().gen_range(0.0..1.0),
        }
    }
    pub fn set_all_states(&mut self, odrive_state: &AxisState) {
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

    /// For a specified odrive, this retrieves the property specified by the map function
    /// for all the readings
    /// 
    /// ## Example
    /// ```rust
    /// get_prop_readings(1, |odrive| odrive.position_estimate)
    /// ```
    /// Returns a Vec<f32> of all the position estimates for Axis 1
    pub fn get_prop_readings(&self, id: &usize, map_func: fn(&ODriveReadings) -> f32) -> Vec<f32> {
        let odrive = self.odrive_data.get(id).expect("Failed to get odrive with id");

        odrive.iter().map(|v| map_func(v)).collect::<Vec<f32>>()
    }

    pub fn add_reading(&mut self, reading: ODriveReadings) {
        let odrive = self.odrive_data.get_mut(&reading.id).expect("Failed to get odrive with id");
        if odrive.len() == odrive.capacity() {
            odrive.pop_front();
        }
        odrive.push_back(reading);
    }
}
