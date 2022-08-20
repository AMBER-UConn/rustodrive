use std::collections::HashMap;

use imgui::{Ui, Selectable, StyleColor};
use strum::IntoEnumIterator;

use crate::readings::{PlottableData};

type Label<'a> = &'a str;

/// This creates a plot that autoscales based on the minimum and maximum values of the plotted data
/// # Arguments
/// * `ui` - An UI object from the imgui-rs library
/// * `label` - The name of the plot
/// * `data` - a slice of f32s to plot
pub fn autoscale_plot(ui: &Ui, label: Label, data: &[f32]) {
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

/// This function creates a dropdown selector for an enum if it derives `IntoIter` and `strum_macros::Display`
/// # Arguments
/// * `ui` - An UI object from the imgui-rs library
/// * `label` - the name of the dropdown
/// * `selected` - This sets the passed reference to the value selected by the user
pub fn dropdown<T: std::fmt::Display + IntoEnumIterator>(ui: &Ui, label: Label, selected: &mut T) {
    if let Some(listbox) = ui.begin_combo(label, selected.to_string()) {
        for mode in T::iter() {
            if Selectable::new(mode.to_string()).build(ui) {
                *selected = mode;
            }
        }
        listbox.end()
    }
}


/// This draws a widget that allows you to select what plots to display on the screen
/// # Arguments
/// * `ui` - An UI object from the imgui-rs library
/// * `checked_state` - A hashmap of what data to plot and it's checked/visible state
/// * `options` - Information for displaying the appropriate plot, with a label, and passing the data to plot
/// 
/// # Example
/// ```rust 
/// let plot_state = HashMap::from([
///     (MotorTemp, false),
///     (InverterTemp, false),
///     (BusVoltage, true),
///     (BusCurrent, true),
/// ]),
/// plot_selectors(ui, plot_state, &[
///     (BusVoltage, "Voltage [V]", &[24.1, 23.99, 24.0]),
///     (BusCurrent, "Current [I]", &[3.2, 3.0, 3.1]),
///     (MotorTemp, "Motor Temperature °C", &[27.1, 27.0, 27.0]),
///     (InverterTemp, "Inverter Temperature °C", &[27.0, 27.0, 27.0]),
/// ])
/// ```
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