
use crate::{support, views::{overview, detail, charts}, app_state::{UIState, AppState, StateParam}};

pub fn ui_main() {
    let system = support::init();
    let mut state = StateParam { ui: UIState::new(), app: AppState::new() };

    system.main_loop( move |_, ui| {
        ui.show_demo_window(&mut true);
        overview::odrive_overview(&mut state, ui);
        detail::detail(&mut state, ui);
        charts::control_panel(&mut state, ui);
    });
}
