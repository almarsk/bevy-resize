use bevy::prelude::*;

use super::mesh_utils;
use super::camera;
use super::level;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 100.0;
pub const SCALE_FACTOR: f32 = 1.1;


pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
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
	level_query: Query<&level::Level>,
) {
	if let Ok(level) = level_query.get_single() {
		let mesh = mesh_utils::star_mesh(7, PLAYER_SIZE / 2., PLAYER_SIZE / 3.);

		commands.spawn((
			Mesh2d(meshes.add(mesh).into()),
			MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgba(
				1., 0.8, 0.0, 1.,
			)))),
			Transform::from_translation(Vec3::new(level.dimension.x, level.dimension.y, 0.) / 2.),
			Player {
				speed: Vec3::new(0., 0., 0.),
				rotation_speed: 0.,
			},
			camera::CameraFocus {},
		));
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
        let delta = player.speed * PLAYER_SPEED * time.delta_secs() / transform.scale.y;
        transform.translation += delta;
        player.speed *= Vec3::splat(0.97);

        let rotation_delta = player.rotation_speed * time.delta_secs() / transform.scale.y;
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
	level_query: Query<&level::Level>,
) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
		if let Ok(level) = level_query.get_single() {
			let half_player_size = PLAYER_SIZE * player_transform.scale.y / 2.0;
			let x_min = 0.0 + half_player_size;
			let x_max = level.dimension.x - half_player_size;
			let y_min = 0.0 + half_player_size;
			let y_max = level.dimension.y - half_player_size;

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
}

