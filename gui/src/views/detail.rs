use imgui::{InputFloat, Slider, Ui, Window};
use rustodrive::state::{ControlMode, InputMode, ODriveAxisState};

use crate::app_state::AppState;

use super::control_panel::{dropdown, plots};

pub struct ODriveUIState {
    pub axis_state: ODriveAxisState,
    pub control_mode: ControlMode,
    pub input_mode: InputMode,
    pub control_mode_val: f32,
}

impl Default for ODriveUIState {
    fn default() -> Self {
        Self {
            axis_state: ODriveAxisState::Idle,
            control_mode: ControlMode::VelocityControl,
            input_mode: InputMode::Inactive,
            control_mode_val: 0.0,
        }
    }
}

pub struct ODriveDetail {
    pub open: bool,
    pub odrive: ODriveUIState,
}

pub fn detail(app_state: &mut AppState, odrive_id: &usize, odrive_detail: &mut ODriveDetail, ui: &Ui) {
        if !odrive_detail.open {
            return;
        }
        Window::new(format!("ODrive {}", odrive_id))
            .size([400.0, 800.0], imgui::Condition::Always)
            .opened(&mut odrive_detail.open)
            .build(ui, || {
                plots(ui);

                let odrive_ui = &mut odrive_detail.odrive;

                dropdown(ui, "ODrive State", &mut odrive_ui.axis_state);
                ui.separator();

                // If the control mode is switched, we need to reset the control mode value
                let before_mode = odrive_ui.control_mode.clone();
                dropdown(ui, "Control Mode", &mut odrive_ui.control_mode);
                if before_mode != odrive_ui.control_mode {
                    odrive_ui.control_mode_val = 0.0;
                }

                // Display the appropriate slider ranges depending on the mode
                match odrive_ui.control_mode {
                    ControlMode::VoltageControl => Slider::new("Voltage", 0.0, 24.0)
                        .build(ui, &mut odrive_ui.control_mode_val),
                    ControlMode::TorqueControl => {
                        Slider::new("Torque", 0.0, 0.22).build(ui, &mut odrive_ui.control_mode_val)
                    }

                    ControlMode::VelocityControl => Slider::new("Velocity", 0.0, 50.0)
                        .build(ui, &mut odrive_ui.control_mode_val),
                    ControlMode::PositionControl => {
                        InputFloat::new(ui, "Position", &mut odrive_ui.control_mode_val).build()
                    }
                };
                ui.separator();

                dropdown(ui, "Input Mode", &mut odrive_ui.input_mode);
                ui.separator();

                // Update the app state with the changes made to the UI
                if ui.button("Apply changes") {
                    app_state.set_all_states(&odrive_ui.axis_state);
                    app_state.set_control_mode(&odrive_ui.control_mode);
                    app_state.set_input_mode(&odrive_ui.input_mode);
                    app_state.set_control_val(&odrive_ui.control_mode_val);
                }
            });
}
