mod camera;
mod level;
mod player;
mod mesh_utils;
mod ui;

use bevy::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(level::LevelPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(ui::UIPlugin)
        .run();
}
