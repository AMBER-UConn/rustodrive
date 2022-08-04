use imgui::{Ui, Window};

use crate::window::AppState;

pub fn charts(state: &mut AppState, ui: &Ui) {
    

    if state.control_panel {
        Window::new("charts")
            .opened(&mut state.control_panel)
            .build(ui, || {
                ui.text("yo yo yo");
            });
    }
}
