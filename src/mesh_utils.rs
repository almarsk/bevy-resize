use bevy::{
    math::VectorSpace, prelude::*, render::{mesh, render_asset}
};
use rand::prelude::*;
use std::f32::consts::{PI, TAU};

pub fn star_mesh(points: u16, radius: f32, inner_radius: f32) -> mesh::Mesh {
    let mut positions = Vec::with_capacity((points * 2 + 1) as usize);
    let mut indices = Vec::with_capacity((points * 6) as usize);
    positions.push(Vec3::ZERO);
    for i in 0..(points * 2) {
        let angle = i as f32 / points as f32 * PI;
        let r = if i % 2 == 0 { radius } else { inner_radius };
        positions.push(Vec3::new(r * angle.cos(), r * angle.sin(), 0.));
        indices.push(0);
        indices.push(i + 1);
        indices.push((i + 1) % (2 * points) + 1);
    }

    mesh::Mesh::new(
        mesh::PrimitiveTopology::TriangleList,
        render_asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(bevy::render::mesh::Indices::U16(indices))
}

pub fn triangle_vertex_positions(randomness: f32) -> [Vec3; 3] {
    let mut positions = [Vec3::ZERO; 3];
    let range = 0..3;
    for i in range {
        let angle = i as f32 * TAU / 3. - TAU / 12.;
        let r = 1. + randomness * random::<f32>();
        positions[i] = Vec3::new(r * angle.cos(), r * angle.sin(), 0.);
    }

    positions
}

pub fn rectangle_outline(width: f32, height: f32) -> mesh::Mesh {
    let positions = vec![
        Vec3::new(0., 0., 0.),
        Vec3::new(width, 0., 0.),
        Vec3::new(width, height, 0.),
        Vec3::new(0., height, 0.),
    ];
    let indices = vec![0, 1, 2, 3, 0];
    mesh::Mesh::new(
        mesh::PrimitiveTopology::LineStrip,
        render_asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_indices(bevy::render::mesh::Indices::U16(indices))
}

pub fn random_lines(n_lines: u16, min: Vec3, max: Vec3) -> mesh::Mesh {
    let mut positions = Vec::new();
    for _ in 0..n_lines {
        positions
            .push(min + (max - min) * Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()));
        positions
            .push(min + (max - min) * Vec3::new(random::<f32>(), random::<f32>(), random::<f32>()));
    }
    mesh::Mesh::new(
        mesh::PrimitiveTopology::LineList,
        render_asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
}
