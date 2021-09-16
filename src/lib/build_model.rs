use std::collections::{ VecDeque, HashMap };

use nannou::app::App;
use nannou::wgpu;
use nannou::geom::vector::vec2;

use crate::{ Model, Audio };
use crate::models::ball::{ Ball, BallStatus, INIT_X, INIT_Y, INIT_R };
use crate::models::block::Block;
use crate::models::player::{ Player, Direction, P_Y, P_SIZE, PACE };
use crate::models::game_config::GameConfig;
use crate::models::win_status::WinStatus;
use crate::models::ticker::Ticker;
use crate::{ key_pressed, key_released, view, audio };
use crate::lib::create_blocks;
use crate::lib::utils::{ load_imgs, retrieve_high_scores};


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
        vec2(0.0, P_Y),
        PACE,
        P_SIZE,
        Direction::Front,
    );

    let win = app.window_rect();
    let blocks: Vec<Block> = create_blocks::execute(1, &win);

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
    let title_textures: HashMap<String, wgpu::Texture> = load_imgs(
        app,
        vec![
          ["title".to_string(), "title_logo.png".to_string()],
        ]
    );
    textures.insert("title".to_string(), title_textures);

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

    // Initialise Ticker
    let mut ticker = Ticker::new(vec![], 0, 0);
    ticker.set_initial_achievements();

    // 1000万点以上とった人がいるとバグる。まぁ...いっかな。
    let min_score = retrieve_high_scores(&10_000_000).last().unwrap().1;
    let game_config = GameConfig::new(
        0,
        0,
        min_score,
        VecDeque::new(),
        false,
        vec!['_', '_', '_', '_', '_', '_', '_', '_', '_', '_'],
        0,
        1
    );
    Model {
        ball,
        player,
        blocks,
        textures,
        stream,
        win_status: WinStatus::Title,
        game_config,
        ticker
    }
}
