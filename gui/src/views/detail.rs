use std::collections::HashMap;

use imgui::{Ui, Window};
use rustodrive::{
    axis::AxisID,
    state::{AxisState, ControlMode, InputMode},
};

use crate::{
    readings::PlottableData::*,
    shared::{
        components::odrive_mode_component,
        state::{BackendState, ODriveDetailState},
        widgets::plot_selectors,
    },
};

impl Default for ODriveDetailState {
    fn default() -> Self {
        Self {
            axis_state: AxisState::Idle,
            control_mode: ControlMode::VelocityControl,
            input_mode: InputMode::Inactive,
            control_mode_val: 0.0,
            plottable_values: HashMap::from([
                (PosEstimate, false),
                (VelEstimate, false),
                (ShadowCount, false),
                (EncoderCount, false),
                (MotorTemp, false),
                (InverterTemp, false),
                (BusVoltage, true),
                (BusCurrent, true),
            ]),
        }
    }
}

pub struct ODriveDetail {
    pub open: bool,
    pub odrive: ODriveDetailState,
}

pub fn detail(
    backend_state: &mut BackendState,
    axis_id: &AxisID,
    odrive_detail: &mut ODriveDetail,
    ui: &Ui,
) {
    if !odrive_detail.open {
        return;
    }
    Window::new(format!("ODrive {}", axis_id))
        .size([400.0, 800.0], imgui::Condition::Always)
        .opened(&mut odrive_detail.open)
        .build(ui, || {
            // Display the plots of odrive detail data
            ui.text("Plots");
            detail_selectable_plots(ui, backend_state, axis_id, &mut odrive_detail.odrive);

            // Display axis state/control mode/input mode widget for all odrives
            let detail_odrive_state = &mut odrive_detail.odrive;
            odrive_mode_component(ui, detail_odrive_state);

            // If the button is clicked, apply the changes in the UI state to the backend state
            if ui.button("Apply changes") {
                backend_state.set_all_states(&detail_odrive_state.axis_state);
                backend_state.set_control_mode(&detail_odrive_state.control_mode);
                backend_state.set_input_mode(&detail_odrive_state.input_mode);
                backend_state.set_control_val(&detail_odrive_state.control_mode_val);
            }
        });
}

fn detail_selectable_plots(
    ui: &Ui,
    backend_state: &mut BackendState,
    axis_id: &AxisID,
    odrive_detail: &mut ODriveDetailState,
) {
    plot_selectors(
        ui,
        &mut odrive_detail.plottable_values,
        &[
            (
                BusVoltage,
                "Voltage [V]",
                &backend_state.get_prop_readings(axis_id, |odrv| odrv.bus_voltage),
            ),
            (
                BusCurrent,
                "Current [I]",
                &backend_state.get_prop_readings(axis_id, |odrv| odrv.bus_current),
            ),
            (
                PosEstimate,
                "Position Estimate",
                &backend_state.get_prop_readings(axis_id, |odrv| odrv.position_estimate),
            ),
            (
                VelEstimate,
                "Velocity Estimate",
                &backend_state.get_prop_readings(axis_id, |odrv| odrv.velocity_estimate),
            ),
            (
                ShadowCount,
                "Shadow Count",
                &backend_state.get_prop_readings(axis_id, |odrv| odrv.shadow_count as f32),
            ),
            (
                EncoderCount,
                "Encoder Count",
                &backend_state.get_prop_readings(axis_id, |odrv| odrv.encoder_count as f32),
            ),
            (
                MotorTemp,
                "Motor Temperature °C",
                &backend_state.get_prop_readings(axis_id, |odrv| odrv.motor_temp),
            ),
            (
                InverterTemp,
                "Inverter Temperature °C",
                &backend_state.get_prop_readings(axis_id, |odrv| odrv.inverter_temp),
            ),
        ],
    );
}
