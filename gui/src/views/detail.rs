use imgui::{Window, Ui};

use crate::window::AppState;

pub fn detail(state: &mut AppState, ui: &Ui) {
    for (odrive_id, window_open) in state.odrive_details.iter_mut() {
        if *window_open {
            Window::new(format!("ODrive {}", odrive_id))
                .opened(window_open)
                .build(ui, || ui.text("this is a specific odrive view"));
        }
    }
}
