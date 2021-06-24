/// プレイ画面のviewをdrawする。
use nannou::draw::Draw;
use nannou::geom::rect::Rect;
use nannou::app::DrawScalar;
use nannou::prelude::RED;
use nannou::geom::point::pt2;

use crate::Model;
use crate::models::player::Direction;


pub fn execute(draw: &Draw, win: &Rect<DrawScalar>, model: &Model) {

    // ball描画
    draw.ellipse().xy(model.ball.p).radius(model.ball.r).color(RED);
    // player描画
    match model.player.dir {
        Direction::Left => {
            draw.texture(
                model.textures.get("player").unwrap().get("l_run").unwrap()
            ).xy(model.player.xy).wh(model.player.wh);
        },
        Direction::Right => {
            draw.texture(
                model.textures.get("player").unwrap().get("r_run").unwrap()
            ).xy(model.player.xy).wh(model.player.wh);
        },
        _ => {
            draw.texture(
                model.textures.get("player").unwrap().get("normal").unwrap()
            ).xy(model.player.xy).wh(model.player.wh);
        }
    }
    // block描画
    for block in model.blocks.iter() {
        draw.ellipse().xy(block.p).radius(block.r).color(block.color());
    }

    // score表示
    let padding = 30.0;
    draw.text(&format!("score: {}", model.game_config.score))
        .xy(pt2(win.right() - padding * 2.0, win.top() - padding));
}

