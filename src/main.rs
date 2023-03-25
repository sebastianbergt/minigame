mod game_state;
use game_state::GameState;
use tetra::ContextBuilder;
fn main() -> tetra::Result {
    ContextBuilder::new("Pong", 640, 480)
        .quit_on_escape(true)
        .build()?
        .run(|_| Ok(GameState {}))
}
