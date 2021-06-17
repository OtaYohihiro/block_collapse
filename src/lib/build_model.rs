use std::collections::{ VecDeque, HashMap };

use nannou::app::App;
use nannou::wgpu;
use nannou::geom::vector::vec2;
use nannou::geom::point::pt2;

use crate::{ Model, Audio };
use crate::models::ball::{ Ball, BallStatus, INIT_X, INIT_Y, INIT_R };
use crate::models::win_status::WinStatus;
use crate::models::player::{ Player, Direction, P_Y, P_SIZE };
use crate::models::game_config::GameConfig;
use crate::{ key_pressed, key_released, view, audio };
use crate::lib::utils::load_imgs;



pub fn execute(app: &App) -> Model {
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
