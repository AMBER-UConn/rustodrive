use crate::readings::PlottableData::*;
use crate::state::{BackendState, StateParam};
use imgui::{InputFloat, Slider, Ui, Window};
use rustodrive::axis::AxisID;
use rustodrive::state::ControlMode;
use strum::IntoEnumIterator;

use super::detail::{DisplayedPlots, ODriveDetailState};
use super::shared::{dropdown, plot_selectors};

// -------------------------- View/Window State --------------------------
/// State for the control panel window. We use [`ODriveDetailState`] even though this
/// is the control panel since it uses the same fields/data
pub struct ControlPanel {
    pub open: bool,
    pub odrives: ODriveDetailState,
}

impl ControlPanel {
    pub fn toggle(&mut self) {
        self.open = !self.open;
    }
}
// -----------------------------------------------------------------------

/// This function computes a Vec of average readings given
/// multiple data sources with the same number of readings
///
/// # Example
/// ```
/// average_readings(&[
///     vec![1.0, 2.0, 0.0, 0.0],
///     vec![0.0, 0.0, 0.0, 0.0],
/// ])
/// // outputs [0.5, 1.0, 0.0, 0.0]
/// ```
fn average_readings(sources: &[Vec<f32>]) -> Vec<f32> {
    let num_readings = sources
        .get(0)
        .expect("No odrives are connected, so no readings received")
        .len();

    // Raise an error if sources have different reading lengths
    for src in sources.iter() {
        if src.len() != num_readings {
            panic!(
                "Data has different lengths A:{} B:{}",
                src.len(),
                num_readings
            )
        }
    }

    // For each reading index, compute the average reading
    let mut avg_readings = Vec::new();

    for reading_index in 0..num_readings {
        let mut total_val = 0.0;

        for data_src in sources {
            total_val += data_src[reading_index]
        }

        avg_readings.push(total_val);
    }

    avg_readings
}

/// This function renders the control panel window
pub fn control_panel(state: &mut StateParam, ui: &Ui) {
    let ctrl_panel = &mut state.ui.control_panel;
    if !ctrl_panel.open {
        return;
    }

    Window::new("All ODrives Control Panel")
        .size([400f32, 800f32], imgui::Condition::Always)
        .opened(&mut ctrl_panel.open)
        .build(ui, || {
            // Display battery indicator
            battery_level(&ui, &state.backend.battery);

            // Display average plots
            ui.text("Average Value Plots");
            average_selectable_plots(
                &ui,
                &mut state.backend,
                &mut ctrl_panel.odrives.plottable_values,
            );

            // Display axis state/control mode/input mode widget for all odrives
            odrive_mode_widget(ui, &mut state.backend, &mut ctrl_panel.odrives)
        });
}

fn battery_level(ui: &Ui, battery_level: &f32) {
    // Display changing color battery indicator based on charge level
    ui.text_colored(
        [2f32 * (1f32 - battery_level), 2f32 * battery_level, 0f32, 1f32],
        format!("{}% Battery", 100.0 * battery_level),
    );
}

fn average_selectable_plots(
    ui: &Ui,
    backend_state: &BackendState,
    plot_state: &mut DisplayedPlots,
) {
    let odrives: Vec<&AxisID> = backend_state.odrive_data.keys().collect();

    let voltages: Vec<Vec<f32>> = odrives
        .iter()
        .map(|id| backend_state.get_prop_readings(id, |odrv| odrv.bus_voltage))
        .collect();
    let currents: Vec<Vec<f32>> = odrives
        .iter()
        .map(|id| backend_state.get_prop_readings(id, |odrv| odrv.bus_voltage))
        .collect();
    let motor_temps: Vec<Vec<f32>> = odrives
        .iter()
        .map(|id| backend_state.get_prop_readings(id, |odrv| odrv.bus_voltage))
        .collect();
    let inverter_temps: Vec<Vec<f32>> = odrives
        .iter()
        .map(|id| backend_state.get_prop_readings(id, |odrv| odrv.bus_voltage))
        .collect();

    plot_selectors(
        ui,
        plot_state,
        &[
            (BusVoltage, "Avg Voltage [V]", &average_readings(&voltages)),
            (
                BusCurrent,
                "Total Current [I]",
                &average_readings(&currents),
            ),
            (
                MotorTemp,
                "Avg. Motor Temperature °C",
                &average_readings(&motor_temps),
            ),
            (
                InverterTemp,
                "Avg. Inverter Temperature °C",
                &average_readings(&inverter_temps),
            ),
        ],
    );
}

fn odrive_mode_widget(ui: &Ui, backend_state: &mut BackendState, all_odrive_gui_state: &mut ODriveDetailState) {

    dropdown(ui, "ODrive State", &mut all_odrive_gui_state.axis_state);
    ui.separator();

    // If the control mode is switched, we need to reset the control mode value
    let before_mode = all_odrive_gui_state.control_mode.clone();
    dropdown(ui, "Control Mode", &mut all_odrive_gui_state.control_mode);
    if before_mode != all_odrive_gui_state.control_mode {
        all_odrive_gui_state.control_mode_val = 0.0;
    }

    // Display the appropriate slider ranges depending on the mode
    match all_odrive_gui_state.control_mode {
        ControlMode::VoltageControl => {
            Slider::new("Voltage", 11.0, 24.0).build(ui, &mut all_odrive_gui_state.control_mode_val)
        }
        ControlMode::TorqueControl => {
            Slider::new("Torque", 0.0, 0.22).build(ui, &mut all_odrive_gui_state.control_mode_val)
        }
        ControlMode::VelocityControl => {
            Slider::new("Velocity", 0.0, 50.0).build(ui, &mut all_odrive_gui_state.control_mode_val)
        }
        ControlMode::PositionControl => {
            InputFloat::new(ui, "Position", &mut all_odrive_gui_state.control_mode_val).build()
        }
    };
    ui.separator();

    dropdown(ui, "Input Mode", &mut all_odrive_gui_state.input_mode);
    ui.separator();

    // If the button is clicked, apply the changes in the UI state to the backend state
    if ui.button("Apply changes") {
        backend_state.set_all_states(&all_odrive_gui_state.axis_state);
        backend_state.set_control_mode(&all_odrive_gui_state.control_mode);
        backend_state.set_input_mode(&all_odrive_gui_state.input_mode);
        backend_state.set_control_val(&all_odrive_gui_state.control_mode_val);
    }
}
