use std::collections::HashMap;

use imgui::{Ui, Selectable, StyleColor};
use strum::IntoEnumIterator;

use crate::readings::{PlottableData};

pub fn sin_animation(amplitude: f32, period: f32, time: f32) -> f32 {
    return amplitude * f32::sin(time / period);
}
pub fn sin_func_arr(amplitude: f32, period: f32, time: f32, speed: f32) -> [f32; 200] {
    let mut data = [0.0; 200];
    for i in 0..data.len() {
        data[i] = amplitude * f32::sin((i as f32 + speed * time) / period)
    }
    data
}

pub fn autoscale_plot(ui: &Ui, label: &str, data: &[f32]) {
    let line_color = ui.push_style_color(StyleColor::PlotLines, [255f32, 0f32, 0f32, 1f32]);
    
    let max_val = data.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).expect("couldn't find max of graph");
    let min_val = data.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)).expect("couldn't find min of graph");
    let margin = 0.1 * (max_val + min_val) / 2.0;

    ui.plot_lines(label, data)
        .graph_size([0f32, 75f32])
        .scale_max(max_val + margin)
        .scale_min(min_val - margin)
        .overlay_text(format!("Value: {}", data.last().unwrap()))
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
type Label<'a> = &'a str;
pub fn plot_selectors(ui: &Ui, checked_state: &mut HashMap<PlottableData, bool>, options: &[(PlottableData, Label, &[f32])]){
    // Display the plots that were selected
    for (data_type, label, data) in options.iter() {
        let displayed = checked_state.get(data_type).expect("Got checkbox option that didn't exist");
        if *displayed {
            autoscale_plot(ui, label, data);
        }
    }

    // For each option create a checkbox to set which plots should be enabled
    for (data_type, label, _) in options.iter() {
        ui.checkbox(label, checked_state.get_mut(data_type).unwrap());
    }

    ui.separator();
}