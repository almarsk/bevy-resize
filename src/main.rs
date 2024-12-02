use std::f32::consts::{PI};

use bevy::prelude::*;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::mesh::PrimitiveTopology;
use bevy::window::PrimaryWindow;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 100.0;
pub const SCALE_FACTOR: f32 = 1.1;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_camera, spawn_player, spawn_coordinate_display),
        )
        .add_systems(Update, (player_size, confine_player_size).chain())
        .add_systems(Update, player_input)
        .add_systems(Update, player_rotation)
        .add_systems(Update, (player_movement, confine_player_movement).chain())
        .add_systems(Update, update_coordinate_display)
        .run();
}

#[derive(Component)]
pub struct Player {
    speed: Vec3,
    rotation_speed: f32,
}

pub fn star_mesh (points: u16, radius: f32, inner_radius: f32) -> Mesh {
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    positions.push(Vec3::splat(0.));
    for i in 0..(points * 2) {
        let angle = i as f32 / points as f32 * PI;
        let r = if i % 2 == 0 { radius } else { inner_radius };
        positions.push(Vec3::new(r * angle.cos(), r * angle.sin(), 0.));
        indices.push(0);
        indices.push(i + 1);
        indices.push((i + 1) % (2 * points) + 1);
    }

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_indices(bevy::render::mesh::Indices::U16(indices))
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();

    let mesh = star_mesh (7, PLAYER_SIZE / 2., PLAYER_SIZE / 3.);

    commands.spawn((
        ColorMesh2dBundle {
            mesh: meshes.add(mesh).into(),
            transform: Transform {
                translation: Vec3 {
                    x: window.width() / 2.0,
                    y: window.height() / 2.0,
                    z: 0.0,
                },
                ..Default::default()
            },
            material: colors.add(ColorMaterial {
                color: Color::linear_rgba(120.0, 56.0, 0.0, 0.8),
                texture: None,
            }),
            ..Default::default()
        },
        Player {
            speed: Vec3::new(0., 0., 0.),
            rotation_speed: 0.,
        },
    ));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

pub fn player_size(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) {
            transform.scale.x *= SCALE_FACTOR;
            transform.scale.y *= SCALE_FACTOR;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            transform.scale.x /= SCALE_FACTOR;
            transform.scale.y /= SCALE_FACTOR;
        }
    }
}

pub fn confine_player_size(mut player_query: Query<&mut Transform, With<Player>>) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let max_dim = 2.;

        if player_transform.scale.y > max_dim {
            player_transform.scale.y = max_dim;
            player_transform.scale.x = max_dim;
        }

        if player_transform.scale.y < 0.5 {
            player_transform.scale.y = 0.5;
            player_transform.scale.x = 0.5;
        }
    }
}

pub fn player_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            player.speed += Vec3::new(-0.1, 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            player.speed += Vec3::new(0.1, 0.0, 0.);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player.speed += Vec3::new(0., 0.1, 0.);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            player.speed += Vec3::new(0., -0.1, 0.);
        }
    }
}

pub fn player_movement(mut player_query: Query<(&mut Transform, &mut Player)>, time: Res<Time>) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let delta = player.speed * PLAYER_SPEED * time.delta_seconds() / transform.scale.y;
        transform.translation += delta;
        player.speed *= Vec3::splat(0.97);

        let rotation_delta = player.rotation_speed * time.delta_seconds() / transform.scale.y;
        transform.rotate_z(rotation_delta);
        player.rotation_speed *= 0.97;
    }
}

pub fn player_rotation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::KeyA) {
            player.rotation_speed += 0.1;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player.rotation_speed -= 0.1;
        }
    }
}

pub fn confine_player_movement(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = PLAYER_SIZE * player_transform.scale.y / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.x < x_min {
            player.speed.x *= -1.;
            translation.x = x_min;
        } else if translation.x > x_max {
            player.speed.x *= -1.;
            translation.x = x_max;
        }
        if translation.y < y_min {
            player.speed.y *= -1.;
            translation.y = y_min;
        } else if translation.y > y_max {
            player.speed.y *= -1.;
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

#[derive(Component)]
pub struct CoordinateDisplay;

pub fn spawn_coordinate_display(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let txt = Text {
        sections: vec![TextSection {
            value: "Coordinates: (0, 0)".to_string(),
            style: TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..Default::default()
            },
        }],
        ..Default::default()
    };

    commands
        .spawn(Text2dBundle {
            text: txt,
            transform: Transform::from_xyz(window.width() * 0.2, window.height() * 0.9, 0.0), // Position the text
            ..default()
        })
        .insert(CoordinateDisplay);
}

pub fn update_coordinate_display(
    player_query: Query<&Transform, With<Player>>,
    mut text_query: Query<(&mut Text, &CoordinateDisplay)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok((mut text, _)) = text_query.get_single_mut() {
            text.sections[0].value = format!(
                "Coordinates: ({:.2}, {:.2})",
                player_transform.translation.x, player_transform.translation.y
            );
        }
    }
}
