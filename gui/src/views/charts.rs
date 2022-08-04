use std::f32::consts::PI;

use imgui::{Ui, Window, StyleColor, Selectable};
use rustodrive::state::ODriveAxisState;
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

pub fn state_selector(selected: &mut ODriveAxisState, app: &mut AppState, ui: &Ui) {

    if let Some(listbox) = ui.begin_combo("Axes States", selected.to_string()) {
        
        for odrive_state in ODriveAxisState::iter() {
            if Selectable::new(odrive_state.to_string()).build(ui) {
                *selected = odrive_state;
            }
        }
        listbox.end()
    } 
    if ui.button("Set state") {
        app.set_all_states(&selected);
    }
}



pub fn control_panel(state: &mut StateParam, ui: &Ui) {
    let all_odrive_state = &mut state.ui.all_odrive_state;
    let app_state = &mut state.app;

    if state.ui.control_panel {
        Window::new("charts").opened(&mut state.ui.control_panel).build(ui, || {
            plots(ui);
            state_selector(all_odrive_state, app_state, ui);
        });

    }
}
