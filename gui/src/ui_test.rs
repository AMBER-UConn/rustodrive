
use imgui::{ChildWindow, Window};
use rustodrive::state::ODriveAxisState;
// use rustodrive::threads::ReadWriteCANThread;
use crate::{support, views::{self, overview::ODriveReadings}};

pub fn ui_main() {
    let system = support::init();

    system.main_loop(move |_, ui| {
        Window::new("ODrive Overview").build(ui, || {
            views::overview::odrive_overview(ui, vec![ODriveReadings {
                can_id: 1,
                current_state: ODriveAxisState::Idle,
                position_estimate: 3141f32,
                velocity_estimate: 21f32,
                shadow_count: 23414,
                encoder_count: 123,
                motor_temp: 31f32,
                inverter_temp: 30f32,
                bus_voltage: 22.31,
                bus_current: 3.12,
            }; 3]);
            ui.show_demo_window(&mut true);
            ui.text_wrapped("Hello world!");
            // ui.text_wrapped("こんにちは世界！");
            
            ui.button("This...is...imgui-rs!");
            ui.separator();
            let mouse_pos = ui.io().mouse_pos;
            ui.text(format!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos[0], mouse_pos[1]
            ));
        });
        
    });
}
