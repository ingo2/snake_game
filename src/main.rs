mod game;
mod renderer;

use game::GameContext;
use game::{DOT_SIZE_IN_PXLS, GRID_X_SIZE, GRID_Y_SIZE};
use renderer::Renderer;
use sdl2::{event::Event, keyboard::Keycode};
use std::time::Duration;

// Approximately 60 fps.
const INTENDED_DELTA_TIME_NS: u64 = 1_000_000_000u64 / 60;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(
            "rust-sdl2 snake-game",
            (GRID_X_SIZE * DOT_SIZE_IN_PXLS) as u32,
            (GRID_Y_SIZE * DOT_SIZE_IN_PXLS) as u32,
        )
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut game_context = GameContext::new();
    let mut renderer = Renderer::new(window)?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut frame_counter = 0;
    'running: loop {
        let t0 = std::time::Instant::now();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::W => game_context.move_up(),
                    Keycode::A => game_context.move_left(),
                    Keycode::S => game_context.move_down(),
                    Keycode::D => game_context.move_right(),
                    Keycode::Escape => game_context.toggle_pause(),
                    _ => {}
                },
                _ => {}
            }
        }

        frame_counter += 1;
        if (frame_counter % 15) == 0 {
            game_context.next_tick();
        }

        renderer.draw(&game_context)?;

        // This aims for roughly 60 fps.
        let t1 = std::time::Instant::now();
        let delta: Duration = t1.duration_since(t0);
        let sleep_duration = if delta < Duration::from_nanos(INTENDED_DELTA_TIME_NS) {
            Duration::from_nanos(INTENDED_DELTA_TIME_NS) - delta
        } else {
            Duration::from_nanos(1_000) // Pauses for 1 microsecond to avoid busy waiting.
        };

        ::std::thread::sleep(sleep_duration);
    }

    Ok(())
}
