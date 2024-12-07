mod camera;
mod mesh_utils;

use bevy::prelude::*;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 100.0;
pub const SCALE_FACTOR: f32 = 1.1;
pub const LEVEL_DIM: Vec2 = Vec2::new(1920., 1080.);
pub const VIEWPORT_DIM: Vec3 = Vec3::new(1280., 720., 0.);
pub const TOP_LEFT: Vec3 = Vec3::new(0., VIEWPORT_DIM.y, 0.);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(camera::CameraPlugin)
        .add_systems(
            Startup,
            (spawn_level, spawn_player, spawn_coordinate_display),
        )
        .add_systems(Update, (player_size, confine_player_size).chain())
        .add_systems(Update, player_acceleration)
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

#[derive(Component)]
pub struct Level {}

pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = mesh_utils::rectangle_outline(LEVEL_DIM.x, LEVEL_DIM.y);

    commands.spawn((
        Mesh2d(meshes.add(mesh).into()),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgba(
            1., 0.8, 0.0, 1.,
        )))),
        Level {},
        camera::CameraBounds {
            min: Vec2::splat(0.),
            max: LEVEL_DIM,
        },
    ));

    // Beautiful background
    commands.spawn((
        Mesh2d(
            meshes
                .add(mesh_utils::random_lines(
                    100,
                    Vec3::splat(0.),
                    Vec3::new(LEVEL_DIM.x, LEVEL_DIM.y, 0.),
                ))
                .into(),
        ),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgba(
            0., 0.3, 0.5, 1.,
        )))),
        Transform::from_xyz(0., 0., -1.),
    ));
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = mesh_utils::star_mesh(7, PLAYER_SIZE / 2., PLAYER_SIZE / 3.);

    commands.spawn((
        Mesh2d(meshes.add(mesh).into()),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgba(
            1., 0.8, 0.0, 1.,
        )))),
        Transform::from_translation(Vec3::new(LEVEL_DIM.x, LEVEL_DIM.y, 0.) / 2.),
        Player {
            speed: Vec3::new(0., 0., 0.),
            rotation_speed: 0.,
        },
        camera::CameraFocus {},
    ));
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

pub fn confine_player_movement(mut player_query: Query<(&mut Transform, &mut Player)>) {
    if let Ok((mut player_transform, mut player)) = player_query.get_single_mut() {
        let half_player_size = PLAYER_SIZE * player_transform.scale.y / 2.0;
        let x_min = 0.0 + half_player_size;
        let x_max = LEVEL_DIM.x - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = LEVEL_DIM.y - half_player_size;

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

pub fn spawn_coordinate_display(mut commands: Commands) {
    commands.spawn((
        Text2d::new("Player XY: [-, -]"),
        TextLayout::new_with_justify(JustifyText::Left),
        Transform::from_translation(TOP_LEFT + Vec3::new(10., -10., 0.)),
        bevy::sprite::Anchor::TopLeft,
        CoordinateDisplay {},
    ));
}

pub fn update_coordinate_display(
    player_query: Query<&Transform, With<Player>>,
    mut text_query: Query<(&mut Text2d, &CoordinateDisplay)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        if let Ok((mut text, _)) = text_query.get_single_mut() {
            text.0 = format!(
                "Player XY: [{:.0}, {:.0}]",
                player_transform.translation.x, player_transform.translation.y
            );
        }
    }
}
