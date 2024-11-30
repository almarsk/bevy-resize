use bevy::prelude::*;
use bevy::sprite::Mesh2dHandle;
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
        .add_systems(Update, player_size)
        .add_systems(Update, player_input)
        .add_systems(Update, player_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, confine_player_size)
        .add_systems(Update, update_coordinate_display)
        .run();
}

#[derive(Component)]
pub struct Player {
    speed: Vec3,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colors: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        ColorMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle {
                radius: PLAYER_SIZE / 2.0,
            })),
            transform: Transform {
                translation: Vec3 {
                    x: window.width() / 2.0,
                    y: window.height() / 2.0,
                    z: 0.0,
                },
                ..Default::default()
            },
            material: colors.add(ColorMaterial {
                color: Color::linear_rgb(120.0, 56.0, 0.0),
                texture: None,
            }),
            ..Default::default()
        },
        Player {
            speed: Vec3::new(0., 0., 0.),
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
        if keyboard_input.pressed(KeyCode::KeyJ) {
            transform.scale.x *= SCALE_FACTOR;
            transform.scale.y *= SCALE_FACTOR;
        }
        if keyboard_input.pressed(KeyCode::KeyK) {
            transform.scale.x /= SCALE_FACTOR;
            transform.scale.y /= SCALE_FACTOR;
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
        let delta = player.speed * PLAYER_SPEED * time.delta_seconds();

        transform.translation += delta;
        player.speed *= Vec3::splat(0.97);
    }
}

pub fn confine_player_size(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let lesser_dim = window.height().min(window.width()) * 0.8 / PLAYER_SIZE;

        if player_transform.scale.y > lesser_dim {
            player_transform.scale.y = lesser_dim;
            player_transform.scale.x = lesser_dim;
        }

        if player_transform.scale.y < 0.5 {
            player_transform.scale.y = 0.5;
            player_transform.scale.x = 0.5;
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
