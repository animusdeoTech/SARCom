#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod data;
mod map;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("SARCOM Kiosk Lab")
            .with_inner_size([1060.0, 520.0])
            .with_min_inner_size([800.0, 480.0]),
        ..Default::default()
    };
    eframe::run_native(
        "SARCOM Kiosk Lab",
        options,
        Box::new(|_cc| Ok(Box::new(app::KioskLabApp::new()))),
    )
}
