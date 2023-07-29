use bevy::prelude::*;

const CHUNK_VOXEL_COUNT: usize = 64;
const CHUNK_WIDTH: usize = 4;

#[derive(Component)]
pub struct World {
    chunks: Vec<Chunk>,
    world_size: u32,
}

pub struct Chunk {
    voxels: [Voxel; CHUNK_VOXEL_COUNT],
}

pub enum Voxel {
    Air,
    Stone,
}

pub fn setup(mut commands: Commands) {
    commands.spawn(World {
        chunks: vec![],
        world_size: 15,
    });
}

impl World {
    fn generate(size: u32) -> World {
        World {
            chunks: vec![],
            world_size: 64,
        }
    }
}

fn main() {
    App::new().add_systems(Update, hello_world).run()
}

pub fn hello_world() {
    println!("Hello world");
}
