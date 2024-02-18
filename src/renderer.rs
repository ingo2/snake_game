use crate::game::DOT_SIZE_IN_PXLS;
use crate::game::{GameContext, GameState};
use nalgebra::Vector2;

use piston_window::*;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn draw(&mut self, game: &GameContext, c: Context, g: &mut G2d) -> Result<(), String> {
        self.draw_background(game, g);
        self.draw_player(game, c, g)?;
        self.draw_food(game, c, g)?;

        Ok(())
    }

    fn draw_background(&mut self, game: &GameContext, g: &mut G2d) {
        let color = match game.state {
            GameState::Playing => [0.0, 0.0, 0.0, 1.0],
            GameState::Paused => [0.1, 0.1, 0.1, 1.0],
        };
        clear(color, g);
    }

    fn draw_dot(
        &mut self,
        point: &Vector2<i32>,
        color: [f32; 4],
        c: Context,
        g: &mut G2d,
    ) -> Result<(), String> {
        rectangle(
            color,
            [
                (point.x * DOT_SIZE_IN_PXLS) as f64,
                (point.y * DOT_SIZE_IN_PXLS) as f64,
                DOT_SIZE_IN_PXLS as f64,
                DOT_SIZE_IN_PXLS as f64,
            ],
            c.transform,
            g,
        );

        Ok(())
    }

    fn draw_player(&mut self, game: &GameContext, c: Context, g: &mut G2d) -> Result<(), String> {
        let green = [0.0, 1.0, 0.0, 1.0];
        for point in &game.player_position {
            self.draw_dot(point, green, c, g)?;
        }

        Ok(())
    }

    fn draw_food(&mut self, game: &GameContext, c: Context, g: &mut G2d) -> Result<(), String> {
        let red = [1.0, 0.0, 0.0, 1.0];
        self.draw_dot(&game.food, red, c, g)?;

        Ok(())
    }
}
