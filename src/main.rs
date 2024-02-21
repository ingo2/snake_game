mod game;
mod renderer;

extern crate piston_window;

use game::GameContext;
use game::{DOT_SIZE_IN_PXLS, GRID_X_SIZE, GRID_Y_SIZE};
use piston_window::*;
use renderer::Renderer;

const DESIRED_UPDATE_RATE: u64 = 60;

fn draw_text(
    c: &Context,
    g: &mut G2d,
    glyphs: &mut Glyphs,
    color: [f32; 4],
    pos: [u32; 2],
    text: &str,
) {
    text::Text::new_color(color, 15)
        .draw(
            text,
            glyphs,
            &c.draw_state,
            c.transform.trans(pos[0] as f64, pos[1] as f64),
            g,
        )
        .unwrap();
}

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

    window.set_ups(DESIRED_UPDATE_RATE);

    let mut game = GameContext::new();
    let mut renderer = Renderer::new();

    let fonts = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("fonts")
        .map_err(|e| e.to_string())
        .unwrap_or_else(|e| panic!("Failed to find fonts folder: {}", e));
    let font = fonts.join("Roboto-Regular.ttf");

    // GlyphCache<'_, TextureContext<Factory, Resources, CommandBuffer>, Texture<Resources>>. 
    // What the heck?!
    let mut glyphs = window
        .load_font(&font)
        .unwrap_or_else(|e| panic!("Failed to load font: {}", e));

    let mut frame_counter = 0;

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
                frame_counter += 1;
                if frame_counter % 15 == 0 {
                    game.next_tick();
                }
            }
            // Ignore everything else.
            _ => {}
        }

        // Handle rendering.
        window.draw_2d(&event, |context, graphics, device| {
            if let Err(e) = renderer.draw(&game, &context, graphics) {
                println!("Error rendering: {}", e);
            }
            draw_text(
                &context,
                graphics,
                &mut glyphs,
                [0.9, 0.9, 0.9, 1.0],
                [5, 20],
                "Hello World!",
            );

            glyphs.factory.encoder.flush(device);
        });
    }
}
