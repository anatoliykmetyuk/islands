use noise::{NoiseFn, Perlin};
use bevy::{prelude::*, render::{render_resource::PrimitiveTopology, mesh::Indices}};

const MAX_HEIGHT: f32 = 10.0;
const TILE_SIZE: usize = 100;
const PERLIN_SCALE: f64 = 50.0;

pub struct Terrain {
    pub height_map: Vec<f32>,
    pub color_map: Vec<Color>,
}

pub struct TerrainPlugin;
impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_terrain);
    }
}

fn setup_terrain(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let heightmap = generate_heightmap(TILE_SIZE, TILE_SIZE, PERLIN_SCALE);
    height_map_to_image(heightmap.clone());
    // Create mesh as a triangle set from the height map
    let mesh = heightmap_to_mesh(heightmap);

    let offset = Vec3::new(-(TILE_SIZE as f32) / 2.0, 0.0, -(TILE_SIZE as f32) / 2.0);
    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 1.0, 0.43),
        metallic: 0.5,
        ..Default::default()
    });
    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        transform: Transform::from_translation(offset),
        material,
        ..Default::default()
    });
}

fn generate_heightmap(width: usize, height: usize, scale: f64) -> Vec<f32> {
    let perlin = Perlin::new(1337);
    let mut height_map = vec![0.0; width * height];
    for y in 0..height {
        for x in 0..width {
            let height_val = perlin.get([(x as f64) / scale, (y as f64) / scale]) as f32;
            height_map[y * width + x] = height_val;
        }
    }
    height_map
}

fn heightmap_to_mesh(height_map: Vec<f32>) -> Mesh {
    let size = (height_map.len() as f32).sqrt() as usize;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    let mut normals = Vec::new();

    // Generate vertices, and normals
    for y in 0..size {
        for x in 0..size {
            let index = y * size + x;
            let height = height_map[index];

            // Calculate position
            let position = Vec3::new(
                x as f32,
                height * MAX_HEIGHT,
                y as f32,
            );
            vertices.push(position);

            // Calculate normal
            let mut normal = Vec3::new(0.0, 0.0, 0.0);
            if x > 0 && x < size - 1 && y > 0 && y < size - 1 {
                let left = height_map[index - 1];
                let right = height_map[index + 1];
                let top = height_map[index - size];
                let bottom = height_map[index + size];

                normal = Vec3::new(left - right, 2.0, top - bottom).normalize();
            }
            normals.push(normal);
        }
    }

    // Generate indices
    for y in 0..size - 1 {
        for x in 0..size - 1 {
            let tl = y * size + x;
            let tr = y * size + (x + 1);
            let bl = (y + 1) * size + x;
            let br = (y + 1) * size + (x + 1);

            indices.push(tl as u32);
            indices.push(bl as u32);
            indices.push(tr as u32);

            indices.push(tr as u32);
            indices.push(bl as u32);
            indices.push(br as u32);
        }
    }

    // Create mesh based on the vertices and indices
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh
}

use image::GrayImage;

fn height_map_to_image(height_map: Vec<f32>) {
    let size = (height_map.len() as f32).sqrt() as u32;
    let mut image = GrayImage::new(size, size);

    for y in 0..size {
        for x in 0..size {
            let index = (y * size + x) as usize;
            let color_intensity = ((height_map[index] + 1.0) / 2.0 * 255.0) as u8;
            image.put_pixel(x, y, image::Luma([color_intensity]));
        }
    }

    image.save("height_map.png").unwrap();
}
