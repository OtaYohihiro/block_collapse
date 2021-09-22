use std::collections::HashMap;
use std::io::{ Write, Read };
use std::fs::{File, copy};
use std::fs::OpenOptions;
use std::path::Path;

use nannou::app::App;
use nannou::prelude::Key;
use nannou::geom::rect::Rect;
use nannou::app::DrawScalar;
use nannou::ui::prelude::color::Color;
use nannou::ui::color;

use nannou::color::{Alpha, Rgb, rgba};

use crate::wgpu;
use crate::Model;
use crate::models::win_status::WinStatus;
use crate::lib::create_blocks;

const RESULT_PATH: &str = "/Users/ota/project/rust/block_collapse/result.txt";
const ACHIEVEMENT_PATH: &str = "/Users/ota/project/rust/block_collapse/achievement.txt";
pub const NO_NAME: &str = "no_name";

// TODO: 引数に&mut modelをとっているような関数用に
// wrappedModel作って、それのメソッドとして定義したい。

fn load_img(app: &App, file_name: &str) -> wgpu::Texture {
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("img").join(file_name);
    return wgpu::Texture::from_path(app, img_path).unwrap();
}

pub fn load_imgs(
    app: &App,
    file_infos: Vec<[String; 2]>
) -> HashMap<String, wgpu::Texture> {
    let mut res: HashMap<String, wgpu::Texture> = HashMap::new();
    for file_info in file_infos.iter() {
        res.insert(file_info[0].clone(), load_img(app, &file_info[1]));
    }

    return res
}

pub fn set_initial_state(model: &mut Model) {
    model.win_status = WinStatus::Title;
    model.ball.set_initial_state();
    model.player.set_initial_state();
    model.game_config.set_initial_state();
    save_achievements(model);
}

// TODO: Log見て気づいたが、これgameover画面で、1/60で呼び出されている。
// file readをその頻度で呼ぶのはおそらく大分良くないから、初回だけ読んで、
// 後はcacheデータから呼ぶみたいにしないとエコでなさそう。
// まぁ動いているから一旦おいておこう。
pub fn retrieve_high_scores(score: &usize) -> Vec<(String, usize)> {
   if !Path::new(RESULT_PATH).exists() {
        copy("result_example.txt", RESULT_PATH).expect("file copy failed");
    }

    // TODO: root_pathを渡すようにappを渡してくる...しか無いのかな...
    // NOTE: https://qiita.com/fujitayy/items/12a80560a356607da637
    // これ読むと、性能悪い感じで処理しているのかもしれない。
    let mut file = File::open(RESULT_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut results: Vec<(String, usize)> = vec![];
    let mut index = 10;
    // 最後が改行のみ行のはずなので、takeで明示的に10行のみ取る。
    for (idx, line) in contents.split("\n").take(10).enumerate() {
        let res: Vec<String> = line.split_whitespace()
            .map(|x| x.to_string()).collect();
        let size = res.len();
        let key = res[0..(size - 1)].join(" ");
        let value = res[size - 1].parse::<usize>().expect("not a number");
        if *score >= value && index == 10 {
            results.push(("no_name".to_string(), *score));
            index = idx;
        }
        results.push((key, value));
    }

    results
}

pub fn retrieve_achievements() -> Vec<String> {
    if !Path::new(ACHIEVEMENT_PATH).exists() {
        File::create(ACHIEVEMENT_PATH).expect("file create failed");
    }

    let mut file = File::open(ACHIEVEMENT_PATH).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut results: Vec<String> = vec![];
    if contents.len() == 0 { return results }

    for line in contents.split("\n") {
        // rarity, title, achieved_at
        let res: Vec<String> = line.split_whitespace()
            .map(|x| x.to_string()).collect();
        // 最終行が改行のみなので
        if res.len() > 0 { results.push(res[1].clone()); }
    }

    results
}

pub fn update_high_scores_and_min_score(model: &mut Model) {
    let mut results: Vec<(String, usize)> = retrieve_high_scores(
        &model.game_config.score);
    let input_name: String = model.game_config
        .input_field.iter().take(7)
        .map(|x| x.to_string())
        .collect::<Vec<_>>().join("");
    let index = results.iter().position(|x| x.0 == NO_NAME).unwrap();
    results[index].0 = input_name;

    // min_scoreを更新しておく。
    model.game_config.min_score = results[9].1;

    let mut file = File::create(RESULT_PATH).expect("create failed");
    for i in results.iter().take(10) {
        file.write_all(format!("{} {}\n", i.0, i.1)
            .as_bytes())
            .expect("write failed");
    }
}

pub fn handle_input(model: &mut Model, input: Key, max_idx: usize) {
    let mut c_input_field = model.game_config.input_field.clone();
    let mut c_input_cursor = model.game_config.input_cursor;
    match input {
        Key::Left => {
            if c_input_cursor > 0 { c_input_cursor -=  1; }
        },
        Key::Right => {
            if c_input_cursor < max_idx { c_input_cursor += 1; }
        },
        Key::Back => {
            c_input_field[c_input_cursor] = '_';
            if c_input_cursor > 0 { c_input_cursor -=  1; }
        },
        _ => {
            match key_to_char(input) {
                Some(i) => {
                    c_input_field[c_input_cursor] = i;
                    if c_input_cursor < max_idx { c_input_cursor += 1; }
                },
                None => (),
            }
        }
    }

    model.game_config.input_field = c_input_field;
    model.game_config.input_cursor = c_input_cursor;
}

fn save_achievements(model: &Model) {
    let mut file = OpenOptions::new().append(true)
        .open(ACHIEVEMENT_PATH).expect("create failed");
    for i in model.ticker.observer_list.iter().filter(|o| o.notified) {
        file.write_all(format!("{} {} {}\n", i.rarity, i.title, i.achieved_at)
            .as_bytes())
            .expect("write failed");
    }

}

fn key_to_char(key: Key) -> Option<char> {
    match key {
        Key::A => Some('A'),
        Key::B => Some('B'),
        Key::C => Some('C'),
        Key::D => Some('D'),
        Key::E => Some('E'),
        Key::F => Some('F'),
        Key::G => Some('G'),
        Key::H => Some('H'),
        Key::I => Some('I'),
        Key::J => Some('J'),
        Key::K => Some('K'),
        Key::L => Some('L'),
        Key::M => Some('M'),
        Key::N => Some('N'),
        Key::O => Some('O'),
        Key::P => Some('P'),
        Key::Q => Some('Q'),
        Key::R => Some('R'),
        Key::S => Some('S'),
        Key::T => Some('T'),
        Key::U => Some('U'),
        Key::V => Some('V'),
        Key::W => Some('W'),
        Key::X => Some('X'),
        Key::Y => Some('Y'),
        Key::Z => Some('Z'),
        Key::Minus => Some('-'),
        Key::Space => Some(' '),
        Key::Key1 => Some('1'),
        Key::Key2 => Some('2'),
        Key::Key3 => Some('3'),
        Key::Key4 => Some('4'),
        Key::Key5 => Some('5'),
        Key::Key6 => Some('6'),
        Key::Key7 => Some('7'),
        Key::Key8 => Some('8'),
        Key::Key9 => Some('9'),
        Key::Key0 => Some('0'),
        _ => None,
    }
}

pub fn load_stage(model: &mut Model, stage: usize, win: &Rect<DrawScalar>) {
    model.blocks = create_blocks::execute(stage, win);
    model.game_config.stage = stage;
    println!("stage {} loaded...", stage);
}

// nannou::ui::prelude::color::Color::Rgbaから、
// 素のRgbaを導出するための関数
pub fn to_rgba(c: Color) -> Alpha<Rgb<f32>, f32> {
    if let Color::Rgba(r, g, b, a) = c {
        rgba(r, g, b, a)
    } else {
        let color::Rgba(r, g, b, a) = c.to_rgb();
        rgba(r, g, b, a)
    }
}
