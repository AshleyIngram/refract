pub mod book1;
pub mod demo;
pub mod scene_kind;
mod scene_preset;
pub mod textures;

pub use book1::Book1Scene;
pub use demo::DemoScene;
pub use scene_kind::SceneKind;
pub(crate) use scene_preset::ScenePreset;
pub use textures::TexturesScene;
