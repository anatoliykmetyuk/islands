use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier3d::prelude::*;

mod terrain;
pub use terrain::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EditorPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(basic_scene)
        .run();
}

fn basic_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Make 3d camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 1.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    // Make red cube at coordinates (0, 1, 0)
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        ..Default::default()
    })
    .insert(RigidBody::Dynamic)
    .insert(Collider::cuboid(0.5, 0.5, 0.5));

    // Make point light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            color: Color::rgb(1.0, 1.0, 1.0),
            intensity: 1000.0,
            range: 100.0,
            ..Default::default()
        },
        ..Default::default()
    });
}
