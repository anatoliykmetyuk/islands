use noise::{NoiseFn, Perlin};
use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

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
    // let terrain = generate_terrain(100, 100);
    // Create mesh as a triangle set from the height map
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    // Insert attribute with dummy vertice positions
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vec![
        // Set of vertices describing a single with coordinates: (0, 0, 0), (1, 0, 0), (0, 1, 0)
        // This is the first triangle. Use Vec3.
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ]);

    // Make colors for each vertex. Use Vec3.
    let colors = vec![
        Vec4::new(1.0, 0.0, 0.0, 1.0),
        Vec4::new(0.0, 1.0, 0.0, 1.0),
        Vec4::new(0.0, 0.0, 1.0, 1.0),
    ];
    // Insert attribute with colors
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        // material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    });
}

fn generate_terrain(width: usize, height: usize) -> Terrain {
    let perlin = Perlin::new(1337);
    let mut height_map = vec![0.0; width * height];
    let mut color_map = vec![Color::WHITE; width * height];
    for y in 0..height {
        for x in 0..width {
            let height_val = perlin.get([(x as f64) / 100.0, (y as f64) / 100.0]) as f32;
            height_map[y * width + x] = height_val;
            color_map[y * width + x] = Color::rgb(height_val, height_val, height_val);
        }
    }

    Terrain {
        height_map,
        color_map,
    }
}
