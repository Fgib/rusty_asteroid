use bevy::prelude::*;

/// Creates a triangular mesh for the player ship
pub fn create_player_triangle_mesh() -> Mesh {
    let mut triangle_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    // Define triangle vertices (elongated, pointing up)
    let vertices = vec![
        [0.0, 15.0, 0.0],    // Top point
        [-10.0, -15.0, 0.0], // Bottom left
        [10.0, -15.0, 0.0],  // Bottom right
    ];

    let indices = vec![0, 1, 2];
    let normals = vec![[0.0, 0.0, 1.0]; 3];
    let uvs = vec![[0.5, 1.0], [0.0, 0.0], [1.0, 0.0]];

    triangle_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    triangle_mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    triangle_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    triangle_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    triangle_mesh
}

/// Creates an arrow-shaped mesh for bullets
pub fn create_bullet_arrow_mesh() -> Mesh {
    let mut arrow_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    // Define arrow vertices (small arrow pointing up)
    let vertices = vec![
        [0.0, 4.0, 0.0],   // Arrow tip
        [-2.0, -2.0, 0.0], // Left wing
        [2.0, -2.0, 0.0],  // Right wing
        [0.0, -4.0, 0.0],  // Arrow tail
    ];

    let indices = vec![
        0, 1, 2, // Arrow head triangle
        1, 3, 2, // Arrow body triangle
    ];

    let normals = vec![[0.0, 0.0, 1.0]; 4];
    let uvs = vec![[0.5, 1.0], [0.0, 0.5], [1.0, 0.5], [0.5, 0.0]];

    arrow_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    arrow_mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    arrow_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    arrow_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    arrow_mesh
}

/// Creates a polygon mesh for asteroids with the specified number of faces (outline only)
pub fn create_asteroid_mesh(faces: u32, radius: f32) -> Mesh {
    let mut asteroid_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::LineList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    // Ensure minimum number of faces for a valid polygon
    let faces = faces.max(3);

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    // Create vertices around the circle with some randomness for irregular shape
    for i in 0..faces {
        let angle = (i as f32 / faces as f32) * 2.0 * std::f32::consts::PI;

        // Add some randomness to the radius for irregular asteroid shape
        // Constrain the random factor to prevent degenerate triangles
        let random_factor = 0.8 + fastrand::f32() * 0.4; // 0.8 to 1.2 multiplier (more conservative)
        let vertex_radius = radius * random_factor;

        let x = angle.cos() * vertex_radius;
        let y = angle.sin() * vertex_radius;

        vertices.push([x, y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);

        // Fixed UV coordinates that don't depend on randomized vertex positions
        // Use the original angle-based position for UV mapping
        let base_x = angle.cos() * radius;
        let base_y = angle.sin() * radius;
        let u = (base_x / radius + 1.0) * 0.5;
        let v = (base_y / radius + 1.0) * 0.5;
        uvs.push([u.clamp(0.0, 1.0), v.clamp(0.0, 1.0)]);
    }

    // Create line segments connecting adjacent vertices to form the outline
    for i in 0..faces {
        let next_i = (i + 1) % faces;
        indices.push(i as u32); // current vertex
        indices.push(next_i as u32); // next vertex
    }

    asteroid_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    asteroid_mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    asteroid_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    asteroid_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    asteroid_mesh
}

/// Creates a pixel art heart mesh based on a 16x16 pixel grid
pub fn create_heart_mesh() -> Mesh {
    let mut heart_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::TriangleList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    // Heart pattern based on the provided image (16x16 grid)
    // 1 = filled pixel, 0 = empty pixel
    #[rustfmt::skip]
    let heart_pattern = [
        [0,0,1,1,0,0,0,0,1,1,0,0,0,0,0,0],
        [0,1,1,1,1,0,0,1,1,1,1,0,0,0,0,0],
        [1,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0],
        [1,1,0,1,1,1,1,1,1,0,1,1,1,0,0,0],
        [1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0],
        [1,1,1,1,1,1,1,1,1,1,1,1,1,0,0,0],
        [0,1,1,1,1,1,1,1,1,1,1,1,0,0,0,0],
        [0,0,1,1,1,1,1,1,1,1,1,0,0,0,0,0],
        [0,0,0,1,1,1,1,1,1,1,0,0,0,0,0,0],
        [0,0,0,0,1,1,1,1,1,0,0,0,0,0,0,0],
        [0,0,0,0,0,1,1,1,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
        [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],
    ];

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    let pixel_size = 1.0; // Size of each pixel in world units
    let heart_width = 16.0 * pixel_size;
    let heart_height = 16.0 * pixel_size;

    // Create vertices for each filled pixel
    let mut vertex_index = 0u32;
    for (row, line) in heart_pattern.iter().enumerate() {
        for (col, &pixel) in line.iter().enumerate() {
            if pixel == 1 {
                // Calculate pixel position (center the heart)
                let x = col as f32 * pixel_size - heart_width * 0.5;
                let y = (15 - row) as f32 * pixel_size - heart_height * 0.5; // Flip Y to match image orientation

                // Create a quad for this pixel (2 triangles)
                let x1 = x;
                let y1 = y;
                let x2 = x + pixel_size;
                let y2 = y + pixel_size;

                // Add 4 vertices for the quad
                vertices.push([x1, y1, 0.0]); // Bottom-left
                vertices.push([x2, y1, 0.0]); // Bottom-right
                vertices.push([x2, y2, 0.0]); // Top-right
                vertices.push([x1, y2, 0.0]); // Top-left

                // Add normals
                for _ in 0..4 {
                    normals.push([0.0, 0.0, 1.0]);
                }

                // Add UVs
                uvs.push([0.0, 0.0]);
                uvs.push([1.0, 0.0]);
                uvs.push([1.0, 1.0]);
                uvs.push([0.0, 1.0]);

                // Add indices for 2 triangles
                indices.push(vertex_index); // Triangle 1
                indices.push(vertex_index + 1);
                indices.push(vertex_index + 2);

                indices.push(vertex_index); // Triangle 2
                indices.push(vertex_index + 2);
                indices.push(vertex_index + 3);

                vertex_index += 4;
            }
        }
    }

    heart_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    heart_mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    heart_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    heart_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    heart_mesh
}

/// Creates an outlined rectangular button mesh
pub fn create_button_outline_mesh(width: f32, height: f32) -> Mesh {
    let mut button_mesh = Mesh::new(
        bevy::render::render_resource::PrimitiveTopology::LineList,
        bevy::render::render_asset::RenderAssetUsages::RENDER_WORLD,
    );

    let half_width = width * 0.5;
    let half_height = height * 0.5;

    // Define the four corners of the rectangle
    let vertices = vec![
        [-half_width, -half_height, 0.0], // Bottom-left
        [half_width, -half_height, 0.0],  // Bottom-right
        [half_width, half_height, 0.0],   // Top-right
        [-half_width, half_height, 0.0],  // Top-left
    ];

    // Create line segments for the outline
    let indices = vec![
        0, 1, // Bottom edge
        1, 2, // Right edge
        2, 3, // Top edge
        3, 0, // Left edge
    ];

    let normals = vec![[0.0, 0.0, 1.0]; 4];
    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

    button_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    button_mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    button_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    button_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    button_mesh
}
