use crate::{readings::ODriveReadings, window::AppState};
use imgui::{Selectable, StyleColor, TableColumnSetup, TableFlags, Ui, Window};

fn odrive_row(app_state: &mut AppState, ui: &Ui, odrv: &ODriveReadings) {
    ui.table_set_column_index(0);

    let row_is_selected = Selectable::new(&odrv.can_id.to_string())
        .span_all_columns(true)
        .allow_double_click(true)
        .build(ui);
    if row_is_selected {
        app_state.detail_view(&odrv.can_id);
    }

    ui.table_next_column();

    for col in &odrv.as_columns()[1..] {
        ui.text(col);
        ui.table_next_column();
    }

    ui.table_next_row();
}

pub fn odrive_overview(state: &mut AppState, ui: &Ui) {
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
                for odrv in state.odrive_data.clone().iter() {
                    odrive_row(state, ui, odrv);
                }
            }

            let button_color = ui.push_style_color(StyleColor::Button, [255f32, 0f32, 0f32, 1f32]);
            if ui.button("Charts") {
                state.toggle_ctrl_panel();
            }
            button_color.pop();
        });
}
