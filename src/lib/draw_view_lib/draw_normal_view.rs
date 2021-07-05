/// プレイ画面のviewをdrawする。
use nannou::draw::Draw;
use nannou::geom::rect::Rect;
use nannou::app::DrawScalar;
use nannou::prelude::RED;
use nannou::geom::point::pt2;
use nannou::geom::vector::vec2;
use nannou::color::rgba;
use chrono::Local;

use crate::Model;
use crate::models::player::Direction;

const TICKER_DURATION: i64 = 3;

pub fn execute(
    draw: &Draw,
    win: &Rect<DrawScalar>,
    model: &Model,
    app_time: f32
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

    // ticker表示
    draw_ticker(draw, win, model, app_time);
}

fn draw_player(draw: &Draw, model: &Model) {
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
}

fn draw_ticker(
    draw: &Draw,
    win: &Rect<DrawScalar>,
    model: &Model,
    app_time: f32
) {
    let localtime = Local::now().timestamp();
    let padding = 30.0;
    for obs in model.ticker.observer_list.iter() {
        let lapse = localtime - obs.achieved_at;
        if lapse <= TICKER_DURATION {
            draw.text(
                &format!("{} ACHIEVED!!", obs.title)
            ).xy(pt2(win.right() - padding * 2.5, win.top() - padding));

            let milsec_lapse = app_time - obs.achieved_app_time;
            draw.rect()
                .xy(pt2(win.right() - padding * 2.0, win.top() - padding))
                .wh(vec2(200.0, 20.0))
                .color(
                    rgba(
                        250.0 / 255.0,
                        250.0 / 255.0,
                        250.0 / 255.0,
                        (milsec_lapse as f32).cos() * 0.5 + 0.5
                    )
                );
        }
    }
}
