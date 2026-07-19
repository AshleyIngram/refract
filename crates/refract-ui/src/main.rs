mod app;
mod render_job;

use app::RefractApp;
use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Refract")
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Refract",
        options,
        Box::new(|_cc| Ok(Box::new(RefractApp::new()))),
    )
}
