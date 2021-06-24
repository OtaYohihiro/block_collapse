use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use nannou::app::App;
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
