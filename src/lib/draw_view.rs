/// スタート画面、メニュー画面、プレイ画面、
/// ゲームオーバ画面などをdrawする関数を格納します。

use nannou::app::App;
use nannou::prelude::RED;
use nannou::geom::point::pt2;

use crate::Model;
use crate::models::player::Direction;
use crate::models::win_status::WinStatus;

pub fn execute(app: &App, model: &Model) {
    match model.win_status {
        WinStatus::Title => draw_title_view(app, model),
        WinStatus::Normal => draw_normal_view(app, model),
        WinStatus::GameOver => draw_gameover_view(app, model),
        _ => (),
    }
}

/// プレイ画面のviewをdrawする。
fn draw_normal_view(app: &App, model: &Model) {
    let draw = app.draw();

    draw.ellipse().xy(model.ball.p).radius(model.ball.r).color(RED);
    match model.player.dir {
        Direction::Left => {
            draw.texture(
                model.textures
                    .get(&"player".to_string()).unwrap()
                    .get(&"l_run".to_string()).unwrap()
            ).xy(model.player.xy).wh(model.player.wh);
        },
        Direction::Right => {
            draw.texture(
                model.textures
                    .get(&"player".to_string()).unwrap()
                    .get(&"r_run".to_string()).unwrap()
            ).xy(model.player.xy).wh(model.player.wh);
        },
        _ => {
            draw.texture(
                model.textures
                    .get(&"player".to_string()).unwrap()
                    .get(&"normal".to_string()).unwrap()
            ).xy(model.player.xy).wh(model.player.wh);
        }
    }

    // score表示
    let win = app.window_rect();
    let padding = 30.0;
    draw.text(&format!("score: {}", model.game_config.score))
        .xy(pt2(win.right() - padding * 2.0, win.top() - padding));
}

fn draw_gameover_view(app: &App, _model: &Model) {
    let draw = app.draw();

    let win = app.window_rect();
    let padding = 30.0;
    draw.text("Game Over...")
        .xy(pt2(win.right() - padding * 2.0, win.top() - padding));
    draw.text("Press T to title")
        .xy(pt2(0.0, 0.0));
}

fn draw_title_view(app: &App, _model: &Model) {
    let draw = app.draw();

    let win = app.window_rect();
    let padding = 30.0;
    draw.text("Title Screen")
        .xy(pt2(win.right() - padding * 2.0, win.top() - padding));

    draw.text("Press S to start")
        .xy(pt2(0.0, 0.0));
}
