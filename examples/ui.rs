use bevy::{color::palettes::css::BLUE, prelude::*};
use bevy_resvg::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let svg: Handle<SvgFile> = asset_server.load("transparent.svg");
    commands.spawn(Camera2d);
    commands.spawn((
        Node {
            width: px(128),
            height: px(128),
            border: UiRect::all(px(8)),
            ..default()
        },
        BorderColor::all(Color::Srgba(BLUE)),
        children![UiSvg(svg)],
    ));
}
