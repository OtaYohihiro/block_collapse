use nannou::prelude::*;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::collections::HashMap;
use std::collections::VecDeque;

mod models;
use models::ball::{ Ball, INIT_X, INIT_Y, INIT_R, BallStatus };
use models::player::{ Player, P_Y, P_SIZE };
use models::player::Direction;
use models::win_status::WinStatus;
use models::game_config::{ GameConfig, SPEED_UP_CMD, SPEED_DOWN_CMD, CLEAR_CMD };

mod lib;
use lib::utils::*;
use lib::draw_view;
use lib::{ handle_key_pressed, handle_key_released } ;

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
        INIT_R,
        0.0,
        BallStatus::Normal,
    );
    let player = Player::new(
        pt2(0.0, P_Y),
        vec2(P_SIZE, P_SIZE),
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

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    handle_key_pressed::execute(model, key);
}

fn key_released(_app: &App, model: &mut Model, key: Key) {
    handle_key_released::execute(model, key);
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
    draw_view::execute(app, model);
    draw.to_frame(app, &frame).unwrap();
}

