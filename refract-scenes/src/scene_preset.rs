use refract::camera::RenderSettings;
use refract::{material::ReflectionType, scene::Scene};

/// Scene geometry and default render settings. Override [`default_render_settings`]
/// only when a scene needs camera or image defaults that differ from
/// [`RenderSettings::default`].
pub(crate) trait ScenePreset {
    fn build(&self, reflection_type: ReflectionType) -> Scene;

    fn default_render_settings(&self) -> RenderSettings {
        RenderSettings::default()
    }
}
