use crate::app_state::StateParam;
use imgui::{InputFloat, Slider, Ui, Window};
use rustodrive::state::{ControlMode};
use strum::IntoEnumIterator;
use crate::readings::PlottableData::*;

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
            plot_selectors(ui, &mut ctrl_panel.odrives.plottable_values, &[
                (BusVoltage, "Avg Voltage [V]", &app_state.map(|odrv| odrv.bus_voltage)),
                (BusCurrent, "Total Current [I]", &app_state.map(|odrv| odrv.bus_current)),
                (MotorTemp, "Avg. Motor Temperature °C", &app_state.map(|odrv| odrv.motor_temp)),
                (InverterTemp, "Avg. Inverter Temperature °C", &app_state.map(|odrv| odrv.inverter_temp)),
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
