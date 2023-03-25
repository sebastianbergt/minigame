mod entity;
use entity::Entity;
mod world;
use tetra::graphics::{self, Color, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, State};

pub struct GameState {
    player: Entity,
    world: Vec<Entity>,
}

const SCALE: f32 = 0.25;

impl GameState {
    pub fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let skunk_texture = Texture::new(ctx, "./res/skunk_1F9A8.png")?;
        let tree_texture = Texture::new(ctx, "./res/skunk_1F9A8.png")?;
        let start_position = Vec2::new(100.0, 150.0);

        let player = Entity::new(skunk_texture)
            .scale(SCALE)
            .position(start_position)
            .rotation(0.0);

        let world: Vec<Entity> = world::generate(SCALE, tree_texture);

        Ok(GameState {
            player: player,
            world: world,
        })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);
        self.player.draw(ctx);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        const STEP_SIZE: f32 = 618.0 * SCALE;
        if input::is_key_down(ctx, Key::W) {
            self.player.position.y -= STEP_SIZE;
            self.player.rotation = std::f32::consts::FRAC_PI_2;
        }
        if input::is_key_down(ctx, Key::S) {
            self.player.position.y += STEP_SIZE;
            self.player.rotation = -std::f32::consts::FRAC_PI_2;
        }
        if input::is_key_down(ctx, Key::A) {
            self.player.position.x -= STEP_SIZE;
            self.player.rotation = 0.0;
        }
        if input::is_key_down(ctx, Key::D) {
            self.player.position.x += STEP_SIZE;
            self.player.rotation = std::f32::consts::PI;
        }

        Ok(())
    }
}
