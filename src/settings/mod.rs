use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub struct SvgFileLoaderSettings {
    pub target_render_size: Option<TargetRenderSize>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TargetRenderSize {
    pub width: u32,
    pub height: u32,
}
