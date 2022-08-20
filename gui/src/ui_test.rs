use std::f32::consts::PI;

use rustodrive::state::{AxisState, ControlMode, InputMode};

use crate::{
    app_state::{BackendState, StateParam, UIState},
    readings::ODriveReadings,
    support,
    views::{control_panel, detail, overview},
};

/// This adds a fake odrive reading for a given time step
fn mock_data(time: &f32, app_state: &mut BackendState) {
    for id in 0..6 {
        app_state.add_reading(ODriveReadings {
            id: id,
            current_state: AxisState::Idle,
            input_mode: InputMode::PosFilter,
            control_mode: ControlMode::PositionControl,
            position_estimate: id as f32 * 200.0 * f32::sin(time / (2.0 * PI)),
            velocity_estimate: id as f32 * 50.0 * f32::sin(33.0 + time / (2.0 * PI)),
            shadow_count: (id as f32 * 1000.0 * f32::sin(33.0 + time / (PI))) as i32,
            encoder_count: (id as f32 * 1000.0 * f32::sin(33.0 + time / (PI))) as i32,
            motor_temp: id as f32 * 38.0,
            inverter_temp: id as f32 * 38.0,
            bus_voltage: id as f32 * 24.0 * f32::sin(33.0 + time / (PI)),
            bus_current: id as f32 * 10.0 * f32::sin(33.0 + time / (PI)),
        })
    }
}

/// This is the entrypoint of the user interface application
pub fn ui_main() {
    let imgui = support::init();
    let mut state = StateParam {
        ui: UIState::new(),
        app: BackendState::new(&[0, 1, 2, 3, 4, 5]),
    };

    imgui.main_loop(move |_, ui| {
        // Retrieve the data from the odrives here
        mock_data(&(ui.time() as f32), &mut state.app);

        // Uncomment this to see a demo of features available in imgui
        // ui.show_demo_window(&mut true);

        // Display the table
        overview::odrive_overview(&mut state, ui);

        // Display the detail windows
        for (odrive_id, odrive_detail) in state.ui.details.iter_mut() {
            detail::detail(&mut state.app, odrive_id, odrive_detail, ui);
        }

        // Display the all-odrives control panel
        control_panel::control_panel(&mut state, ui);
    });
}
