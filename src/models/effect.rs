use std::collections::HashMap;

use nannou::geom::vector::Vector2;
//use nannou::ui::prelude::color::Rgba;
use nannou::color::Rgba;


pub enum Object {
    Ball,
    Block,
    Player,
    Ticker
}

// https://qiita.com/Kogia_sima/items/6899c5196813cf231054
// &strなのかStringなのかの解決策を提示してくれてる素晴らしい記事。こういう記事
// を書けるようになりたい。

/// 一定秒数かけて描画するEffectのobject。
/// modelに持たせて描画させて、描画時間が終わったら除外する。
// optionsはこんなイメージ。
// color_options = (Rgb(x,x,x), Rgb(x,x,x)) // fromカラー, toカラー
// options = {
//     shape: "ellipse | rect | cross",
//     shape_effect: "fadein | fadeout | keep",
//     color_effect: "fadein | fadeout | keep",
//     easing_type: https://easings.net/ja easeInQuart とかそんな感じ。
// }
pub struct Effect {
    pub time: f32,
    pub elapsed_time: f32, // 来れないとeasing関数使えない。
    pub obj_type: Object,
    pub position: Vector2,
    pub size: Vector2, // width, height
    pub color_options: (Rgba, Rgba), // from_color, to_color
    pub options: HashMap<String, String>
}

impl Effect {
    pub fn new(
        time: f32,
        obj_type: Object,
        position: Vector2,
        size: Vector2, // width, height
        color_options: (Rgba, Rgba),
        options_: Vec<(impl Into<String> + Clone, impl Into<String> + Clone)>
    ) -> Effect {
        let mut options: HashMap<String, String> = HashMap::new();
        for i in options_.iter() {
            options.insert(i.0.clone().into(), i.1.clone().into());
        }

        Effect{
            time,
            elapsed_time: 0.0,
            obj_type,
            position,
            size,
            color_options,
            options
        }
    }

    pub fn shape_effect(&self) -> &String {
        self.options.get("shape_effect").unwrap()
    }

    pub fn shape(&self) -> &String {
        self.options.get("shape").unwrap()
    }

}


