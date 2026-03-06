use crate::raster::{
    asset::{SvgFile, loader::SvgFileLoader},
    component::{Svg, UiSvg},
};
use bevy::prelude::*;
use std::collections::HashSet;

/// The [`Plugin`] for initialising the
/// [Rasterised](https://en.wikipedia.org/wiki/Raster_graphics)
/// [`Asset`] and [`AssetLoader`](bevy::asset::AssetLoader).
pub struct SvgRasterPlugin;

impl Plugin for SvgRasterPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<SvgFile>()
            .init_asset_loader::<SvgFileLoader>()
            .add_systems(
                Update,
                (
                    handle_svg_loaded,
                    handle_svg_modified,
                    handle_svg_removed,
                    handle_ui_svg_loaded,
                    handle_ui_svg_modified,
                    handle_ui_svg_removed,
                ),
            );
    }
}

macro_rules! read_events {
    ($svg_events:expr, $($asset_event:path) | +) => {{
        let mut ids = None;
        for event in $svg_events.read() {
            match event {
                $($asset_event { id }) | + => {
                    ids.get_or_insert_with(HashSet::new).insert(*id);
                }
                _ => (),
            }
        }
        let Some(ids) = ids else {
            return;
        };
        ids
    }};
}

fn new_image_from_svg(
    id: AssetId<SvgFile>,
    svg_assets: &Assets<SvgFile>,
    images: &mut Assets<Image>,
) -> Option<Handle<Image>> {
    svg_assets.get(id).map_or_else(
        || {
            warn!("`{id}` reported in events, but not found in `Assets`");
            None
        },
        |svg_file| Some(images.add(svg_file.0.clone())),
    )
}

fn sync_existing_image_from_svg(
    id: AssetId<SvgFile>,
    svg_assets: &Assets<SvgFile>,
    images: &mut Assets<Image>,
    image_handle: &mut Handle<Image>,
) {
    if let Some(svg_file) = svg_assets.get(id) {
        if let Some(image) = images.get_mut(image_handle.id()) {
            *image = svg_file.0.clone();
            debug!("Updated `Image` data for modified `{id}`");
        } else {
            *image_handle = images.add(svg_file.0.clone());
            debug!("Replaced `Handle<Image>` for modified `{id}`");
        }
    } else {
        warn!("`{id}` reported as modified, but not found in `Assets`");
    }
}

/// Handles newly loaded [`SvgFile`]s by adding [`Sprite`] components to waiting
/// entities. This responds to [`AssetEvent::LoadedWithDependencies`].
fn handle_svg_loaded(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    svg_assets: Res<Assets<SvgFile>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, &Svg), Without<Sprite>>,
) {
    let loaded_ids = read_events!(svg_events, AssetEvent::LoadedWithDependencies);
    if loaded_ids.is_empty() {
        return;
    }

    for (entity, svg) in &query {
        let id = svg.0.id();
        if loaded_ids.contains(&id)
            && let Some(image_handle) = new_image_from_svg(id, &svg_assets, &mut images)
        {
            commands
                .entity(entity)
                .insert(Sprite::from_image(image_handle));
            debug!("Added `Sprite` for `{id}` to entity {entity:?}");
        }
    }
}

/// Handles modified [`SvgFile`]s (e.g. through
/// [hot-reloading](https://github.com/bevyengine/bevy/blob/main/examples/asset/hot_asset_reloading.rs))
/// by updating existing [`Sprite`]s. This responds to [`AssetEvent::Modified`].
fn handle_svg_modified(
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    svg_assets: Res<Assets<SvgFile>>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&Svg, &mut Sprite)>,
) {
    let modified_ids = read_events!(svg_events, AssetEvent::Modified);
    if modified_ids.is_empty() {
        return;
    }

    for (svg, mut sprite) in &mut query {
        let id = svg.0.id();
        if modified_ids.contains(&id) {
            sync_existing_image_from_svg(id, &svg_assets, &mut images, &mut sprite.image);
        }
    }
}

/// Handles removed and unused [`SvgFile`]s by cleaning up associated [`Sprite`]
/// components. This fires when an [`Asset`] is either explicitly removed from
/// the asset system, or removed due to the last strong handle being dropped.
/// This corresponds to [`AssetEvent::Removed`] and [`AssetEvent::Unused`],
/// respectively.
fn handle_svg_removed(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    query: Query<(Entity, &Svg), With<Sprite>>,
) {
    let removed_ids = read_events!(svg_events, AssetEvent::Removed | AssetEvent::Unused);

    if removed_ids.is_empty() {
        return;
    }

    for (entity, svg) in query {
        let id = svg.0.id();
        if removed_ids.contains(&id) {
            commands.entity(entity).remove::<Sprite>();
            info!("Removed `Sprite` for `{id}` from entity {entity:?}");
        }
    }
}

/// Handles newly loaded [`SvgFile`]s by adding [`ImageNode`] components to
/// waiting entities in UI. This responds to
/// [`AssetEvent::LoadedWithDependencies`].
fn handle_ui_svg_loaded(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    svg_assets: Res<Assets<SvgFile>>,
    mut images: ResMut<Assets<Image>>,
    query: Query<(Entity, &UiSvg), Without<ImageNode>>,
) {
    let loaded_ids = read_events!(svg_events, AssetEvent::LoadedWithDependencies);

    if loaded_ids.is_empty() {
        return;
    }

    for (entity, svg) in &query {
        let id = svg.0.id();
        if loaded_ids.contains(&id)
            && let Some(image_handle) = new_image_from_svg(id, &svg_assets, &mut images)
        {
            commands.entity(entity).insert(ImageNode::new(image_handle));
            debug!("Added `ImageNode` for `{id}` to entity {entity:?}");
        }
    }
}

/// Handles modified [`SvgFile`]s by updating existing [`ImageNode`]s.
/// This responds to [`AssetEvent::Modified`].
fn handle_ui_svg_modified(
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    svg_assets: Res<Assets<SvgFile>>,
    mut images: ResMut<Assets<Image>>,
    mut query: Query<(&UiSvg, &mut ImageNode)>,
) {
    let modified_ids = read_events!(svg_events, AssetEvent::Modified);

    if modified_ids.is_empty() {
        return;
    }

    for (svg, mut image_node) in &mut query {
        let id = svg.0.id();
        if modified_ids.contains(&id) {
            sync_existing_image_from_svg(id, &svg_assets, &mut images, &mut image_node.image);
        }
    }
}

/// Handles removed and unused [`SvgFile`]s by cleaning up associated
/// [`ImageNode`] components in UI. This corresponds to
/// [`AssetEvent::Removed`] and [`AssetEvent::Unused`], respectively.
fn handle_ui_svg_removed(
    mut commands: Commands,
    mut svg_events: MessageReader<AssetEvent<SvgFile>>,
    query: Query<(Entity, &UiSvg), With<ImageNode>>,
) {
    let removed_ids = read_events!(svg_events, AssetEvent::Removed | AssetEvent::Unused);

    if removed_ids.is_empty() {
        return;
    }

    for (entity, svg) in query {
        let id = svg.0.id();
        if removed_ids.contains(&id) {
            commands.entity(entity).remove::<ImageNode>();
            info!("Removed `ImageNode` for `{id}` from entity {entity:?}");
        }
    }
}
