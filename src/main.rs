mod game;
mod renderer;

extern crate piston_window;

use game::GameContext;
use game::{DOT_SIZE_IN_PXLS, GRID_X_SIZE, GRID_Y_SIZE};
use piston_window::*;
use renderer::Renderer;

const DESIRED_FRAME_RATE: u64 = 60;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Happy little Snake Game",
        [
            (GRID_X_SIZE * DOT_SIZE_IN_PXLS) as u32,
            (GRID_Y_SIZE * DOT_SIZE_IN_PXLS) as u32,
        ],
    )
    .exit_on_esc(true)
    .build()
    .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    window.set_ups(DESIRED_FRAME_RATE);

    let mut game = GameContext::new();
    let mut renderer = Renderer::new();
    let mut frame_counter = 0;

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W => game.move_up(),
                Key::A => game.move_left(),
                Key::S => game.move_down(),
                Key::D => game.move_right(),
                Key::Space => game.toggle_pause(),
                _ => {} // Ignore all other keys.
            }
        }

        window.draw_2d(&event, |c, g, _| {
            // TODO: Updating state should be decoupled from drawing.
            frame_counter += 1;
            if (frame_counter % 15) == 0 {
                game.next_tick();
            }

            match renderer.draw(&game, c, g) {
                Ok(_) => {}
                Err(e) => println!("Error rendering: {}", e),
            }
        });
    }
}
