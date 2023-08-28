
pub struct TerrainPlugin {}

impl Default for TerrainPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        // TODO
    }
}

impl TerrainPlugin {
    pub fn new() -> Self {
        Self {}
    }
}
