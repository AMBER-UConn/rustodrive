
use rustodrive::threads::ReadWriteCANThread;
use rustodrive_gui::support;

pub fn ui_main() {
    let system = support::init();
    let mut value = 0;
    let choices = ["test test this is 1", "test test this is 2"];
    system.main_loop(move |_, ui| {
        ui.text_wrapped("Hello world!");
        // ui.text_wrapped("こんにちは世界！");
        if ui.button(choices[value]) {
            value += 1;
            value %= 2;
        }

        ui.button("This...is...imgui-rs!");
        ui.separator();
        let mouse_pos = ui.io().mouse_pos;
        ui.text(format!(
            "Mouse Position: ({:.1},{:.1})",
            mouse_pos[0], mouse_pos[1]
        ));
    });
}
