extern crate nalgebra as na;

use na::{vector, Vector2};
use rand::Rng;
use std::collections::VecDeque;

type Vec2i = na::Vector2<i32>;

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

impl From<PlayerDirection> for Vec2i {
    fn from(direction: PlayerDirection) -> Vec2i {
        match direction {
            PlayerDirection::Up => vector![0, -1],
            PlayerDirection::Down => vector![0, 1],
            PlayerDirection::Right => vector![1, 0],
            PlayerDirection::Left => vector![-1, 0],
        }
    }
}

pub struct GameContext {
    pub player_position: VecDeque<Vec2i>,
    pub player_direction: PlayerDirection,
    pub food: Vec2i,
    pub state: GameState,
}

impl GameContext {
    pub fn new() -> GameContext {
        let mut instance = GameContext {
            player_position: VecDeque::from(vec![vector![3, 1], vector![2, 1], vector![1, 1]]),
            player_direction: PlayerDirection::Right,
            state: GameState::Paused,
            food: vector![0, 0],
        };
        instance.spawn_food();
        instance
    }

    fn spawn_food(&mut self) {
        let mut rng = rand::thread_rng();
        loop {
            let x = rng.gen_range(1..GRID_X_SIZE - 1) as i32;
            let y = rng.gen_range(1..GRID_Y_SIZE - 1) as i32;
            if !self.player_position.contains(&vector![x, y]) {
                self.food = vector![x, y];
                break;
            }
        }
    }

    pub fn next_tick(&mut self) {
        if let GameState::Paused = self.state {
            return;
        }

        let head_position = self.player_position.front().unwrap();
        let next_head_position = Vector2::from(self.player_direction) + *head_position;

        let reached_food = next_head_position == self.food;

        if !reached_food {
            self.player_position.pop_back();
        }

        self.player_position.push_front(next_head_position);

        if reached_food {
            self.spawn_food();
        }
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
