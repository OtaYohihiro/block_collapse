/// スタート画面、メニュー画面、プレイ画面、
/// ゲームオーバ画面などをdrawする関数を格納します。
use nannou::app::App;
use nannou::prelude::RED;
use nannou::geom::point::pt2;
use nannou::geom::vector::vec2;
use nannou::draw::Draw;
use nannou::geom::rect::Rect;
use nannou::app::DrawScalar;

use crate::Model;
use crate::models::player::Direction;
use crate::models::win_status::WinStatus;

use crate::lib::draw_view_lib::draw_gameover_view;

pub fn execute(app: &App, model: &Model) {
    let draw = app.draw();
    let win = app.window_rect();

    match model.win_status {
        WinStatus::Title => draw_title_view(&draw, &win, model),
        WinStatus::Normal => draw_normal_view(&draw, &win, model),
        WinStatus::GameOver => draw_gameover_view::execute(&draw, &win, model),
        WinStatus::Menu => draw_menu_view(&draw, &win, model),
    }
}

/// プレイ画面のviewをdrawする。
fn draw_normal_view(draw: &Draw, win: &Rect<DrawScalar>, model: &Model) {

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

fn draw_title_view(draw: &Draw, _win: &Rect<DrawScalar>, model: &Model) {
    draw.texture(
        model.textures
            .get("title").unwrap()
            .get("title").unwrap()
    ).xy(pt2(0.0, 0.0)).wh(vec2(400.0, 150.0));

    draw.text("Press S to start")
        .xy(pt2(0.0, 150.0));
}

fn draw_menu_view(draw: &Draw, win: &Rect<DrawScalar>, model: &Model) {
    draw_normal_view(draw, win, model);
    let padding = 30.0;
    draw.text("Menu screen")
        .xy(pt2(win.left() + padding * 2.0, win.top() - padding));

    draw.text("Press X to close")
        .xy(pt2(0.0, 0.0));
}
