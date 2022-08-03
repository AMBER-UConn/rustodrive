use imgui::{Ui, Window, TableColumnFlags, TableColumnSetup, TableFlags};
use rustodrive::{self, state::ODriveAxisState};

#[derive(Clone)]
pub struct ODriveReadings {
    pub can_id: usize,
    pub current_state: ODriveAxisState,
    // controller_status: ControllerStatus,
    // axis_error: AxisError
    // motor_error: MotorError,
    pub position_estimate: f32,
    pub velocity_estimate: f32,
    pub shadow_count: i32,
    pub encoder_count: i32,
    pub motor_temp: f32,
    pub inverter_temp: f32,
    pub bus_voltage: f32,
    pub bus_current: f32,
}

fn odrive_row(ui: &Ui, odrv: &ODriveReadings) {
    let columns = [
        odrv.can_id.to_string(),
        odrv.current_state.to_string(),
        odrv.position_estimate.to_string(),
        odrv.velocity_estimate.to_string(),
        odrv.shadow_count.to_string(),
        odrv.encoder_count.to_string(),
        odrv.motor_temp.to_string(),
        odrv.inverter_temp.to_string(),
        odrv.bus_voltage.to_string(),
        odrv.bus_current.to_string()
    ];

    ui.table_next_row();
    ui.table_set_column_index(0);
    for col in columns {
        ui.text(col);
        ui.table_next_column();
    }
    ui.table_next_row();
}

pub fn odrive_overview(ui: &Ui, odrives: Vec<ODriveReadings>) {
    Window::new("Live Stats").size([0f32, 200f32], imgui::Condition::FirstUseEver).build(ui, || {
        if let Some(_t) = ui.begin_table_header_with_flags("stats", [
            TableColumnSetup::new("ID"),
            TableColumnSetup::new("State"),
            TableColumnSetup::new("Position Est. [???]"),
            TableColumnSetup::new("Velocity Est. [R/s]"),
            TableColumnSetup::new("Shadow Count"),
            TableColumnSetup::new("Encoder Count"),
            TableColumnSetup::new("Motor Temp [°C]"),
            TableColumnSetup::new("Inverter Temp [°C]"),
            TableColumnSetup::new("Bus [V]"),
            TableColumnSetup::new("Current [I]"),
        ],
        TableFlags::SORTABLE,
    ) {
            for odrv in odrives.iter() {
                odrive_row(ui, odrv);
            }
        }
    });
}