// Parts taken from 生于斯's fork of this repository:
// https://github.com/shengyusi-SYS/bevy_svg_ui
use bevy::prelude::*;

/// A color tint to apply to an SVG when rendering.
/// This will be multiplied with the SVG's original colors.
#[derive(Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct SvgColor(pub Color);
