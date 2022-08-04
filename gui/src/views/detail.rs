use imgui::{Ui, Window, Slider, InputFloat};
use rustodrive::state::ControlMode;

use crate::app_state::StateParam;

use super::charts::{plots, dropdown};

pub fn detail(state: &mut StateParam, ui: &Ui) {
    for (odrive_id, window_open) in state.ui.odrive_details.iter_mut() {
        if *window_open {
            Window::new(format!("ODrive {}", odrive_id))
            .size([400.0, 800.0], imgui::Condition::Always)
                .opened(window_open)
                .build(ui, || {
                    plots(ui);

                    dropdown(ui, "ODrive State", &mut state.ui.odrives_state);
                    ui.separator();

                    dropdown(ui, "Control Mode", &mut state.ui.odrives_control_mode);
                    match state.ui.odrives_control_mode {
                        ControlMode::VoltageControl => Slider::new("Voltage", 11.0, 24.0)
                            .build(ui, &mut state.ui.odrives_voltage),
                        ControlMode::TorqueControl => {
                            Slider::new("Torque", 0.0, 0.22).build(ui, &mut state.ui.odrives_torque)
                        }
                        ControlMode::VelocityControl => Slider::new("Velocity", 0.0, 50.0)
                            .build(ui, &mut state.ui.odrives_torque),
                        ControlMode::PositionControl => {
                            InputFloat::new(ui, "Position", &mut state.ui.odrives_position).build()
                        }
                    };
                    ui.separator();

                    dropdown(ui, "Input Mode", &mut state.ui.odrives_input_mode);
                    ui.separator();

                    if ui.button("Apply changes") {
                        state.app.set_all_states(&state.ui.odrives_state);
                        state.app.set_control_mode(&state.ui.odrives_control_mode);
                        state.app.set_input_mode(&state.ui.odrives_input_mode);
                    }
                });
        }
    }
}
