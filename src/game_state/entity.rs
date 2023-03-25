use tetra::graphics::Texture;
use tetra::math::Vec2;

pub struct Entity {
    pub texture: Texture,
    pub position: Vec2<f32>,
}

impl Entity {
    pub fn new(texture: Texture, position: Vec2<f32>) -> Entity {
        Entity { texture, position }
    }
}
