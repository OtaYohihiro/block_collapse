use nannou::prelude::Key;

use crate::Model;
use crate::models::win_status::WinStatus;
use crate::models::player::Direction;
use crate::models::game_config::{ SPEED_UP_CMD, SPEED_DOWN_CMD, CLEAR_CMD };
use crate::lib::utils::{ set_initial_state, handle_input };

pub fn execute(model: &mut Model, key: Key) {
    match model.win_status {
        WinStatus::Normal => {
            // 隠しコマンド判定
            let mut hidden_cmds = model.game_config.hidden_cmds.clone();
            hidden_cmds.push_back(key);
            if hidden_cmds.len() == 6 { hidden_cmds.pop_front(); }
            // 隠しコマンド発動. VecQueueとArrayの比較なので、matchは使えない。
            if hidden_cmds == SPEED_UP_CMD {
                model.ball.madly_speed_up();
            } else if hidden_cmds == SPEED_DOWN_CMD {
                model.ball.quite_speed_down();
            } else if hidden_cmds == CLEAR_CMD {
                model.ball.set_initial_state();
            }

            model.game_config.hidden_cmds = hidden_cmds;

            match key {
                Key::Left => {
                    model.player.go(-1);
                    model.player.dir = Direction::Left;
                },
                Key::Right => {
                    model.player.go(1);
                    model.player.dir = Direction::Right;
                },
                Key::X => {
                    println!("X pressed!!");
                    model.win_status = WinStatus::Menu;
                },
                _ => {}
            }
        },
        WinStatus::Menu => {
            match key {
                Key::Up => println!("Up!!"),
                Key::Down => println!("Down!!"),
                Key::X => {
                    println!("X pressed!!");
                    model.win_status = WinStatus::Normal;
                },
                _ => {}
            }
        },
        WinStatus::GameOver => {
            match key {
                Key::R => {
                    // ゲーム画面初期化処理
                    set_initial_state(model);
                },
                Key::T => {
                    model.win_status = WinStatus::Title;
                },
                _ => {}
            }
        },
        WinStatus::Title => {
            match key {
                Key::S => {
                    set_initial_state(model);
                },
                _ => {}
            }
        },
        WinStatus::RecordBreak => {
            // TODO: confirming増えてきたら、Stateパターンにしたほうが良いかも。
            if model.game_config.confirming {
                match key {
                    Key::Return => {
                        model.game_config.confirming = false;
                        model.win_status = WinStatus::Title;
                    },
                    Key::X => {
                        model.game_config.confirming = false;
                    }
                    _ => {}
                }

            } else {
                match key {
                    Key::Return => {
                        model.game_config.confirming = true;
                    },
                    _ => {
                        handle_input(model, key, 6);
                    }
                }
            }
        },
    }
}
