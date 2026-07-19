use eframe::egui::{self, Button, ComboBox, DragValue, Grid, Ui};
use refract::material::ReflectionType;
use refract::point::Point;

use crate::render_job::{RenderConfig, derived_height};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsAction {
    None,
    StartRender,
    CancelRender,
}

pub fn show(ui: &mut Ui, config: &mut RenderConfig, is_rendering: bool) -> SettingsAction {
    ui.heading("Settings");
    ui.add_space(8.0);

    section(ui, "Image", |ui| image_settings(ui, config));
    section(ui, "Camera", |ui| camera_settings(ui, config));
    section(ui, "Materials", |ui| material_settings(ui, config));

    ui.add_space(12.0);
    action_buttons(ui, is_rendering)
}

fn section(ui: &mut Ui, title: &str, add_contents: impl FnOnce(&mut Ui)) {
    egui::CollapsingHeader::new(title)
        .default_open(true)
        .show(ui, add_contents);
    ui.add_space(4.0);
}

fn image_settings(ui: &mut Ui, config: &mut RenderConfig) {
    Grid::new("image_settings")
        .num_columns(2)
        .spacing([12.0, 6.0])
        .show(ui, |ui| {
            ui.label("Width");
            ui.add(
                DragValue::new(&mut config.width)
                    .range(100..=3840)
                    .suffix(" px"),
            );
            ui.end_row();

            ui.label("Resolution");
            ui.label(format!("{} x {}", config.width, derived_height(config.width)));
            ui.end_row();

            ui.label("Samples per pixel");
            ui.add(DragValue::new(&mut config.samples_per_pixel).range(1..=5000));
            ui.end_row();

            ui.label("Max bounces");
            ui.add(DragValue::new(&mut config.max_depth).range(1..=200));
            ui.end_row();
        });
}

fn camera_settings(ui: &mut Ui, config: &mut RenderConfig) {
    Grid::new("camera_settings")
        .num_columns(2)
        .spacing([12.0, 6.0])
        .show(ui, |ui| {
            ui.label("Look from");
            point_input(ui, &mut config.look_from);
            ui.end_row();

            ui.label("Look at");
            point_input(ui, &mut config.look_at);
            ui.end_row();

            ui.label("Vertical FOV");
            ui.add(
                DragValue::new(&mut config.field_of_view)
                    .range(1.0..=120.0)
                    .speed(0.5)
                    .suffix("°"),
            );
            ui.end_row();

            ui.label("Defocus angle");
            ui.add(
                DragValue::new(&mut config.defocus_angle)
                    .range(0.0..=10.0)
                    .speed(0.05)
                    .suffix("°"),
            );
            ui.end_row();

            ui.label("Focus distance");
            ui.add(
                DragValue::new(&mut config.focus_distance)
                    .range(0.1..=100.0)
                    .speed(0.1),
            );
            ui.end_row();
        });
}

fn point_input(ui: &mut Ui, point: &mut Point) {
    ui.horizontal(|ui| {
        ui.add(DragValue::new(&mut point.x).speed(0.1).prefix("x "));
        ui.add(DragValue::new(&mut point.y).speed(0.1).prefix("y "));
        ui.add(DragValue::new(&mut point.z).speed(0.1).prefix("z "));
    });
}

fn material_settings(ui: &mut Ui, config: &mut RenderConfig) {
    ComboBox::from_label("Diffuse model")
        .selected_text(reflection_type_label(config.reflection_type))
        .show_ui(ui, |ui| {
            for reflection_type in [ReflectionType::Lambertian, ReflectionType::Diffuse] {
                ui.selectable_value(
                    &mut config.reflection_type,
                    reflection_type,
                    reflection_type_label(reflection_type),
                );
            }
        });
}

fn reflection_type_label(reflection_type: ReflectionType) -> &'static str {
    match reflection_type {
        ReflectionType::Diffuse => "Diffuse",
        ReflectionType::Lambertian => "Lambertian",
    }
}

fn action_buttons(ui: &mut Ui, is_rendering: bool) -> SettingsAction {
    let render_label = if is_rendering { "Restart render" } else { "Render" };
    let full_width = [ui.available_width(), 28.0];

    if ui.add_sized(full_width, Button::new(render_label)).clicked() {
        return SettingsAction::StartRender;
    }

    if is_rendering && ui.add_sized(full_width, Button::new("Cancel")).clicked() {
        return SettingsAction::CancelRender;
    }

    SettingsAction::None
}
