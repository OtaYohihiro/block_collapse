use nannou::prelude::Key;

use crate::Model;
use crate::models::win_status::WinStatus;
use crate::models::player::Direction;

pub fn execute(model: &mut Model, key: Key) {
    match model.win_status {
        WinStatus::Normal => {
            match key {
                Key::Left => {
                    model.player.dir = Direction::Front;
                },
                Key::Right => {
                    model.player.dir = Direction::Front;
                },
                _ => {}
            }
        },
        _ => {}
    }
}
