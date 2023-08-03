#![windows_subsystem = "windows"]

use ppd_editor::editor;

fn main() {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        centered: true,
        initial_window_size: Some(eframe::epaint::vec2(1200.0, 700.0)),
        ..Default::default()
    };

    eframe::run_native(
        editor::APP_TITLE,
        native_options,
        Box::new(editor::setup_eframe),
    );
}
