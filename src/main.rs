mod game_state;
use game_state::GameState;
mod maze;
use tetra::ContextBuilder;
fn main() -> tetra::Result {
    ContextBuilder::new("Skunk", 1024, 768)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}
