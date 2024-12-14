#![feature(stmt_expr_attributes)]

mod camera;
mod level;
mod mesh_utils;
mod player;
mod ui;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(level::LevelPlugin)
        .add_plugins(camera::CameraPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(ui::UIPlugin)
        .add_systems(Update, exit_app_on_esc)
        .run();
}

fn exit_app_on_esc(keys: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}
