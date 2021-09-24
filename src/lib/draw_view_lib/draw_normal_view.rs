/// プレイ画面のviewをdrawする。
use nannou::draw::Draw;
use nannou::geom::rect::Rect;
use nannou::app::DrawScalar;
use nannou::prelude::RED;
use nannou::geom::point::pt2;
use nannou::geom::vector::vec2;

use crate::Model;
use crate::models::player::Direction;

pub fn execute(
    draw: &Draw,
    win: &Rect<DrawScalar>,
    model: &Model,
) {
    // ball描画
    draw.ellipse().xy(model.ball.p).radius(model.ball.r).color(RED);
    // player描画
    draw_player(draw, model);

    // block描画
    for block in model.blocks.iter() {
        draw.ellipse().xy(block.p).radius(block.r).color(block.color());
    }

    // score表示
    let padding = 30.0;
    draw.text(&format!("score: {}", model.game_config.score))
        .xy(pt2(win.left() + padding * 2.0, win.top() - padding));
}

fn draw_player(draw: &Draw, model: &Model) {
    let x = model.player.p.x;
    let y = model.player.p.y;
    let size = model.player.r;
    match model.player.dir {
        Direction::Left => {
            draw.texture(
                model.textures.get("player").unwrap().get("l_run").unwrap()
            ).xy(pt2(x, y)).wh(vec2(size, size));
        },
        Direction::Right => {
            draw.texture(
                model.textures.get("player").unwrap().get("r_run").unwrap()
            ).xy(pt2(x, y)).wh(vec2(size, size));
        },
        _ => {
            draw.texture(
                model.textures.get("player").unwrap().get("normal").unwrap()
            ).xy(pt2(x, y)).wh(vec2(size, size));
        }
    }
}
