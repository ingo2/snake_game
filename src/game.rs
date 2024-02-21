extern crate nalgebra as na;

use na::{vector, Vector2};
use rand::Rng;
use std::collections::VecDeque;

type Vec2i = na::Vector2<i32>;

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

impl From<SnakeDirection> for Vec2i {
    fn from(direction: SnakeDirection) -> Vec2i {
        match direction {
            SnakeDirection::Up => vector![0, -1],
            SnakeDirection::Down => vector![0, 1],
            SnakeDirection::Right => vector![1, 0],
            SnakeDirection::Left => vector![-1, 0],
        }
    }
}

pub struct GameContext {
    pub snake_segments: VecDeque<Vec2i>,
    pub snake_direction: SnakeDirection,
    pub food: Vec2i,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        let mut instance = GameContext {
            snake_segments: VecDeque::new(),
            snake_direction: SnakeDirection::Right,
            state: GameState::Paused,
            food: vector![0, 0],
        };
        instance.start();
        instance
    }

    fn start(&mut self) {
        self.snake_segments = VecDeque::from(vec![vector![3, 2], vector![2, 2], vector![1, 2]]);
        self.snake_direction = SnakeDirection::Right;
        self.state = GameState::Paused;
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

    fn check_game_over(&self, next_head_position: &Vec2i) -> bool {
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

        let head_position = self.snake_segments.front().unwrap();
        let next_head_position = head_position + Vector2::from(self.snake_direction);

        let reached_food = next_head_position == self.food;
        if !reached_food {
            self.snake_segments.pop_back();
        }

        if self.check_game_over(&next_head_position) {
            self.start();
            return;
        }

        self.snake_segments.push_front(next_head_position);

        if reached_food {
            self.spawn_food();
        }
    }

    fn is_snake_segment(&self, point: &Vec2i) -> bool {
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
