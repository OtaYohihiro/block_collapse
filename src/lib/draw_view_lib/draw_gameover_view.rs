use nannou::geom::point::pt2;
use nannou::draw::Draw;
use nannou::geom::rect::Rect;
use nannou::app::DrawScalar;
use nannou::prelude::{ RED, WHITE };

use crate::Model;
use crate::lib::utils::{ retrieve_high_scores, NO_NAME };


pub fn execute(draw: &Draw, _win: &Rect<DrawScalar>, model: &Model) {
    draw_high_scores(draw, &model.game_config.score);
    draw.text("Game Over...")
        .xy(pt2(0.0, -30.0));
    draw.text("Press T to title")
        .xy(pt2(0.0, -80.0));
}

fn draw_high_scores(draw: &Draw, score: &usize) {
    let scores = retrieve_high_scores(score);
    draw.text("HIGH SCORES...").xy(pt2(0.0, 325.0));
    draw.text("Rank").xy(pt2(-150.0, 305.0));
    draw.text("Name").xy(pt2(-100.0, 305.0));
    draw.text("Score").xy(pt2(100.0, 305.0));
    for (idx, s) in scores.iter().enumerate() {
        if idx == 9 { break; }
        draw_high_score(draw, idx, s);
    }
}

fn draw_high_score(draw: &Draw, idx: usize, s: &(String, usize)) {
    let mut draw_color = WHITE;
    if &s.0 == NO_NAME { draw_color = RED }
    draw.text(&(idx + 1).to_string())
        .xy(pt2(-150.0, 280.0 - idx as f32 * 25.0)).color(draw_color);
    draw.text(&s.0)
        .xy(pt2(-100.0, 280.0 - idx as f32 * 25.0)).color(draw_color);
    draw.text(&s.1.to_string())
        .xy(pt2(100.0, 280.0 - idx as f32 * 25.0)).color(draw_color);
}
