use std::time::Duration;

use eframe::egui::{
    self, Color32, ColorImage, ProgressBar, TextureHandle, TextureOptions,
};

use crate::render_job::{RenderConfig, RenderJob};

const REPAINT_INTERVAL: Duration = Duration::from_millis(33);

pub struct RefractApp {
    job: RenderJob,
    texture: Option<TextureHandle>,
    texture_is_final: bool,
}

impl RefractApp {
    pub fn new() -> Self {
        Self {
            job: RenderJob::spawn(RenderConfig::default()),
            texture: None,
            texture_is_final: false,
        }
    }

    fn refresh_texture(&mut self, ctx: &egui::Context) {
        if self.texture_is_final {
            return;
        }

        let buffer = self.job.buffer();
        let size = [buffer.width() as usize, buffer.height() as usize];
        let image = ColorImage::from_rgba_unmultiplied(size, &buffer.snapshot_rgba());

        match &mut self.texture {
            Some(texture) => texture.set(image, TextureOptions::NEAREST),
            None => {
                self.texture = Some(ctx.load_texture("render", image, TextureOptions::NEAREST));
            }
        }

        self.texture_is_final = self.job.is_complete();
    }

    fn show_status_bar(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let status = if self.job.is_complete() {
                "Done"
            } else {
                "Rendering..."
            };
            ui.label(status);

            ui.label(format_elapsed(self.job.elapsed()));

            let buffer = self.job.buffer();
            ui.label(format!("{}x{}", buffer.width(), buffer.height()));

            ui.add(
                ProgressBar::new(self.job.progress())
                    .show_percentage()
                    .animate(!self.job.is_complete()),
            );
        });
    }

    fn show_render_view(&self, ui: &mut egui::Ui) {
        let Some(texture) = &self.texture else {
            return;
        };

        ui.centered_and_justified(|ui| {
            ui.add(
                egui::Image::from_texture(texture)
                    .fit_to_exact_size(ui.available_size())
                    .maintain_aspect_ratio(true),
            );
        });
    }
}

impl eframe::App for RefractApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.refresh_texture(ui.ctx());

        egui::Panel::bottom("status_bar")
            .frame(egui::Frame::side_top_panel(ui.style()).inner_margin(8.0))
            .show(ui, |ui| self.show_status_bar(ui));

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Color32::from_gray(10)))
            .show(ui, |ui| self.show_render_view(ui));

        if !self.texture_is_final {
            ui.ctx().request_repaint_after(REPAINT_INTERVAL);
        }
    }
}

fn format_elapsed(elapsed: Duration) -> String {
    let total_seconds = elapsed.as_secs();
    format!("{}:{:02}", total_seconds / 60, total_seconds % 60)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_elapsed_under_a_minute_shows_zero_minutes() {
        let elapsed = Duration::from_secs(42);

        let formatted = format_elapsed(elapsed);

        assert_eq!(formatted, "0:42");
    }

    #[test]
    fn format_elapsed_over_a_minute_pads_seconds() {
        let elapsed = Duration::from_secs(65);

        let formatted = format_elapsed(elapsed);

        assert_eq!(formatted, "1:05");
    }
}
