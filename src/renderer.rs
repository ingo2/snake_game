use crate::game::DOT_SIZE_IN_PXLS;
use crate::game::{Game, GameState};

use piston_window::*;

type Vec2i = nalgebra::Vector2<i32>;

const FOOD_COLOR: [f32; 4] = [0.85, 0.1, 0.1, 1.0];
const SNAKE_COLOR: [f32; 4] = [0.1, 0.65, 0.1, 1.0];
const SCORE_COLOR: [f32; 4] = [0.7, 0.4, 0.9, 1.0];

pub struct Renderer {
    glyphs: Option<Glyphs>,
}

impl Renderer {
    pub fn new() -> Self {
        Renderer { glyphs: None }
    }

    pub fn init(&mut self, window: &mut PistonWindow) -> Result<(), String> {
        let fonts = find_folder::Search::ParentsThenKids(3, 3)
            .for_folder("fonts")
            .map_err(|e| e.to_string())?;

        let font = fonts.join("Roboto-Regular.ttf");

        self.glyphs = Some(window.load_font(&font).map_err(|e| e.to_string())?);

        Ok(())
    }

    pub fn draw(
        &mut self,
        game: &Game,
        c: &Context,
        g: &mut G2d,
        d: &mut GfxDevice,
    ) -> Result<(), String> {
        self.draw_background(game, g);
        self.draw_score(&game, c, g, d)?;
        self.draw_snake(game, c, g)?;
        self.draw_food(game, c, g)?;

        Ok(())
    }

    fn draw_background(&mut self, game: &Game, g: &mut G2d) {
        let color = match game.state {
            GameState::Playing => [0.0, 0.0, 0.0, 1.0],
            GameState::Paused => [0.1, 0.1, 0.1, 1.0],
        };

        clear(color, g);
    }

    fn draw_score(
        &mut self,
        game: &Game,
        c: &Context,
        g: &mut G2d,
        d: &mut GfxDevice,
    ) -> Result<(), String> {
        let glyphs = self.glyphs.as_mut().ok_or("Glyphs not initialized")?;

        let text = format!("Score: {} | High-Score: {}", game.score, game.high_score);

        text::Text::new_color(SCORE_COLOR, 15)
            .draw(
                text.as_ref(),
                glyphs,
                &c.draw_state,
                c.transform.trans(5.0, 18.0),
                g,
            )
            .map_err(|e| e.to_string())?;

        glyphs.factory.encoder.flush(d);

        Ok(())
    }

    fn draw_dot(
        &mut self,
        point: &Vec2i,
        color: [f32; 4],
        c: &Context,
        g: &mut G2d,
    ) -> Result<(), String> {
        let x = (point.x * DOT_SIZE_IN_PXLS) as f64;
        let y = (point.y * DOT_SIZE_IN_PXLS) as f64;

        rectangle(
            color,
            [x, y, DOT_SIZE_IN_PXLS as f64, DOT_SIZE_IN_PXLS as f64],
            c.transform,
            g,
        );

        Ok(())
    }

    fn draw_snake(&mut self, game: &Game, c: &Context, g: &mut G2d) -> Result<(), String> {
        for point in &game.snake_segments {
            self.draw_dot(point, SNAKE_COLOR, c, g)?;
        }

        Ok(())
    }

    fn draw_food(&mut self, game: &Game, c: &Context, g: &mut G2d) -> Result<(), String> {
        self.draw_dot(&game.food, FOOD_COLOR, c, g)?;

        Ok(())
    }
}
