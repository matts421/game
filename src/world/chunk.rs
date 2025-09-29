use bevy::prelude::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VoxelId {
    Air = 0,
    Dirt = 1,
    Grass = 2,
    Stone = 3,
}

/// Identifies a chunk’s position in the world
#[derive(Component)]
struct ChunkCoord(IVec3);

/// Stores the voxel data for this chunk
#[derive(Component)]
struct VoxelData {
    voxels: Vec<VoxelId>, // flat Vec<>, index = x + y*width + z*width*height
    size: UVec3,          // e.g. 16x16x256
}
impl VoxelData {
    fn index(&self, x: u32, y: u32, z: u32) -> usize {
        (x + y * self.size.x + z * self.size.x * self.size.y) as usize
    }
    fn get(&self, x: u32, y: u32, z: u32) -> VoxelId {
        self.voxels[self.index(x, y, z)]
    }
}

/// Mesh handle for rendering
#[derive(Component)]
struct ChunkMesh {
    handle: Handle<Mesh>,
}

// commands.spawn((
//     ChunkCoord(chunk_pos),
//     VoxelData::new(size),
//     MaterialMeshBundle {
//         mesh: meshes.add(chunk_mesh),
//         material: voxel_material.clone(),
//         transform: Transform::from_translation(chunk_world_pos),
//         ..default()
//     },
// ));

// fn update_chunk_meshes(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut query: Query<(Entity, &VoxelData, &mut ChunkMesh), Changed<VoxelData>>,
// ) {
//     for (entity, voxels, mut mesh) in query.iter_mut() {
//         let new_mesh = generate_mesh(&voxels);
//         mesh.handle = meshes.add(new_mesh);
//         commands.entity(entity).insert(mesh.handle.clone());
//     }
// }

// struct ChunkMap(HashMap<IVec3, Entity>);
