use crate::game_start::Location;

use crate::{game_start::Block, gfx::SceneDescribe};
struct Scene {}

impl SceneDescribe for Scene {
    fn ready_model(&self) {
        // vertex buffer
    }

    fn ready_texture(&self) {}

    fn model_to_buffer(&self) {}

    fn camera_setting(&self) {
        // camera uniform buffer
    }

    fn update(&self) {}
}

impl Scene {
    fn create_blocks(&self) {
        let block1 = Block::Entity(Location { x: 0, y: 0 });
        let block2 = Block::Entity(Location { x: 1, y: 0 });
    }
}
