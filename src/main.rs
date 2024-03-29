mod game;
mod renderer;

use game::Game;
use game::{DOT_SIZE_IN_PXLS, GRID_X_SIZE, GRID_Y_SIZE};
use piston_window::*;
use renderer::Renderer;

const DESIRED_UPDATE_RATE: u64 = 60;

fn main() {
    let mut window: PistonWindow = WindowSettings::new(
        "Happy little Snake Game",
        [
            (GRID_X_SIZE * DOT_SIZE_IN_PXLS) as u32,
            (GRID_Y_SIZE * DOT_SIZE_IN_PXLS) as u32,
        ],
    )
    .exit_on_esc(true)
    .resizable(false)
    .build()
    .unwrap_or_else(|e| panic!("Failed to build window: {}", e));

    window.set_ups(DESIRED_UPDATE_RATE);

    let mut game = Game::new();
    let mut renderer = Renderer::new();
    renderer
        .init(&mut window)
        .unwrap_or_else(|e| panic!("Failed to initialize renderer: {}", e));

    while let Some(event) = window.next() {
        match event {
            // Handle keyboard input.
            Event::Input(
                Input::Button(ButtonArgs {
                    state: ButtonState::Press,
                    button: Button::Keyboard(key),
                    ..
                }),
                ..,
            ) => {
                match key {
                    Key::W | Key::Up => game.move_up(),
                    Key::A | Key::Left => game.move_left(),
                    Key::S | Key::Down => game.move_down(),
                    Key::D | Key::Right => game.move_right(),
                    Key::Space => game.toggle_pause(),
                    _ => {} // Ignore all other keys.
                }
            }
            // Handle updates.
            Event::Loop(Loop::Update(_)) => {
                game.next_tick();
            }
            // Ignore everything else.
            _ => {}
        }

        // Handle rendering.
        window.draw_2d(&event, |c, g, d| {
            if let Err(e) = renderer.draw(&game, &c, g, d) {
                println!("Error rendering: {}", e);
            }
        });
    }
}
