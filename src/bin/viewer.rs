#![windows_subsystem = "windows"]

use ppd_editor::viewer;

fn main() {
    env_logger::init();

    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        viewer::APP_TITLE,
        native_options,
        Box::new(|cc| viewer::setup_eframe(cc, None)),
    );
}
