
use crate::{support, views::{overview, detail, charts}, window::AppState};

pub fn ui_main() {
    let system = support::init();
    let mut app_state = AppState::new();

    system.main_loop( move |_, ui| {
        ui.show_demo_window(&mut true);
        overview::odrive_overview(&mut app_state, ui);
        detail::detail(&mut app_state, ui);
        charts::charts(&mut app_state, ui);
    });
}
