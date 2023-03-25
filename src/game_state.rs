use tetra::graphics::{self, Color};
use tetra::{Context, State};
pub struct GameState {}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::BLACK);

        Ok(())
    }
}

// Tests module
// #[cfg(test)]
// mod tests {
//     // Import the parent module's functions or types
//     use super::*;

//     // Write your unit tests as functions with the #[test] attribute
//     #[test]
//     fn test_add() {
//         assert_eq!(add(1, 2), 3);
//         assert_eq!(add(0, 0), 0);
//     }
// }
