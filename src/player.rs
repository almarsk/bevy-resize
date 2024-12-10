use bevy::prelude::*;

use super::camera;
use super::level;
use super::mesh_utils;

pub const ACCELERATION: f32 = 50.;
pub const SPRITE_RADIUS: f32 = 50.0;
pub const SCALE_FACTOR: f32 = 1.1;
pub const SPEED_DECAY: f32 = 0.99;
pub const ROTATION_DECAY: f32 = 0.99;
pub const BOUNCE_SPEED_DAMPING: f32 = 0.7;
pub const BOUNCE_SPEED_TO_ROTATION: f32 = 0.01;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(PostStartup, init_position_player);
        app.add_systems(Update, (player_size, confine_player_size).chain());
        app.add_systems(Update, player_acceleration);
        app.add_systems(Update, player_rotation);
        app.add_systems(Update, (player_movement, confine_player_movement).chain());
    }
}

#[derive(Component)]
pub struct Player {
    speed: Vec3,
    rotation_speed: f32,
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = mesh_utils::star_mesh(9, SPRITE_RADIUS, 0.66 * SPRITE_RADIUS);

    commands.spawn((
        Mesh2d(meshes.add(mesh).into()),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgba(
            1., 0.8, 0.0, 1.,
        )))),
        Player {
            speed: Vec3::new(0., 0., 0.),
            rotation_speed: 0.,
        },
        camera::CameraFocus {},
    ));
}

pub fn init_position_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    level_query: Query<&level::Level>,
) {
    let mut player = player_query.single_mut();
    if let Ok(level) = level_query.get_single() {
        player.translation = Vec3::new(level.dimension.x, level.dimension.y, 0.) / 2.;
    }
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

pub fn player_acceleration(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            player.speed.x -= ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            player.speed.x += ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player.speed.y += ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            player.speed.y -= ACCELERATION;
        }
    }
}

pub fn player_movement(mut player_query: Query<(&mut Transform, &mut Player)>, time: Res<Time>) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let delta = player.speed * time.delta_secs() / transform.scale.y;
        transform.translation += delta;
        player.speed *= SPEED_DECAY;

        let rotation_delta = player.rotation_speed * time.delta_secs() / transform.scale.y;
        transform.rotate_z(rotation_delta);
        player.rotation_speed *= ROTATION_DECAY;
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
    level_query: Query<&level::Level>,
) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        if let Ok(level) = level_query.get_single() {
            let scaled_player_radius = SPRITE_RADIUS * player_transform.scale.y;
            let x_min = scaled_player_radius;
            let x_max = level.dimension.x - scaled_player_radius;
            let y_min = scaled_player_radius;
            let y_max = level.dimension.y - scaled_player_radius;

            let mut translation = player_transform.translation;

            if translation.x < x_min {
                translation.x = x_min;
                let tangent = Vec3::Y;
                player.rotation_speed += tangent.dot(player.speed) * BOUNCE_SPEED_TO_ROTATION;
                player.speed.x *= -1.;
                player.speed *= BOUNCE_SPEED_DAMPING;
            } else if translation.x > x_max {
                translation.x = x_max;
                let tangent = -Vec3::Y;
                player.rotation_speed += tangent.dot(player.speed) * BOUNCE_SPEED_TO_ROTATION;
                player.speed.x *= -1.;
                player.speed *= BOUNCE_SPEED_DAMPING;
            }
            if translation.y < y_min {
                translation.y = y_min;
                let tangent = -Vec3::X;
                player.rotation_speed += tangent.dot(player.speed) * BOUNCE_SPEED_TO_ROTATION;
                player.speed.y *= -1.;
                player.speed *= BOUNCE_SPEED_DAMPING;
            } else if translation.y > y_max {
                translation.y = y_max;
                let tangent = Vec3::X;
                player.rotation_speed += tangent.dot(player.speed) * BOUNCE_SPEED_TO_ROTATION;
                player.speed.y *= -1.;
                player.speed *= BOUNCE_SPEED_DAMPING;
            }

            player_transform.translation = translation;
        }
    }
}
