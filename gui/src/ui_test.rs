
use crate::{support, views::{overview, detail, control_panel}, app_state::{UIState, AppState, StateParam}};

pub fn ui_main() {
    let system = support::init();
    let mut state = StateParam { ui: UIState::new(), app: AppState::new() };

    system.main_loop( move |_, ui| {
        ui.show_demo_window(&mut true);
        overview::odrive_overview(&mut state, ui);

        // Display the detail pages
        for (odrive_id, odrive_detail) in state.ui.details.iter_mut() {
            detail::detail(&mut state.app, odrive_id, odrive_detail, ui);
        }
        control_panel::control_panel(&mut state, ui);
    });
}
