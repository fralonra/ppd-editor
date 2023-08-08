#![windows_subsystem = "windows"]

use eframe::IconData;
use ppd_editor::editor::{self, APP_ID};

fn main() {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        app_id: Some(APP_ID.to_owned()),
        centered: true,
        icon_data: match IconData::try_from_png_bytes(include_bytes!(
            "../../build/logo/ppd-editor.png"
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
        editor::APP_TITLE,
        native_options,
        Box::new(editor::setup_eframe),
    );
}
