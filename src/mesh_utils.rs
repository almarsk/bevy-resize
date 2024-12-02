use std::f32::consts::PI;

use bevy::{
    prelude::*, 
    render::{mesh, render_asset},
};

pub fn star_mesh (points: u16, radius: f32, inner_radius: f32) -> mesh::Mesh {
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

    mesh::Mesh::new(mesh::PrimitiveTopology::TriangleList, render_asset::RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_indices(bevy::render::mesh::Indices::U16(indices))
}

pub fn rectangle_outline (width: f32, height: f32) -> mesh::Mesh {
    let positions = vec![
        Vec3::new(0., 0., 0.),
        Vec3::new(width, 0., 0.),
        Vec3::new(width, height, 0.),
        Vec3::new(0., height, 0.),
    ];
    let indices = vec![0, 1, 2, 3, 0];
    mesh::Mesh::new(mesh::PrimitiveTopology::LineStrip, render_asset::RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_indices(bevy::render::mesh::Indices::U16(indices))
}
