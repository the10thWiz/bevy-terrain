use bevy::prelude::*;

mod terrain;
use terrain::{TerrainPlugin, TerrainManager};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(TerrainPlugin::new())
        .add_systems(Startup, init_terrain)
        .run()
}

fn init_terrain(mut terrain: ResMut<TerrainManager>) {
    terrain.init_terrain();
}
