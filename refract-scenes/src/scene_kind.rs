use refract::{material::ReflectionType, scene::Scene};

use crate::{Book1Scene, DemoScene, TexturesScene};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SceneKind {
    #[default]
    Demo,
    Book1,
    Textures,
}

impl SceneKind {
    pub const ALL: [Self; 3] = [Self::Demo, Self::Book1, Self::Textures];

    pub fn label(self) -> &'static str {
        match self {
            Self::Demo => "Demo",
            Self::Book1 => "Book 1",
            Self::Textures => "Textures"
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        if value.eq_ignore_ascii_case("demo") {
            Some(Self::Demo)
        } else if value.eq_ignore_ascii_case("book1") || value.eq_ignore_ascii_case("book-1") {
            Some(Self::Book1)
        } else if value.eq_ignore_ascii_case("textures") {
            Some(Self::Textures)
        } else {
            None
        }
    }

    pub fn build(self, reflection_type: ReflectionType) -> Scene {
        match self {
            Self::Demo => DemoScene::build(reflection_type),
            Self::Book1 => Book1Scene::build(reflection_type),
            Self::Textures => TexturesScene::build(reflection_type),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use refract::material::ReflectionType;

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
    fn parse_textures() {
        assert_eq!(SceneKind::parse("textures"), Some(SceneKind::Textures));
        assert_eq!(SceneKind::parse("Textures"), Some(SceneKind::Textures));
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
    fn build_returns_scene() {
        let _scene = SceneKind::Demo.build(ReflectionType::Lambertian);
    }
}
