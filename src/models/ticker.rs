use chrono::Local;

use nannou::geom::point::pt2;
use nannou::geom::vector::vec2;
use nannou::geom::rect::Rect;
use nannou::app::DrawScalar;
use nannou::ui::color::WHITE;

use crate::models::achievement::{ Achievement, ACHIEVEMENTS };
use crate::models::effect::{ Effect, Object };
use crate::Model;
use crate::lib::utils::{ retrieve_achievements, to_rgba };
use crate::lib::utils_for_effect::add_effect;
use crate::lib::draw_view_lib::draw_effect_view::PADDING;

pub struct TickerObject {
    pub category: String,
    pub value: usize,
    pub changed: bool
}

impl TickerObject {
    pub fn new(category: impl Into<String>, value: usize, changed: bool) -> TickerObject {
        TickerObject { category: category.into(), value, changed}
    }
}

#[derive(Clone)]
pub struct Ticker {
    pub observer_list: Vec<Achievement>,
    pub c_score: usize,
    pub c_reflect_cnt: usize,
}


impl Ticker {
    pub fn new(
        observer_list: Vec<Achievement>,
        c_score: usize,
        c_reflect_cnt: usize
    ) -> Ticker {
        Ticker { observer_list, c_score, c_reflect_cnt }
    }

    pub fn add_observer(&mut self, achievement: Achievement) {
        self.observer_list.push(achievement);
    }

    fn update_observer(&mut self, idx: &usize) {
        let localtime = Local::now().timestamp();
        self.observer_list[*idx].achieved_at = localtime;
        self.observer_list[*idx].notified = true;
    }

    pub fn notify_observer(&mut self, model: &mut Model, win: &Rect<DrawScalar>) {
        let t_obj = self.state_changed(model);
        let mut updated_observer_idx: Vec<usize> = vec![];
        if t_obj.changed {
            for (idx, o) in self.observer_list.iter().enumerate() {
                if o.update(&t_obj) {
                    let effect = Effect::new(
                        2.0,
                        Object::Ticker,
                        pt2(win.right() - PADDING * 2.0, win.top() - PADDING),
                        vec2(200.0, 20.0),
                        (to_rgba(WHITE), to_rgba(WHITE)),
                        vec![
                            ("shape_effect", "keep"),
                            ("color_effect", "fadeout"),
                            ("shape", "rect_ticker"),
                            ("msg", &o.notify_msg())
                        ]
                    );
                    add_effect(model, effect);

                    updated_observer_idx.push(idx);
                }
            }

            for i in updated_observer_idx.iter().rev() {
                self.update_observer(i);
            }
        }
    }

    pub fn state_changed(&mut self, model: &Model) -> TickerObject {
        if self.c_score != model.game_config.score {
            self.c_score = model.game_config.score;
            return TickerObject::new(
                "score",
                model.game_config.score,
                true
            )
        }
        if self.c_reflect_cnt != model.game_config.reflect_cnt {
            self.c_reflect_cnt = model.game_config.reflect_cnt;
            return TickerObject::new(
                "reflect_cnt",
                model.game_config.reflect_cnt,
                true
            )
        }

        return TickerObject::new("", 0, false)
    }

    pub fn set_initial_achievements(&mut self) {
        // 達成済みのリストは除外する。
        let achieved = retrieve_achievements();
        for x in ACHIEVEMENTS.iter() {
            match achieved.iter().find(|&a| *a == x.2.to_string()) {
                Some(_) => (),
                None => {
                    let a = Achievement::new(
                        x.0,
                        x.1,
                        x.2,
                        x.3,
                        x.4,
                        0, // 1970-01-01 0:00:00
                        false, // 未通知フラグ
                        // &mut ticker
                    );
                    self.add_observer(a);
                },
            }
        };
    }
}
