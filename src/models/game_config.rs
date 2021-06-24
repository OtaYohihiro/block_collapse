use nannou::prelude::Key;
use std::collections::VecDeque;

pub const SPEED_UP_CMD: [Key; 5] = [
    Key::Left, Key::Left, Key::Right, Key::Right, Key::Up
];
pub const SPEED_DOWN_CMD: [Key; 5] = [
    Key::Left, Key::Left, Key::Right, Key::Right, Key::Down
];

pub const CLEAR_CMD: [Key; 5] = [
    Key::C, Key::C, Key::C, Key::C, Key::C
];

pub struct GameConfig {
    pub score: usize,
    pub min_score: usize,
    pub hidden_cmds: VecDeque<Key>,
}

impl GameConfig {
    pub fn new(score: usize, min_score: usize, hidden_cmds: VecDeque<Key>) -> GameConfig {
        GameConfig {score, min_score, hidden_cmds}
    }

    pub fn set_initial_state(&mut self) {
        self.score = 0;
    }
}
