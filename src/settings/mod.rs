pub mod options;

use crate::settings::options::OptionsDef;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[non_exhaustive]
pub struct SvgFileLoaderSettings {
    pub target_render_size: Option<TargetRenderSize>,
    pub options: OptionsDef,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct TargetRenderSize {
    pub width: u32,
    pub height: u32,
}
