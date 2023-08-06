#![windows_subsystem = "windows"]

use eframe::IconData;
use ppd_editor::viewer;

fn main() {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        centered: true,
        icon_data: match IconData::try_from_png_bytes(include_bytes!(
            "../../build/logo/ppd-viewer.png"
        )) {
            Ok(icon) => Some(icon),
            Err(err) => {
                log::warn!("Failed to load window icon: {}", err);
                None
            }
        },
        initial_window_size: Some(eframe::epaint::vec2(1200.0, 700.0)),
        ..Default::default()
    };

    eframe::run_native(
        viewer::APP_TITLE,
        native_options,
        Box::new(|cc| viewer::setup_eframe(cc, None)),
    );
}
