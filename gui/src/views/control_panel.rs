use std::f32::consts::PI;

use crate::app_state::StateParam;
use imgui::{InputFloat, Selectable, Slider, StyleColor, Ui, Window};
use rustodrive::state::{ControlMode};
use strum::IntoEnumIterator;

use super::detail::ODriveUIState;

pub struct ControlPanel {
    pub open: bool,
    pub odrives: ODriveUIState,
}

impl ControlPanel {
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }
}

fn sin_func(amplitude: f32, period: f32, time: f32, speed: f32) -> [f32; 200] {
    let mut data = [0.0; 200];
    for i in 0..data.len() {
        data[i] = amplitude * f32::sin((i as f32 + speed * time) / period)
    }
    data
}

pub fn plots(ui: &Ui) {
    let line_color = ui.push_style_color(StyleColor::PlotLines, [255f32, 0f32, 0f32, 1f32]);
    let current_data = &sin_func(10.0, 2.0 * PI, ui.time() as f32, 5.0);
    ui.plot_lines("Total Current [I]", current_data)
        .graph_size([0f32, 75f32])
        .scale_max(15f32)
        .scale_min(-0.5)
        .overlay_text(format!("Value: {}", current_data.last().unwrap()))
        .build();

    let voltage_data = &sin_func(24.0, 2.0 * PI, ui.time() as f32, 5.0);

    ui.plot_lines("Avg Voltage [V]", voltage_data)
        .graph_size([0f32, 75f32])
        .scale_max(15f32)
        .scale_min(-0.5)
        .overlay_text(format!("Value: {}", voltage_data.last().unwrap()))
        .build();
    line_color.pop();
}

pub fn dropdown<T: std::fmt::Display + IntoEnumIterator>(ui: &Ui, label: &str, selected: &mut T) {
    if let Some(listbox) = ui.begin_combo(label, selected.to_string()) {
        for mode in T::iter() {
            if Selectable::new(mode.to_string()).build(ui) {
                *selected = mode;
            }
        }
        listbox.end()
    }
}

pub fn control_panel(state: &mut StateParam, ui: &Ui) {
    let ctrl_panel = &mut state.ui.control_panel;
    if !ctrl_panel.open {
        return;
    }

    Window::new("All ODrives Control Panel")
        .size([400f32, 800f32], imgui::Condition::Always)
        .opened(&mut ctrl_panel.open)
        .build(ui, || {
            let battery = state.app.battery;
            ui.text_colored(
                [2f32 * (1f32 - battery), 2f32 * battery, 0f32, 1f32],
                format!("{}% Battery", 100.0 * battery),
            );

            plots(ui);

            let odrive_ui = &mut ctrl_panel.odrives;

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
                ControlMode::VoltageControl => {
                    Slider::new("Voltage", 11.0, 24.0).build(ui, &mut odrive_ui.control_mode_val)
                }
                ControlMode::TorqueControl => {
                    Slider::new("Torque", 0.0, 0.22).build(ui, &mut odrive_ui.control_mode_val)
                }
                ControlMode::VelocityControl => {
                    Slider::new("Velocity", 0.0, 50.0).build(ui, &mut odrive_ui.control_mode_val)
                }
                ControlMode::PositionControl => {
                    InputFloat::new(ui, "Position", &mut odrive_ui.control_mode_val).build()
                }
            };
            ui.separator();

            dropdown(ui, "Input Mode", &mut odrive_ui.input_mode);
            ui.separator();

            if ui.button("Apply changes") {
                state.app.set_all_states(&odrive_ui.axis_state);
                state.app.set_control_mode(&odrive_ui.control_mode);
                state.app.set_input_mode(&odrive_ui.input_mode);
                state.app.set_control_val(&odrive_ui.control_mode_val);
            }
        });
}
