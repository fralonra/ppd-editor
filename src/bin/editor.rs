#![windows_subsystem = "windows"]

use ppd_editor::editor;

fn main() {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        editor::APP_TITLE,
        native_options,
        Box::new(editor::setup_eframe),
    );
}
