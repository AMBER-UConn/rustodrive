use imgui::{Window, Ui};

use crate::app_state::{StateParam};

pub fn detail(state: &mut StateParam, ui: &Ui) {
    for (odrive_id, window_open) in state.ui.odrive_details.iter_mut() {
        if *window_open {
            Window::new(format!("ODrive {}", odrive_id))
                .opened(window_open)
                .build(ui, || ui.text("this is a specific odrive view"));
        }
    }
}
