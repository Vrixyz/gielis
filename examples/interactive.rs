use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use gielis::{Gielis, xy_from_phi};

pub fn generate_leaf_mesh(gielis: &Gielis, resolution: u32) -> (Vec<Vec3>, Vec<[u32; 3]>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let mut leaf_vertices = Vec::new();

    leaf_vertices.push(Vec3::new(0.0, 0.0, 0.0));
    for i in 0..resolution {
        let phi = (i as f32 / resolution as f32) * std::f32::consts::TAU;
        let rphi = gielis.rphi(phi);

        let (x, y) = xy_from_phi(rphi, phi);

        leaf_vertices.push(Vec3::new(x, y, 0.0));
    }

    indices.append(
        (1..leaf_vertices.len() - 1)
            .map(|i| [0, i as u32, (i + 1) as u32])
            .collect::<Vec<_>>()
            .as_mut(),
    );
    indices.push([1, 0, (leaf_vertices.len() - 1) as u32]);

    vertices.append(&mut leaf_vertices);
    (vertices, indices)
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<GielisParameters>()
        .add_systems(Startup, setup)
        .add_systems(Update, update_mesh)
        .run();
}

#[derive(Component, Reflect)]
pub struct GielisParameters {
    pub gielis: Gielis,
    pub resolution: u32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    let mesh_handle: Handle<Mesh> = meshes.add(Sphere::new(1f32));

    commands
        .spawn(Mesh3d(mesh_handle))
        .insert(MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.8, 0.8),
            ..default()
        })))
        .insert(GielisParameters {
            gielis: Gielis {
                a: 1.0,
                b: 1.0,
                m: 1.0,
                n1: 1.0,
                n2: 1.0,
                n3: 1.0,
            },
            resolution: 64,
        });
}

fn update_mesh(
    gielises: Query<(&GielisParameters, &Mesh3d), Changed<GielisParameters>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (gielis, mesh_handle) in gielises.iter() {
        let (vertices, indices) = generate_leaf_mesh(&gielis.gielis, gielis.resolution);
        let mesh = meshes.get_mut(&mesh_handle.0).unwrap();
        *mesh = bevy_mesh3d(vertices, indices.into_flattened());
    }
}

fn bevy_mesh3d(vertices: Vec<Vec3>, indices: Vec<u32>) -> Mesh {
    // Generate the mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, Default::default());
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices),
    );
    mesh.insert_indices(Indices::U32(indices));
    mesh.compute_normals();
    mesh
}
