use super::camera;
use super::mesh_utils;

use bevy::prelude::*;

pub const LEVEL_DIM: Vec2 = Vec2::new(1920., 1080.);

#[derive(Component)]
pub struct Level {
    pub dimension: Vec2,
}

#[derive(Component)]
pub struct Obstacle {}

pub struct LevelPlugin;
impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_level, spawn_obstacles));
    }
}

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
        Level {
            dimension: LEVEL_DIM,
        },
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

pub fn spawn_obstacles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let positions = vec![
        Vec3::new(0.25, 0.25, 0.),
        Vec3::new(0.75, 0.25, 0.),
        Vec3::new(0.25, 0.75, 0.),
        Vec3::new(0.75, 0.75, 0.),
    ];

    for pos in positions {
        let mesh = mesh_utils::triangle_mesh(1.);
        commands.spawn((
            Mesh2d(meshes.add(mesh).into()),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::linear_rgba(
                0., 0.8, 0.4, 1.,
            )))),
            Transform::from_translation(Vec3::new(LEVEL_DIM.x, LEVEL_DIM.y, 0.) * pos)
                .with_scale(Vec3::new(100., 100., 0.)),
            Obstacle {},
        ));
    }
}
