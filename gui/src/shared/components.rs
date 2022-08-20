use imgui::{Ui, Slider, InputFloat};
use rustodrive::state::ControlMode;



use super::{widgets::dropdown, state::ODriveDetailState};

/// This function renders an axis state/control mode/input mode selector component
pub fn odrive_mode_component(ui: &Ui, odrive_gui_state: &mut ODriveDetailState) {
    dropdown(ui, "ODrive State", &mut odrive_gui_state.axis_state);
    ui.separator();

    // If the control mode is switched, we need to reset the control mode value
    let before_mode = odrive_gui_state.control_mode.clone();
    dropdown(ui, "Control Mode", &mut odrive_gui_state.control_mode);
    if before_mode != odrive_gui_state.control_mode {
        odrive_gui_state.control_mode_val = 0.0;
    }

    // Display the appropriate slider ranges depending on the control mode
    match odrive_gui_state.control_mode {
        ControlMode::VoltageControl => {
            Slider::new("Voltage", 11.0, 24.0).build(ui, &mut odrive_gui_state.control_mode_val)
        }
        ControlMode::TorqueControl => {
            Slider::new("Torque", 0.0, 0.22).build(ui, &mut odrive_gui_state.control_mode_val)
        }
        ControlMode::VelocityControl => {
            Slider::new("Velocity", 0.0, 50.0).build(ui, &mut odrive_gui_state.control_mode_val)
        }
        ControlMode::PositionControl => {
            InputFloat::new(ui, "Position", &mut odrive_gui_state.control_mode_val).build()
        }
    };
    ui.separator();

    // Displays the input mode selector
    dropdown(ui, "Input Mode", &mut odrive_gui_state.input_mode);
    ui.separator();
}
