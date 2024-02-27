use nalgebra::{vector, Vector2};
use rand::Rng;
use std::collections::VecDeque;

pub const GRID_X_SIZE: i32 = 35;
pub const GRID_Y_SIZE: i32 = 25;
pub const DOT_SIZE_IN_PXLS: i32 = 15;

#[derive(Copy, Clone)]
pub enum GameState {
    Playing,
    Paused,
}

#[derive(Copy, Clone)]
pub enum SnakeDirection {
    Up,
    Down,
    Right,
    Left,
}

impl From<SnakeDirection> for Vector2<i32> {
    fn from(direction: SnakeDirection) -> Vector2<i32> {
        match direction {
            SnakeDirection::Up => vector![0, -1],
            SnakeDirection::Down => vector![0, 1],
            SnakeDirection::Right => vector![1, 0],
            SnakeDirection::Left => vector![-1, 0],
        }
    }
}

pub struct Game {
    pub snake_segments: VecDeque<Vector2<i32>>,
    pub snake_direction: SnakeDirection,
    pub food: Vector2<i32>,
    pub state: GameState,
    pub tick_counter: u32,
    pub tick_rate: u32,
    pub score: u32,
    pub high_score: u32,
}

impl Game {
    pub fn new() -> Game {
        let mut instance = Game {
            snake_segments: VecDeque::new(),
            snake_direction: SnakeDirection::Right,
            state: GameState::Paused,
            tick_counter: 0,
            tick_rate: 15,
            food: vector![0, 0],
            score: 0,
            high_score: 0,
        };
        instance.start();
        instance
    }

    fn start(&mut self) {
        self.snake_segments = VecDeque::from(vec![vector![3, 2], vector![2, 2], vector![1, 2]]);
        self.snake_direction = SnakeDirection::Right;
        self.state = GameState::Paused;
        self.score = 0;
        self.spawn_food();
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(1..GRID_X_SIZE - 1);
            let y = rng.gen_range(1..GRID_Y_SIZE - 1);
            if !self.snake_segments.contains(&vector![x, y]) {
                self.food = vector![x, y];
                break;
            }
        }
    }

    fn check_game_over(&self, next_head_position: &Vector2<i32>) -> bool {
        let x = next_head_position.x;
        let y = next_head_position.y;

        if x < 0 || x >= GRID_X_SIZE || y < 0 || y >= GRID_Y_SIZE {
            return true;
        }

        if self.is_snake_segment(&vector![x, y]) {
            return true;
        }

        false
    }

    pub fn next_tick(&mut self) {
        if let GameState::Paused = self.state {
            return;
        }

        // The tick rate determines how fast the snake moves. The higher the tick
        // rate, the slower the snake moves.
        self.tick_counter += 1;
        if self.tick_counter % self.tick_rate != 0 {
            return;
        }
        self.tick_counter = 0;

        // To create the illusion of movement, we add a new head segment in the direction of travel
        // and remove the last segment. If the new head segment is on the food, we don't remove the
        // last segment, effectively increasing the length of the snake by one.
        let head_position = self.snake_segments.front().unwrap();
        let next_head_position = head_position + Vector2::from(self.snake_direction);

        let reached_food = next_head_position == self.food;
        if !reached_food {
            self.snake_segments.pop_back();
        }

        // If the snake goes out of bounds or hits itself, the game is over.
        if self.check_game_over(&next_head_position) {
            self.start();
            return;
        }

        self.snake_segments.push_front(next_head_position);

        if reached_food {
            self.spawn_food();
            self.score += 1;
            if self.score > self.high_score {
                self.high_score = self.score;
            }

            // Increase the speed of the snake every 3 points.
            if self.score % 3 == 0 && self.tick_rate > 5 {
                self.tick_rate -= 1;
            }
        }
    }

    fn is_snake_segment(&self, point: &Vector2<i32>) -> bool {
        self.snake_segments.contains(point)
    }

    fn illegal_move(&self, dir: SnakeDirection) -> bool {
        // This prevents the snake from reversing direction into itself.
        let head_position = self.snake_segments.front().unwrap();
        let next_head_position = head_position + Vector2::from(dir);
        self.is_snake_segment(&next_head_position)
    }

    pub fn move_up(&mut self) {
        if !self.illegal_move(SnakeDirection::Up) {
            self.snake_direction = SnakeDirection::Up;
        }
    }

    pub fn move_down(&mut self) {
        if !self.illegal_move(SnakeDirection::Down) {
            self.snake_direction = SnakeDirection::Down;
        }
    }

    pub fn move_right(&mut self) {
        if !self.illegal_move(SnakeDirection::Right) {
            self.snake_direction = SnakeDirection::Right;
        }
    }

    pub fn move_left(&mut self) {
        if !self.illegal_move(SnakeDirection::Left) {
            self.snake_direction = SnakeDirection::Left;
        }
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
        }
    }
}
