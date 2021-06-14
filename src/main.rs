use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::collections::HashMap;
use std::collections::VecDeque;

mod models;
use models::ball::{Ball, INIT_X, INIT_Y};
use models::player::Player;
use models::player::Direction;
use models::win_status::WinStatus;
use models::game_config::{ GameConfig, SPEED_UP_CMD, SPEED_DOWN_CMD, CLEAR_CMD };

mod lib;
use lib::utils::*;
use lib::draw_view::*;

fn main() {
    nannou::app(model)
        //.event(event)
        .update(update)
        //.simple_window(view)
        .run();
}

pub struct Model {
    ball: Ball,
    player: Player,
    textures: HashMap<String, HashMap<String, wgpu::Texture>>,
    stream: audio::Stream<Audio>,
    win_status: WinStatus,
    game_config: GameConfig,
}

struct Audio {
    sounds: Vec<audrey::read::BufFileReader>,
}

fn model(app: &App) -> Model {
    app.new_window()
        //.size(600, 400)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .view(view)
        .build()
        .unwrap();

    let ball = Ball::new(
        vec2(0.0, 0.0),
        vec2(INIT_X, INIT_Y),
    );
    let player = Player::new(
        pt2(0.0, -280.0),
        vec2(45.0, 45.0),
        Direction::Front,
    );

    // Initialize textures.
    let mut textures = HashMap::new();
    let player_textures: HashMap<String, wgpu::Texture> = load_imgs(
        app,
        vec![
          ["normal".to_string(), "tibichar.gif".to_string()],
          ["l_run".to_string(), "tibichar_l_run.gif".to_string()],
          ["r_run".to_string(), "tibichar_r_run.gif".to_string()],
        ]
    );
    textures.insert("player".to_string(), player_textures);

    // Initialise the audio host so we can spawn an audio stream.
    let audio_host = audio::Host::new();
    // Initialise the state that we want to live on the audio thread.
    let sounds = vec![];
    let model = Audio { sounds };
    let stream = audio_host
        .new_output_stream(model)
        .render(audio)
        .build()
        .unwrap();

    let game_config = GameConfig::new(0, VecDeque::new());
    Model {
        ball,
        player,
        textures,
        stream,
        win_status: WinStatus::Normal,
        game_config,
    }
}

// 1/60sごとに実行される関数。
fn update(app: &App, model: &mut Model, _update: Update) {
    match model.win_status {
        WinStatus::Normal => {
            let reflect_flg = model.ball.reflect(app);
            if reflect_flg { model.ball.clone().reflect_sound(app, model) }
            model.ball.go();
        },
        _ => {}
    }
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match model.win_status {
        WinStatus::Normal => {
            // 隠しコマンド判定
            let mut hidden_cmds = model.game_config.hidden_cmds.clone();
            hidden_cmds.push_back(key);
            if hidden_cmds.len() == 6 { hidden_cmds.pop_front(); }
            // 隠しコマンド発動
            if hidden_cmds == SPEED_UP_CMD {
                model.ball.madly_speed_up();
            } else if hidden_cmds == SPEED_DOWN_CMD {
                model.ball.quite_speed_down();
            } else if hidden_cmds == CLEAR_CMD {
                model.ball.set_initial_speed();
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
                    model.win_status = WinStatus::Pause;
                },
                _ => {}
            }
        },
        WinStatus::Pause => {
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
        _ => {}
    }
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
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

// A function that renders the given `Audio` to the given `Buffer`.
// In this case we play the audio file.
fn audio(audio: &mut Audio, buffer: &mut Buffer) {
    let mut have_ended = vec![];
    let len_frames = buffer.len_frames();

    // Sum all of the sounds onto the buffer.
    for (i, sound) in audio.sounds.iter_mut().enumerate() {
        let mut frame_count = 0;
        let file_frames = sound.frames::<[f32; 2]>().filter_map(Result::ok);
        for (frame, file_frame) in buffer.frames_mut().zip(file_frames) {
            for (sample, file_sample) in frame.iter_mut().zip(&file_frame) {
                *sample += *file_sample;
            }
            frame_count += 1;
        }

        // If the sound yielded less samples than are in the buffer, it must have ended.
        if frame_count < len_frames {
            have_ended.push(i);
        }
    }

    // Remove all sounds that have ended.
    for i in have_ended.into_iter().rev() {
        audio.sounds.remove(i);
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(PURPLE);

    // NOTE: ここでdrawを宣言しないとplayerやballが描画されなくなる。
    // https://docs.rs/nannou/0.16.0/src/nannou/app.rs.html#893-897
    // draw()が呼び出す度reset()が走るためです。
    let draw = app.draw();
    // 通常のプレイ画面を描画する。
    draw_playing_view(app, model);
    draw.to_frame(app, &frame).unwrap();
}

