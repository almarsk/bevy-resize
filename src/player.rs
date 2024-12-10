use bevy::prelude::*;

use super::camera;
use super::level;
use super::mesh_utils;

pub const SPRITE_RADIUS: f32 = 50.0;
pub const Z_INDEX: f32 = 10.;  // Make sure the player mesh is rendered in front of all the other meshes
pub const ACCELERATION: f32 = 50.;
pub const SCALE_FACTOR: f32 = 1.1;
pub const MAX_VELOCITY: f32 = 2000.;
pub const MAX_SPIN: f32 = 60.;
pub const VELOCITY_DECAY: f32 = 0.99;
pub const SPIN_DECAY: f32 = 0.99;
pub const BOUNCE_VELOCITY_DAMPING: f32 = 0.7;
pub const BOUNCE_VELOCITY_TO_SPIN: f32 = 0.005;
pub const BOUNCE_SPIN_TO_DIRECTION: f32 = 50.;

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
    velocity: Vec3,
    spin: f32,
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
            velocity: Vec3::ZERO,
            spin: 0.,
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
        player.translation = Vec3::new(level.dimension.x, level.dimension.y, Z_INDEX) / 2.;
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
            player.velocity.x -= ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            player.velocity.x += ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            player.velocity.y += ACCELERATION;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            player.velocity.y -= ACCELERATION;
        }
    }
}

pub fn player_movement(mut player_query: Query<(&mut Transform, &mut Player)>, time: Res<Time>) {
    if let Ok((mut transform, mut player)) = player_query.get_single_mut() {
        let delta = player.velocity * time.delta_secs() / transform.scale.y;
        transform.translation += delta;
        if player.velocity.length() > MAX_VELOCITY {
            player.velocity = player.velocity.normalize() * MAX_VELOCITY;
        }
        player.velocity *= VELOCITY_DECAY;

        let rotation_delta = player.spin * time.delta_secs() / transform.scale.y;
        transform.rotate_z(rotation_delta);
        if player.spin.abs() > MAX_SPIN / transform.scale.y {
            player.spin = player.spin.signum() * MAX_SPIN / transform.scale.y;
        }
        player.spin *= SPIN_DECAY;
    }
}

pub fn player_rotation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Player>,
) {
    if let Ok(mut player) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::KeyA) {
            player.spin += 0.1;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            player.spin -= 0.1;
        }
    }
}

pub struct Collision {
    surface_normal: Vec3
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
            let mut collisions: Vec<Collision> = Vec::new();

            // Detect collisions and move the player out of the collision
            if translation.x < x_min {
                translation.x = x_min;
                collisions.push(Collision{surface_normal: Vec3::X});
            } else if translation.x > x_max {
                translation.x = x_max;
                collisions.push(Collision{surface_normal: -Vec3::X});
            }
            if translation.y < y_min {
                translation.y = y_min;
                collisions.push(Collision{surface_normal: Vec3::Y});
            } else if translation.y > y_max {
                translation.y = y_max;
                collisions.push(Collision{surface_normal: -Vec3::Y});
            }

            // Process detected collisions, update player speed and spin
            if !collisions.is_empty() {
                let player_velocity = player.velocity;
                let player_spin = player.spin;
                for collision in collisions {
                    player.spin -= player_velocity.cross(collision.surface_normal).z * BOUNCE_VELOCITY_TO_SPIN / player_transform.scale.y;
                    player.velocity -= 2.0 * player_velocity.dot(collision.surface_normal) * collision.surface_normal * BOUNCE_VELOCITY_DAMPING
                        + BOUNCE_SPIN_TO_DIRECTION * Vec3::new(player_spin * collision.surface_normal.y, -player_spin * collision.surface_normal.x, 0.);
                }
            }

            player_transform.translation = translation;
        }
    }
}
