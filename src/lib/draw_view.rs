/// スタート画面、メニュー画面、プレイ画面、
/// ゲームオーバ画面などをdrawする関数を格納します。
use nannou::app::App;
use nannou::geom::point::pt2;
use nannou::geom::vector::vec2;
use nannou::draw::Draw;
use nannou::geom::rect::Rect;
use nannou::app::DrawScalar;

use crate::Model;
use crate::models::win_status::WinStatus;

use crate::lib::draw_view_lib::{draw_normal_view, draw_gameover_view};

pub fn execute(app: &App, model: &Model) {
    let draw = app.draw();
    let win = app.window_rect();

    match model.win_status {
        WinStatus::Title => draw_title_view(&draw, &win, model),
        WinStatus::Normal => draw_normal_view::execute(&draw, &win, model),
        WinStatus::Menu => draw_menu_view(&draw, &win, model),
        WinStatus::GameOver => draw_gameover_view::execute(&draw, &win, model),
        WinStatus::RecordBreak => draw_recordbreak_view(&draw, &win, model),
    }
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
    draw_normal_view::execute(draw, win, model);
    let padding = 30.0;
    draw.text("Menu screen")
        .xy(pt2(win.left() + padding * 2.0, win.top() - padding));

    draw.text("Press X to close")
        .xy(pt2(0.0, 0.0));
}

fn draw_recordbreak_view(draw: &Draw, win: &Rect<DrawScalar>, model: &Model) {
    draw_gameover_view::execute(&draw, &win, model);
    // 名前入力のviewを表示。
    draw.text("Input your name.")
        .xy(pt2(-150.0, 10.0));
    draw.line().points(pt2(-10.0, 0.0), pt2(10.0, 0.0));
}
