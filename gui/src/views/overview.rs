use imgui::{TableColumnSetup, TableFlags, Ui, Window, TableSortDirection, Selectable, ColorButton, WindowFlags, ChildWindow, StyleColor, SelectableFlags, };
use rustodrive::state::ODriveAxisState;

use crate::window::AppState;

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


fn odrive_row(app_state: &mut AppState, ui: &Ui, odrv: &ODriveReadings) {
    let columns = [
        // odrv.can_id.to_string(),
        odrv.current_state.to_string(),
        odrv.position_estimate.to_string(),
        odrv.velocity_estimate.to_string(),
        odrv.shadow_count.to_string(),
        odrv.encoder_count.to_string(),
        odrv.motor_temp.to_string(),
        odrv.inverter_temp.to_string(),
        odrv.bus_voltage.to_string(),
        odrv.bus_current.to_string(),
    ];

    ui.table_set_column_index(0);
    
    let row_is_selected = Selectable::new(&odrv.can_id.to_string()).span_all_columns(true).allow_double_click(true).build(ui);
    if row_is_selected {
        app_state.detail_view(&odrv.can_id);
    }
    ui.table_next_column();

    for col in columns {
        ui.text(col);
        ui.table_next_column();
    }
    
    ui.table_next_row();
}

pub fn odrive_overview(app_state: &mut AppState, ui: &Ui, odrives: Vec<ODriveReadings>) {
    Window::new("Live Stats")
        .size([0f32, 200f32], imgui::Condition::FirstUseEver)
        .build(ui, || {
            if let Some(_t) = ui.begin_table_header_with_flags(
                "stats",
                [
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
                TableFlags::ROW_BG | TableFlags::BORDERS_OUTER | TableFlags::NO_BORDERS_IN_BODY,
            ) {
                ui.table_next_row();
                for odrv in odrives.iter() {
                    odrive_row(app_state, ui, odrv);
                }
            }

            for (odrive_id, window_open) in app_state.odrive_details.iter_mut() {
                if *window_open {
                    Window::new(format!("ODrive {}", odrive_id)).opened(window_open).build(ui, || {
                        ui.text("this is a specific odrive view")
                    });
                }
                
            }

            let button_color = ui.push_style_color(StyleColor::Button, [255f32, 0f32, 0f32, 1f32]);
            if ui.button("Charts") {
                app_state.toggle_ctrl_panel();
            }
            button_color.pop();

            if app_state.control_panel {
                Window::new("charts").opened(&mut app_state.control_panel).build(ui, || {
                    ui.text("yo yo yo");
                });
            }
        });
}
