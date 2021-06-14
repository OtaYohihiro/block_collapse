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
    pub score: u16,
    pub hidden_cmds: VecDeque<Key>,
}

impl GameConfig {
    pub fn new(score: u16, hidden_cmds: VecDeque<Key>) -> GameConfig {
        GameConfig {score, hidden_cmds}
    }
}
