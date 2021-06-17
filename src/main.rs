use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::collections::HashMap;

mod models;
use models::ball::{ Ball, BallStatus };
use models::player:: Player;
use models::block::Block;
use models::win_status::WinStatus;
use models::game_config::GameConfig;

mod lib;
use lib::{ draw_view, handle_key_pressed, handle_key_released, build_model } ;

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

pub struct Audio {
    sounds: Vec<audrey::read::BufFileReader>,
}

fn model(app: &App) -> Model {
    build_model::execute(app)
}

// 1/60sごとに実行される関数。
fn update(app: &App, model: &mut Model, _update: Update) {
    match model.win_status {
        WinStatus::Normal => {
            let reflect_flg = model.ball.reflect(app, &model.player);
            if reflect_flg { model.ball.clone().reflect_sound(app, model); }
            match model.ball.status {
                BallStatus::Normal => model.ball.go(),
                BallStatus::Failed => {
                    // not implemented.
                    // model.ball.explode(); // ballが爆発するgifにする。
                    model.win_status = WinStatus::GameOver;
                },
            }
        },
        _ => {}
    }
}

pub fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    handle_key_pressed::execute(model, key);
}

pub fn key_released(_app: &App, model: &mut Model, key: Key) {
    handle_key_released::execute(model, key);
}

// A function that renders the given `Audio` to the given `Buffer`.
// In this case we play the audio file.
pub fn audio(audio: &mut Audio, buffer: &mut Buffer) {
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

pub fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(PURPLE);

    // NOTE: ここでdrawを宣言しないとplayerやballが描画されなくなる。
    // https://docs.rs/nannou/0.16.0/src/nannou/app.rs.html#893-897
    // draw()が呼び出す度reset()が走るためです。
    let draw = app.draw();
    draw_view::execute(app, model);
    draw.to_frame(app, &frame).unwrap();
}

