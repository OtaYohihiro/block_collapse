use once_cell::sync::Lazy;

use nannou::draw::Draw;
use nannou::geom::rect::Rect;
use nannou::app::DrawScalar;
use nannou::geom::point::{Point2, pt2};
use nannou::geom::vector::vec2;

use crate::Model;

const SPAN: f32 = 20.0;
const CROSS_POS: Lazy<[Point2; 4]> = Lazy::new(|| [
    pt2(1.0, 0.0),
    pt2(-1.0, 0.0),
    pt2(0.0, 1.0),
    pt2(0.0, -1.0)
]);


// TODO: fadeout処理諸々が実装されてないので処理を入れること。easing関数も入れてない。
pub fn execute(draw: &Draw, _win: &Rect<DrawScalar>, model: &Model) {
    for effect in model.effect_vec.iter() {
        if effect.shape() == "cross" && effect.shape_effect() == "fadeout" {
            for i in CROSS_POS.iter() {
                draw.rect()
                    .xy(effect.position + *i * SPAN)
                    .wh(vec2(10.0, 10.0))
                    .color(effect.color_options.0);
            }
        }
    }
}


// fadein/outなら、from_colorとto_color渡してそれをtimeで按分する感じにしたい。
//   easing関数は一旦固定で。
// crossなら上下左右に0.5sごとに遠のいていく■を表示する。

