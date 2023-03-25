use tetra::graphics::{self, Color, Texture};
use tetra::math::Vec2;
use tetra::{Context, State};

pub struct GameState {
    player_texture: Texture,
    block_texture: Texture,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<Self> {
        let player_texture = Texture::new(ctx, "./res/skunk_1F9A8.png")?;
        let block_texture = Texture::new(ctx, "./res/block_2B1B.png")?;
        let game_state = GameState {
            player_texture,
            block_texture,
        };
        Ok(game_state)
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        self.player_texture.draw(ctx, Vec2::new(16.0, 16.0));
        Ok(())
    }
}
