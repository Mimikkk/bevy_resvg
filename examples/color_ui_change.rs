use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
};
use bevy_resvg::prelude::*;

#[derive(Resource, Default)]
struct UseGreen(bool);

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, SvgPlugin))
        .init_resource::<UseGreen>()
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_svg_color_on_space)
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
        children![(UiSvg(svg), SvgColor(Color::Srgba(RED)))],
    ));
}

fn toggle_svg_color_on_space(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut use_green: ResMut<UseGreen>,
    mut svg_colors: Query<&mut SvgColor, With<UiSvg>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    use_green.0 = !use_green.0;
    let next_color = if use_green.0 {
        Color::Srgba(GREEN)
    } else {
        Color::Srgba(RED)
    };

    for mut svg_color in &mut svg_colors {
        svg_color.0 = next_color;
    }
}
