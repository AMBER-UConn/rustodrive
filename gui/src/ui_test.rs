
use std::f32::consts::PI;

use rustodrive::state::{AxisState, InputMode, ControlMode};

use crate::{support, views::{overview, detail, control_panel}, app_state::{UIState, AppState, StateParam}, readings::ODriveReadings};

fn mock_data(time: &f32, app_state: &mut AppState) {
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
            bus_current: id as f32 * 10.0 * f32::sin(33.0 + time / (PI))
        })
    }
}

pub fn ui_main() {
    let system = support::init();
    let mut state = StateParam { ui: UIState::new(), app: AppState::new(&[0, 1, 2, 3, 4, 5]) };
    mock_data(&0.0, &mut state.app);

    system.main_loop( move |_, ui| {
        ui.show_demo_window(&mut true);
        overview::odrive_overview(&mut state, ui);

        // Display the detail pages
        for (odrive_id, odrive_detail) in state.ui.details.iter_mut() {
            detail::detail(&mut state.app, odrive_id, odrive_detail, ui);
        }
        control_panel::control_panel(&mut state, ui);

        mock_data(&(ui.time() as f32), &mut state.app);
    });
}
