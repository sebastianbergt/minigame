use tetra::graphics::{DrawParams, Texture};
use tetra::math::Vec2;
use tetra::Context;

pub struct Entity {
    texture: Texture,
    origin: Vec2<f32>,
    scale: Vec2<f32>,
    position: Vec2<f32>,
    rotation: f32,
}

impl Entity {
    pub fn new(texture: Texture) -> Entity {
        let origin = Vec2 {
            x: texture.width() as f32 / 2.0,
            y: texture.height() as f32 / 2.0,
        };
        Entity {
            texture: texture,
            scale: Vec2 { x: 1.0, y: 1.0 },
            origin: origin,
            position: Vec2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
        }
    }

    // pub fn scale_xy(mut self, scale: Vec2<f32>) -> Entity {
    //     self.scale = scale;
    //     self
    // }

    pub fn scale(mut self, scale: f32) -> Entity {
        self.scale = Vec2::new(scale, scale);
        self
    }

    pub fn position(mut self, position: Vec2<f32>) -> Entity {
        self.position = position;
        self
    }

    pub fn rotation(mut self, rotation: f32) -> Entity {
        self.rotation = rotation;
        self
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.texture.draw(ctx, self.draw_params());
    }

    fn draw_params(&self) -> DrawParams {
        DrawParams::new()
            .origin(self.origin)
            .position(self.position)
            .scale(self.scale)
            .rotation(self.rotation)
    }
}
