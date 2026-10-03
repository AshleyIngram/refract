pub mod book1;
pub mod demo;
pub mod earth;
pub mod lighting;
pub mod perlin;
mod quads;
pub mod scene_kind;
mod scene_preset;

pub use book1::Book1Scene;
pub use demo::DemoScene;
pub use earth::EarthScene;
pub use lighting::LightingScene;
pub use perlin::PerlinNoiseScene;
pub use quads::QuadsScene;
pub use scene_kind::SceneKind;
pub(crate) use scene_preset::ScenePreset;
