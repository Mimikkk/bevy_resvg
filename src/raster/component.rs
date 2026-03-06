use crate::raster::asset::SvgFile;
use bevy::prelude::*;

/// The [`Component`] that one needs to wrap [`SvgFile`]s in before
/// spawning them.
#[derive(Component, Default)]
pub struct Svg(pub Handle<SvgFile>);

/// The [`Component`] that one needs to wrap [`SvgFile`]s in before
/// using them in Bevy UI's.
#[derive(Component, Default)]
pub struct UiSvg(pub Handle<SvgFile>);
