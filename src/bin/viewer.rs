#![windows_subsystem = "windows"]

use clap::Parser;
use eframe::IconData;
use ppd_editor::viewer::{self, APP_ID};

#[derive(Parser)]
#[command(name = "ppd-viewer")]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    open: Option<String>,
}

fn main() {
    env_logger::init();

    let cli = Cli::parse();

    let native_options = eframe::NativeOptions {
        app_id: Some(APP_ID.to_owned()),
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

    let ppd = cli.open.and_then(|path| match paperdoll_tar::load(&path) {
        Ok(ppd) => Some(ppd),
        Err(err) => {
            log::warn!("Failed to load paperdoll file {}: {}", path, err);
            None
        }
    });

    if let Err(err) = eframe::run_native(
        viewer::APP_TITLE,
        native_options,
        Box::new(|cc| viewer::setup_eframe(cc, ppd)),
    ) {
        log::error!("Failed to run ppd-viewer: {}", err);
    }
}
