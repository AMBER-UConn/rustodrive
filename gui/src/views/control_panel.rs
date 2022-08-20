use crate::app_state::{StateParam, BackendState, self};
use imgui::{InputFloat, Slider, Ui, Window};
use rustodrive::state::{ControlMode};
use strum::IntoEnumIterator;
use crate::readings::{PlottableData::*, ODriveReadings};

use super::detail::ODriveDetailState;
use super::shared::{dropdown, plot_selectors};

pub struct ControlPanel {
    pub open: bool,
    pub odrives: ODriveDetailState,
}

impl ControlPanel {
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }
}


fn average_readings(sources: &[Vec<f32>]) -> Vec<f32> {
    let mut avg_readings = Vec::new();
    let num_readings = sources.get(0).expect("No odrives are connected, so no readings received").len();
    // TODO raise an error if somehow the sources have different lengths

    for reading_index in 0..num_readings {
        let mut total_val = 0.0;

        for data_src in sources {
            total_val += data_src[reading_index]
        }

        avg_readings.push(total_val);
    }

    return avg_readings;
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
            // Display changing color battery indicator
            let battery = state.app.battery;
            ui.text_colored(
                [2f32 * (1f32 - battery), 2f32 * battery, 0f32, 1f32],
                format!("{}% Battery", 100.0 * battery),
            );

            // Display current and voltage plots
            ui.text("Plots");
            let app_state = &state.app;

            let voltages: Vec<Vec<f32>> = app_state.odrive_data.keys().map(|id| app_state.get_prop_readings(id, |odrv| odrv.bus_voltage)).collect();
            let currents: Vec<Vec<f32>> = app_state.odrive_data.keys().map(|id| app_state.get_prop_readings(id, |odrv| odrv.bus_voltage)).collect();
            let motor_temps: Vec<Vec<f32>> = app_state.odrive_data.keys().map(|id| app_state.get_prop_readings(id, |odrv| odrv.bus_voltage)).collect();
            let inverter_temps: Vec<Vec<f32>> = app_state.odrive_data.keys().map(|id| app_state.get_prop_readings(id, |odrv| odrv.bus_voltage)).collect();
            
            plot_selectors(ui, &mut ctrl_panel.odrives.plottable_values, &[
                (BusVoltage, "Avg Voltage [V]", &average_readings(&voltages)),
                (BusCurrent, "Total Current [I]",&average_readings(&currents)),
                (MotorTemp, "Avg. Motor Temperature °C", &average_readings(&motor_temps)),
                (InverterTemp, "Avg. Inverter Temperature °C", &average_readings(&inverter_temps)),
            ]);

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
