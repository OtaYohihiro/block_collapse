use nannou::app::App;
use nannou::color::*;
use nannou::prelude::rgb::Rgb;
use nannou::color::encoding::Srgb;
use nannou::geom::vector::Vector2;
use crate::models::ball::Ball;

pub struct Block {
    pub p: Vector2,
    pub v: Vector2,
    pub r: f32,
    pub life: u8, // ブロックの強度
}

impl Block {
    pub fn new(p: Vector2, v: Vector2, r: f32, life: u8) -> Block {
        Block{p, v, r, life}
    }

    pub fn b_color(&self) -> Rgb<Srgb, u8> {
        match &self.life {
            &3 => return FIREBRICK,
            &2 => return MEDIUMTURQUOISE,
            &1 => return CYAN,
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

        if dist <= r_sum {
            ball.v.y *= -1.0;
            return true;
        }

        return false
    }
}




