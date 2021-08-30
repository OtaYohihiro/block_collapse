use chrono::Local;

use crate::models::achievement::{ Achievement, ACHIEVEMENTS };
use crate::Model;

pub struct TickerObject {
    pub category: String,
    pub value: usize,
    pub changed: bool
}

impl TickerObject {
    pub fn new(category: String, value: usize, changed: bool) -> TickerObject {
        TickerObject { category, value, changed}
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

    // fn remove_observer(&mut self, idx: &usize) {
    //     self.observer_list.remove(*idx);
    // }
    fn update_observer(&mut self, idx: &usize, app_time: f32) {
        let localtime = Local::now().timestamp();
        self.observer_list[*idx].achieved_at = localtime;
        self.observer_list[*idx].achieved_app_time = app_time;
        self.observer_list[*idx].notified = true;
    }

    pub fn notify_observer(&mut self, model: &Model, app_time: f32) {
        let t_obj = self.state_changed(model);
        let mut updated_observer_idx: Vec<usize> = vec![];
        if t_obj.changed {
            for (idx, o) in self.observer_list.iter().enumerate() {
                if o.update(&t_obj) {
                    updated_observer_idx.push(idx);
                }
            }

            for i in updated_observer_idx.iter().rev() {
                self.update_observer(i, app_time);
            }
        }
    }

    pub fn state_changed(&mut self, model: &Model) -> TickerObject {
        if self.c_score != model.game_config.score {
            self.c_score = model.game_config.score;
            return TickerObject::new(
                "score".to_string(),
                model.game_config.score,
                true
            )
        }
        if self.c_reflect_cnt != model.game_config.reflect_cnt {
            self.c_reflect_cnt = model.game_config.reflect_cnt;
            return TickerObject::new(
                "reflect_cnt".to_string(),
                model.game_config.reflect_cnt,
                true
            )
        }

        return TickerObject::new("".to_string(), 0, false)
    }

    pub fn set_initial_achievements(&mut self) {
        // NOTE: 達成済みのリストは除外する。
        for x in ACHIEVEMENTS.iter() {
            let a = Achievement::new(
                x.0,
                x.1.to_string(),
                x.2.to_string(),
                x.3,
                x.4.to_string(),
                0, // 1970-01-01 0:00:00
                0.0, // app_timeの初期値
                false, // 未通知フラグ
                // &mut ticker
            );
            self.add_observer(a);
        };
    }
}
