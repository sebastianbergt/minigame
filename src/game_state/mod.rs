mod entity;

use entity::Entity;

use tetra::graphics::{self, Color, Texture};
use tetra::math::Vec2;
use tetra::{Context, State};

pub struct GameState {
    player: Entity,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let skunk_texture = Texture::new(ctx, "./res/skunk_1F9A8.png")?;
        let start_pose = Vec2::new(100.0f32, 150.0f32);

        Ok(GameState {
            player: Entity::new(skunk_texture, start_pose),
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        self.player.texture.draw(ctx, self.player.position);
        Ok(())
    }
}
