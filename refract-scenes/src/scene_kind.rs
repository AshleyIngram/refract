use std::str::FromStr;

use refract::camera::RenderSettings;
use refract::{material::ReflectionType, scene::Scene};
use strum::{Display, EnumIter, EnumString, IntoEnumIterator};

use crate::{Book1Scene, DemoScene, EarthScene, ScenePreset};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter, Display, EnumString)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
pub enum SceneKind {
    #[default]
    #[strum(to_string = "Demo")]
    Demo,
    #[strum(serialize = "book1", serialize = "book-1", to_string = "Book 1")]
    Book1,
    #[strum(to_string = "Earth")]
    Earth,
}

impl SceneKind {
    pub fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }

    pub fn parse(value: &str) -> Option<Self> {
        Self::from_str(value).ok()
    }

    pub fn build(self, reflection_type: ReflectionType) -> Scene {
        match self {
            Self::Demo => DemoScene.build(reflection_type),
            Self::Book1 => Book1Scene.build(reflection_type),
            Self::Earth => EarthScene.build(reflection_type),
        }
    }

    pub fn default_render_settings(self) -> RenderSettings {
        match self {
            Self::Demo => DemoScene.default_render_settings(),
            Self::Book1 => Book1Scene.default_render_settings(),
            Self::Earth => EarthScene.default_render_settings(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use refract::material::ReflectionType;
    use strum::IntoEnumIterator;

    #[test]
    fn iter_includes_every_variant() {
        assert_eq!(SceneKind::iter().count(), 3);
    }

    #[test]
    fn parse_demo() {
        assert_eq!(SceneKind::parse("demo"), Some(SceneKind::Demo));
        assert_eq!(SceneKind::parse("Demo"), Some(SceneKind::Demo));
    }

    #[test]
    fn parse_book1() {
        assert_eq!(SceneKind::parse("book1"), Some(SceneKind::Book1));
        assert_eq!(SceneKind::parse("book-1"), Some(SceneKind::Book1));
    }

    #[test]
    fn parse_earth() {
        assert_eq!(SceneKind::parse("earth"), Some(SceneKind::Earth));
        assert_eq!(SceneKind::parse("Earth"), Some(SceneKind::Earth));
    }

    #[test]
    fn parse_unknown_none() {
        assert_eq!(SceneKind::parse("book3"), None);
        assert_eq!(SceneKind::parse("unknown"), None);
    }

    #[test]
    fn default_is_demo() {
        assert_eq!(SceneKind::default(), SceneKind::Demo);
    }

    #[test]
    fn display_labels() {
        assert_eq!(SceneKind::Demo.to_string(), "Demo");
        assert_eq!(SceneKind::Book1.to_string(), "Book 1");
        assert_eq!(SceneKind::Earth.to_string(), "Earth");
    }

    #[test]
    fn build_returns_scene() {
        let _scene = SceneKind::Demo.build(ReflectionType::Lambertian);
    }

    #[test]
    fn default_render_settings_returns_values() {
        let settings = SceneKind::Earth.default_render_settings();

        assert_eq!(settings.defocus_angle, 0.0);
    }
}
