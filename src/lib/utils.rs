use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use nannou::app::App;
use nannou::prelude::Key;

use crate::wgpu;
use crate::Model;
use crate::models::win_status::WinStatus;


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
    model.win_status = WinStatus::Normal;
    model.ball.set_initial_state();
    model.player.set_initial_state();
    model.game_config.set_initial_state();
}

pub fn retrieve_high_scores(score: &usize) -> Vec<(String, usize)> {
    // TODO: root_pathを渡すようにappを渡してくる...しか無いのかな...
    let path = Path::new("/Users/ota/project/rust/block_collapse/result.txt");
    let display = path.display();

    //パスを指定してファイルを開く
    let f = match File::open(&path) {
        Err(why) => panic!(
            "couldn't open {}: {}",
            display,
            why
        ),
        Ok(file) => file,
    };

    let reader = BufReader::new(f);
    let mut results: Vec<(String, usize)> = vec![];
    let mut index = 10;
    for (idx, line) in reader.lines().enumerate() {
        let res: Vec<String> = line.unwrap().split_whitespace().map(|x| x.to_string()).collect();
        let value = res[1].parse::<usize>().unwrap();
        let key = res[0].clone();
        if *score >= value && index == 10 {
            results.push(("no_name".to_string(), *score));
            index = idx;
        }
        results.push((key, value));
    }

    results
}

pub fn save_high_scores() {
    // not implemented yet.
}

pub fn handle_input(model: &mut Model, input: Key, max_idx: usize) {
    // - s:左右キーを押すと入力フィールドが左右に移動する
    // - s: カーソルが左端、右端にいる場合はそれ以上横に行けない。
    // - s:delete Keyを押すと入力をスペースにした上で、cursorが一個戻る。
    // - s:何か文字を入力するとカーソル位置に応じてフィールドに文字が保存され、cursorが一個進む。
    // - s:右端で入力をしたら、そこの文字が変わる。
    println!("{:?}", input);
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
