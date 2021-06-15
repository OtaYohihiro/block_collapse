/// スタート画面、メニュー画面、プレイ画面、
/// ゲームオーバ画面などをdrawする関数を格納します。

use nannou::app::App;
use nannou::prelude::RED;
use nannou::geom::point::pt2;

use crate::Model;
use crate::models::player::Direction;

/// プレイ画面のviewをdrawする。
pub fn draw_playing_view(app: &App, model: &Model) {
    let draw = app.draw();

    draw.ellipse().xy(model.ball.p).radius(ball.r).color(RED);
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
