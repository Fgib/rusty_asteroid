use bevy::prelude::*;
use std::f32::consts::PI;

pub fn create_star_mesh(points: usize, outer_radius: f32, inner_radius: f32) -> Mesh {
    let mut vertices: Vec<[f32; 3]> = Vec::new();

    vertices.push([0.0, 0.0, 0.0]);

    for i in 0..(points * 2) {
        let angle = (i as f32 * 2.0 * PI) / (points * 2) as f32;
        let radius = if i % 2 == 0 {
            outer_radius
        } else {
            inner_radius
        };
        vertices.push([radius * angle.cos(), radius * angle.sin(), 0.0]);
    }

    let mut indices = Vec::new();

    for i in 0..(points * 2) {
        let next = if i == points * 2 - 1 { 1 } else { i + 2 };

        indices.push(0u32);
        indices.push((i + 1) as u32);
        indices.push(next as u32);
    }

    Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
}

pub fn create_diamond_mesh(size: f32) -> Mesh {
    let vertices = vec![
        [0.0, size, 0.0],  // Top
        [size, 0.0, 0.0],  // Right
        [0.0, -size, 0.0], // Bottom
        [-size, 0.0, 0.0], // Left
    ];

    let indices = vec![0u32, 1, 2, 0, 2, 3];

    Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
}

pub fn create_triangle_mesh(size: f32) -> Mesh {
    Mesh::from(Triangle2d::new(
        Vec2::new(0.0, size),
        Vec2::new(-size * 0.866, -size * 0.5),
        Vec2::new(size * 0.866, -size * 0.5),
    ))
}

pub fn create_hexagon_mesh(size: f32) -> Mesh {
    Mesh::from(RegularPolygon::new(size, 6))
}

pub fn create_cross_mesh(size: f32) -> Mesh {
    let half_size = size * 0.5;
    let thickness = size * 0.2;

    let vertices = vec![
        // Horizontal bar
        [-half_size, thickness, 0.0],
        [half_size, thickness, 0.0],
        [half_size, -thickness, 0.0],
        [-half_size, -thickness, 0.0],
        // Vertical bar
        [-thickness, half_size, 0.0],
        [thickness, half_size, 0.0],
        [thickness, -half_size, 0.0],
        [-thickness, -half_size, 0.0],
    ];

    let indices = vec![
        // Horizontal bar
        0u32, 1, 2, 0, 2, 3, // Vertical bar
        4, 5, 6, 4, 6, 7,
    ];

    Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
}
