/// スタート画面、メニュー画面、プレイ画面、
/// ゲームオーバ画面などをdrawする関数を格納します。
use nannou::app::App;
use nannou::color::*;
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
    let app_time = app.time;
    let win = app.window_rect();

    match model.win_status {
        WinStatus::Title => draw_title_view(&draw, &win, model),
        WinStatus::Normal => draw_normal_view::execute(&draw, &win, model, app_time),
        WinStatus::Menu => draw_menu_view(&draw, &win, model, app_time),
        WinStatus::GameOver => draw_gameover_view::execute(&draw, &win, model),
        WinStatus::RecordBreak => draw_recordbreak_view(app, &draw, &win, model),
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

fn draw_menu_view(
    draw: &Draw,
    win: &Rect<DrawScalar>,
    model: &Model,
    app_time: f32
) {
    draw_normal_view::execute(draw, win, model, app_time);
    let padding = 30.0;
    draw.text("Menu screen")
        .xy(pt2(win.left() + padding * 2.0, win.top() - padding));

    draw.text("Press X to close")
        .xy(pt2(0.0, 0.0));
}

fn draw_recordbreak_view(app: &App, draw: &Draw, win: &Rect<DrawScalar>, model: &Model) {
    draw_gameover_view::execute(&draw, &win, model);
    // 名前入力のviewを表示。
    let color = if app.elapsed_frames() % 24 <= 11 { RED } else { WHITE };
    draw.text("Break a Record!!").font_size(20).color(color)
        .xy(pt2(0.0, 355.0));
    draw.text("Input your name.").font_size(15)
        .xy(pt2(-90.0, 10.0));
    let space: f32 = 10.0;
    for i in 0..7 {
        // NOTE: https://qiita.com/mHALr/items/26dc38154491d302752b を参考に
        let mut buffer = [0u8; 4];
        let ampersand_str: &mut str = model
            .game_config.input_field[i].encode_utf8(&mut buffer);

        if model.game_config.input_cursor == i {
            let letter = if app.elapsed_frames() % 24 <= 11 { "■" } else { ampersand_str };
            draw.text(letter)
                .xy(pt2(-0.0 + space * i as f32, 0.0));
        } else {
            draw.text(ampersand_str)
                .xy(pt2(-0.0 + space * i as f32, 0.0));
        }
    }
}
