use std::collections::{HashSet, HashMap};

use imgui::Ui;
use rustodrive::axis::{AxisID};
pub struct AppState {
    pub control_panel: bool,
    pub odrive_details: HashMap<AxisID, bool>,
}

impl AppState {
    pub fn new() -> Self {
        Self { control_panel: false, odrive_details: HashMap::new() }
    }

    pub fn toggle_ctrl_panel(&mut self) {
        self.control_panel = !self.control_panel;
    }

    pub fn detail_view(&mut self, odrive: &AxisID) {
        self.odrive_details.insert(*odrive, true);
    }
}




