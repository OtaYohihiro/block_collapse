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
    pub confirming: bool, // 確認windowが出ているかどうか。よく出ると思うので、attributeとして持つ。
    pub input_field: Vec<char>,
    pub input_cursor: usize // そんな文字はいらないけども、index操作することが多い関係で、usizeのほうが良い。
}

impl GameConfig {
    pub fn new(
        score: usize,
        min_score: usize,
        hidden_cmds: VecDeque<Key>,
        confirming: bool,
        input_field: Vec<char>,
        input_cursor: usize
    ) -> GameConfig {
        GameConfig {
            score, min_score, hidden_cmds,
            confirming, input_field, input_cursor
        }
    }

    pub fn set_initial_state(&mut self) {
        self.score = 0;
        self.set_initial_input_state();
        // self.confirming = false;
    }

    pub fn set_initial_input_state(&mut self) {
        self.input_field = vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'];
        self.input_cursor = 0;
    }
}
