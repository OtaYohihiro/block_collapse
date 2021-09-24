use once_cell::sync::Lazy;

use nannou::draw::Draw;
use nannou::geom::rect::Rect;
use nannou::app::DrawScalar;
use nannou::geom::point::{Point2, pt2};
use nannou::geom::vector::vec2;

use crate::Model;
use crate::models::effect::Effect;

const SPAN: f32 = 10.0;
const CROSS_POS: Lazy<[Point2; 4]> = Lazy::new(|| [
    pt2(1.0, 0.0),
    pt2(-1.0, 0.0),
    pt2(0.0, 1.0),
    pt2(0.0, -1.0)
]);
const EFFECT_TIME_UNIT: f32 = 0.25;
pub const PADDING: f32 = 30.0;


pub fn execute(draw: &Draw, _win: &Rect<DrawScalar>, model: &Model) {
    for effect in model.effect_vec.iter() {
        match effect.shape() {
            "cross" => draw_cross_effect(draw, effect),
            "rect_ticker" => draw_rect_ticker_effect(draw, effect),
            _ => (),
        }
    }
}

fn draw_cross_effect(draw: &Draw, effect: &Effect) {
    // TODO: fadein | out | keepでわけないとな。
    if effect.shape_effect() == "fadeout" {
        let max = (effect.time / EFFECT_TIME_UNIT).ceil();
        let idx = (effect.elapsed_time / EFFECT_TIME_UNIT).ceil();

        for i in CROSS_POS.iter() {
            if idx == 1.0 {
                draw.rect()
                    .xy(effect.position + *i * SPAN * idx)
                    .wh(vec2(5.0, 5.0))
                    .color(effect.color_options.0);
            } else if idx == max {
                let mut rgba = effect.color_options.0;
                rgba.alpha = 0.1;

                draw.rect()
                    .xy(effect.position + *i * SPAN * (max - 1.0))
                    .wh(vec2(5.0, 5.0))
                    .color(rgba);
            } else {
                let mut rgba = effect.color_options.0;
                rgba.alpha = 0.55;

                draw.rect()
                    .xy(effect.position + *i * SPAN * (idx - 1.0) )
                    .wh(vec2(5.0, 5.0))
                    .color(rgba);
                draw.rect()
                    .xy(effect.position + *i * SPAN * idx)
                    .wh(vec2(5.0, 5.0))
                    .color(rgba);
            }
        }
    }
}

// NOTE: もう少し装飾的なのをつけたいような。
fn draw_rect_ticker_effect(draw: &Draw, effect: &Effect) {
    if effect.shape_effect() == "keep" {
        draw.text(effect.options.get("msg").unwrap()).xy(effect.position);

        let mut rgba = effect.color_options.0;
        rgba.alpha = (effect.elapsed_time + 0.25).cos() * 0.5 + 0.5;
        draw.rect()
            .xy(effect.position)
            .wh(vec2(200.0, 20.0))
            .color(rgba);
    }
}
