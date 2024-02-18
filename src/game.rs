extern crate nalgebra as na;

use na::{vector, Vector2};
use rand::Rng;
use std::collections::VecDeque;

pub const GRID_X_SIZE: i32 = 40;
pub const GRID_Y_SIZE: i32 = 30;
pub const DOT_SIZE_IN_PXLS: i32 = 20;

#[derive(Copy, Clone)]
pub enum GameState {
    Playing,
    Paused,
}

#[derive(Copy, Clone)]
pub enum PlayerDirection {
    Up,
    Down,
    Right,
    Left,
}

impl From<PlayerDirection> for Vector2<i32> {
    fn from(direction: PlayerDirection) -> Vector2<i32> {
        match direction {
            PlayerDirection::Up => vector![0, -1],
            PlayerDirection::Down => vector![0, 1],
            PlayerDirection::Right => vector![1, 0],
            PlayerDirection::Left => vector![-1, 0],
        }
    }
}

pub struct GameContext {
    pub player_position: Vec<Vector2<i32>>,
    pub player_direction: PlayerDirection,
    pub food: Vector2<i32>,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        let mut instance = GameContext {
            player_position: vec![vector![3, 1], vector![2, 1], vector![1, 1]],
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: vector![0, 0],
        };
        instance.spawn_food();
        instance
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..GRID_X_SIZE);
        let y = rng.gen_range(0..GRID_Y_SIZE);
        self.food = vector![x, y];
    }

    pub fn next_tick(&mut self) {
        if let GameState::Paused = self.state {
            return;
        }

        let head_position = self.player_position.first().unwrap();
        let next_head_position = Vector2::from(self.player_direction) + *head_position;

        if next_head_position == self.food {
            self.spawn_food();
        } else {
            self.player_position.pop();
        }
        self.player_position.reverse();
        self.player_position.push(next_head_position);
        self.player_position.reverse();
    }

    pub fn move_up(&mut self) {
        self.player_direction = PlayerDirection::Up;
    }

    pub fn move_down(&mut self) {
        self.player_direction = PlayerDirection::Down;
    }

    pub fn move_right(&mut self) {
        self.player_direction = PlayerDirection::Right;
    }

    pub fn move_left(&mut self) {
        self.player_direction = PlayerDirection::Left;
    }

    pub fn toggle_pause(&mut self) {
        self.state = match self.state {
            GameState::Playing => GameState::Paused,
            GameState::Paused => GameState::Playing,
        }
    }
}
