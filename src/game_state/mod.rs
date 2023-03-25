mod entity;

use entity::Entity;

use tetra::graphics::{self, Color, Texture};
use tetra::math::Vec2;
use tetra::{Context, State};

pub struct GameState {
    player: Entity,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let skunk_texture = Texture::new(ctx, "./res/skunk_1F9A8.png")?;
        let start_position = Vec2::new(100.0, 150.0);

        let player = Entity::new(skunk_texture)
            .scale(0.25)
            .position(start_position)
            .rotation(std::f32::consts::FRAC_PI_2);

        Ok(GameState { player: player })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        self.player.draw(ctx);
        Ok(())
    }
}
