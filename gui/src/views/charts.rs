use std::f32::consts::PI;

use imgui::{Ui, Window, StyleColor, Selectable};
use rustodrive::state::{ODriveAxisState, ControlMode, InputMode};
use strum::IntoEnumIterator;
use crate::app_state::{AppState, StateParam, UIState};

fn sin_func(amplitude: f32, period: f32, time: f32, speed: f32) -> [f32; 200] {
    let mut data = [0.0; 200];
    for i in 0..data.len() {
        data[i] = amplitude * f32::sin((i as f32 + speed * time) / period)
    }
    data
}


pub fn plots(ui: &Ui) {
    let line_color = ui.push_style_color(StyleColor::PlotLines, [255f32, 0f32, 0f32, 1f32]);
    let current_data = &sin_func(10.0, 2.0 * PI,  ui.time() as f32, 5.0);
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
    if state.ui.control_panel {
        Window::new("charts").build(ui, || {
            plots(ui);
            dropdown(ui, "ODrive State", &mut state.ui.odrives_state);
            dropdown(ui, "Control Mode", &mut state.ui.odrives_control_mode);
            dropdown(ui, "Input Mode", &mut state.ui.odrives_input_mode);

            if ui.button("Apply changes") {
                state.app.set_all_states(&state.ui.odrives_state);
                state.app.set_control_mode(&state.ui.odrives_control_mode);
                state.app.set_input_mode(&state.ui.odrives_input_mode);
            }
        });
    }
}
