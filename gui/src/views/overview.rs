use crate::{readings::ODriveReadings, app_state::{UIState, StateParam}};
use imgui::{Selectable, StyleColor, TableColumnSetup, TableFlags, Ui, Window};

fn odrive_row(app_state: &mut UIState, ui: &Ui, odrv: &ODriveReadings) {
    ui.table_set_column_index(0);

    let row_is_selected = Selectable::new(&odrv.id.to_string())
        .span_all_columns(true)
        .allow_double_click(true)
        .build(ui);
    if row_is_selected {
        app_state.detail_view(odrv);
    }

    ui.table_next_column();

    for col in &odrv.as_columns()[1..] {
        ui.text(col);
        ui.table_next_column();
    }

    ui.table_next_row();
}

pub fn odrive_overview(state: &mut StateParam, ui: &Ui) {

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
                for (id, odrv) in state.app.odrive_data.clone().iter() {
                    odrive_row(&mut state.ui, ui, odrv.front().unwrap());
                }
            }

            let button_color = ui.push_style_color(StyleColor::Button, [255f32, 0f32, 0f32, 1f32]);
            if ui.button("Charts") {
                state.ui.control_panel.toggle();
            }
            button_color.pop();
        });
}
