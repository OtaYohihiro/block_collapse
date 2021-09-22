use std::collections::HashMap;

use nannou::prelude::*;
use nannou::ui::color::{WHITE, TRANSPARENT};

use nannou_audio as audio;
use nannou_audio::Buffer;

mod models;
use models::ball::{ Ball, BallStatus };
use models::block::{ Block, MAX_B_NUM };
use models::effect::{ Effect, Object };
use models::game_config::GameConfig;
use models::player:: Player;
use models::win_status::WinStatus;
use models::ticker::Ticker;
// https://diary.kitaitimakoto.net/2018/02/20.html
// まじで迷った。
use models::concerns::reflect::ReflectLogic;

mod lib;
use lib::{
    draw_view,
    handle_key_pressed,
    handle_key_released,
    build_model,
} ;
use lib::utils::to_rgba;
use lib::utils_for_effect::{ add_effect, update_effect };

fn main() {
    nannou::app(model)
        //.event(event)
        .update(update)
        //.simple_window(view)
        .run();
}

// NOTE: これってそういえばなんでattributeにpubつけないで良いんだろう。
pub struct Model {
    ball: Ball,
    player: Player,
    blocks: Vec<Block>,
    textures: HashMap<String, HashMap<String, wgpu::Texture>>,
    stream: audio::Stream<Audio>,
    win_status: WinStatus,
    game_config: GameConfig,
    effect_vec: Vec<Effect>,
    ticker: Ticker,
}

pub struct Audio {
    sounds: Vec<audrey::read::BufFileReader>,
}

fn model(app: &App) -> Model {
    build_model::execute(app)
}

// 1/60sごとに実行される関数。
fn update(app: &App, model: &mut Model, _update: Update) {
    let mut c_ticker = model.ticker.clone();
    c_ticker.notify_observer(model, app.time);
    update_effect(model);
    match model.win_status {
        WinStatus::Normal => {
            let mut c_ball = model.ball;
            let reflect_flg = c_ball.reflect(app, &model.player);
            if reflect_flg {
                c_ball.reflect_sound(app, model);

                let effect = Effect::new(
                    2.0,
                    Object::Ball,
                    c_ball.p.clone(),
                    vec2(0.0, 0.0),
                    (to_rgba(WHITE), to_rgba(TRANSPARENT)),
                    vec![("shape_effect", "fadeout"), ("color_effect", "fadeout"),("shape", "cross")]
                );
                add_effect(model, effect);

                model.game_config.reflect_cnt += 1;
            }

            let mut c_blocks = model.blocks.clone();
            let mut index = MAX_B_NUM;
            for (idx, block) in c_blocks.iter_mut().enumerate() {
                if block.reflect(app, &mut c_ball) {
                    model.game_config.score += 10;
                    block.reflect_sound(app, model);
                    if block.life == 0 { index = idx as u16 }
                    break;
                }
            }
            model.blocks = c_blocks;
            model.ball = c_ball;
            if index != MAX_B_NUM { model.blocks.remove(index.into()); }

            match model.ball.status {
                BallStatus::Normal => model.ball.go(),
                BallStatus::Failed => {
                    // not implemented.
                    // model.ball.explode(); // ballが爆発するgifにする。
                    if model.game_config.score >= model.game_config.min_score {
                        model.win_status = WinStatus::RecordBreak;
                    } else {
                        model.win_status = WinStatus::GameOver;
                    }
                },
            }
        },
        _ => {}
    }
    model.ticker = c_ticker;
}

pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    handle_key_pressed::execute(model, key, app);
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

