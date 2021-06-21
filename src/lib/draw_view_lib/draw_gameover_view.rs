use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;

use nannou::geom::point::pt2;
use nannou::draw::Draw;
use nannou::geom::rect::Rect;
use nannou::app::DrawScalar;

use crate::Model;

const NO_NAME: &str = "no_name";

pub fn execute(draw: &Draw, _win: &Rect<DrawScalar>, model: &Model) {
    draw_high_scores(draw, &model.game_config.score);
    draw.text("Game Over...")
        .xy(pt2(0.0, -20.0));
    draw.text("Press T to title")
        .xy(pt2(0.0, -70.0));
}

fn draw_high_scores(draw: &Draw, score: &u16) {
    let scores = retrieve_high_scores(score);
    draw.text("HIGH SCORES...").xy(pt2(0.0, 325.0));
    draw.text("Rank").xy(pt2(-150.0, 305.0));
    draw.text("Name").xy(pt2(-100.0, 305.0));
    draw.text("Score").xy(pt2(100.0, 305.0));
    for (idx, s) in scores.iter().enumerate() {
        if idx == 9 { break; }
        if &s.0 == NO_NAME {
            draw.text("Break a Record!!")
                .xy(pt2(0.0, 355.0))
                .font_size(20);
        }
        draw.text(&(idx + 1).to_string()).xy(pt2(-150.0, 280.0 - idx as f32 * 25.0));
        draw.text(&s.0).xy(pt2(-100.0, 280.0 - idx as f32 * 25.0));
        draw.text(&s.1.to_string()).xy(pt2(100.0, 280.0 - idx as f32 * 25.0));
    }
}

fn retrieve_high_scores(score: &u16) -> Vec<(String, u16)> {
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
    let mut results: Vec<(String, u16)> = vec![];
    let mut index = 10;
    for (idx, line) in reader.lines().enumerate() {
        let res: Vec<String> = line.unwrap().split_whitespace().map(|x| x.to_string()).collect();
        let value = res[1].parse::<u16>().unwrap();
        let key = res[0].clone();
        if *score >= value && index == 10 {
            results.push(("no_name".to_string(), *score));
            index = idx;
        }
        results.push((key, value));
    }

    results
}
