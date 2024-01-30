use bevy::{prelude::*, tasks::{Task, AsyncComputeTaskPool}};

pub struct TerrainPlugin {}

impl Default for TerrainPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TerrainManager::init());
        // TODO
    }
}

impl TerrainPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Resource)]
pub struct TerrainManager {
    terrain_task: Task<Updates>,
}

impl TerrainManager {
    fn init() -> Self {
        Self {
            terrain_task: AsyncComputeTaskPool::get().spawn(std::future::pending()),
        }
    }

    pub fn init_terrain(&mut self) {
        // TODO
    }
}

#[derive(Debug)]
struct Updates {}

