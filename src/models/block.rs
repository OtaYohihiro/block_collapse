use nannou::app::App;
use nannou::color::*;
use nannou::prelude::rgb::Rgb;
use nannou::color::encoding::Srgb;
use nannou::geom::vector::Vector2;

use crate::models::ball::Ball;
use crate::Model;

pub const BLOCK_SIZE: f32 = 15.0;
pub const MAX_B_NUM: u16 = 1000;
const CONTACT_DURATION: f32 = 0.15; // frameかと思いきや、秒数だった。

#[derive(Clone)]
pub struct Block {
    pub p: Vector2,
    pub r: f32,
    pub life: u8, // ブロックの強度
}

impl Block {
    pub fn new(p: Vector2, r: f32, life: u8) -> Block {
        Block{p, r, life}
    }

    pub fn color(&self) -> Rgb<Srgb, u8> {
        match &self.life {
            &3 => return CYAN,
            &2 => return GOLD,
            &1 => return FIREBRICK,
            _ => return GHOSTWHITE,
        }
    }

    pub fn reflect(&mut self, app: &App, ball: &mut Ball) -> bool {
        // ballとの当たり判定
        //// ballとplayerの距離
        let dist = (
            (self.p[0] - ball.p[0]).powf(2.0) +
                (self.p[1] - ball.p[1]).powf(2.0)
        ).powf(0.5);
        //// 半径同士のsum
        let r_sum = self.r + ball.r;
        let duration = app.time - ball.latest_contact_at;

        if dist <= r_sum && duration >= CONTACT_DURATION {
            ball.v.y *= -1.0;
            ball.latest_contact_at = app.time;
            self.life_minus();
            return true;
        }

        return false
    }

    pub fn reflect_sound(&self, app: &App, model: &mut Model) {
        model.game_config.score += 10000;

        let assets = app.assets_path().unwrap();
        let path = assets.join("sounds").join("反射音.wav");
        let sound = audrey::open(path).expect("failed to load sound");
        model.stream
            .send(move |audio| { audio.sounds.push(sound) })
            .ok();
    }


    fn life_minus(&mut self) {
        self.life -= 1;
    }
}

