use nannou::app::App;
use nannou::geom::vector::{ Vector2, vec2 };
use nannou::app::DrawScalar;

use crate::Model;
use crate::models::player::Player;

pub const INIT_X: f32 = 60.0;
pub const INIT_Y: f32 = 120.0;
pub const INIT_R: f32 = 10.0;
const CONTACT_DURATION: f32 = 12.0; // 12frameなので0.2sで一旦。

#[derive(Clone)]
pub enum BallStatus {
  Normal,
  Failed,
}

// latest_contact_at: playerとの最終接触時間をframeRateで記録しておく。
#[derive(Clone)]
pub struct Ball {
    pub p: Vector2,
    pub v: Vector2,
    pub r: f32,
    pub latest_contact_at: DrawScalar, // f32. https://docs.rs/nannou/0.16.0/nannou/app/type.DrawScalar.html
    pub status: BallStatus,
}

impl Ball {
    pub fn new(
        p: Vector2,
        v: Vector2,
        r: f32,
        latest_contact_at: DrawScalar,
        status: BallStatus,
    ) -> Ball {
      Ball {p, v, r, latest_contact_at, status}
    }

    pub fn go(&mut self) {
        self.p += self.v / 60.0
    }

    /// 反射のしたらtrueを返し、ballの向きを返る。
    pub fn reflect(&mut self, app: &App, player: &Player) -> bool {
        // 壁との当たり判定
        let win = app.window_rect();

        if self.p.x <= win.left() && self.v.x <= 0.0 {
            self.v.x *= -1.0;
            return true
        }
        if win.right() <= self.p.x && self.v.x >= 0.0 {
            self.v.x *= -1.0;
            return true
        }

        if win.top() <= self.p.y && self.v.y >= 0.0 {
            self.v.y *= -1.0;
            return true
        }
        // 下に行ったらそもそも失敗になる。
        if self.p.y <= win.bottom() {
            // self.v.y *= -1.0;
            self.status = BallStatus::Failed;
            return true
        }

        // playerとの当たり判定
        //// ballとplayerの距離
        let dist = (
            (self.p[0] - player.xy[0]).powf(2.0) +
                (self.p[1] - player.xy[1]).powf(2.0)
        ).powf(0.5);
        //// 半径同士のsum
        let r_sum = self.r + player.wh[0];
        let duration = app.time - self.latest_contact_at;

        // NOTE: 接触判定ちょい甘いが一旦. playerの範囲が微妙だな。
        if dist <= r_sum && duration >= CONTACT_DURATION {
            self.latest_contact_at = app.time;
            self.v.y *= -1.0;
            return true;
        }

        return false
    }

    pub fn reflect_sound(self, app: &App, model: &mut Model) {
        model.game_config.score += 10;

        let assets = app.assets_path().unwrap();
        let path = assets.join("sounds").join("反射音.wav");
        let sound = audrey::open(path).expect("failed to load sound");
        model.stream
            .send(move |audio| { audio.sounds.push(sound) })
            .ok();
    }

    pub fn madly_speed_up(&mut self) {
        println!("speed up triggered! madly spped_up.");
        self.v.x *= 3.;
        self.v.y *= 2.5;
    }

    pub fn quite_speed_down(&mut self) {
        println!("speed down triggered! madly spped_down.");
        self.v.x /= 4.;
        self.v.y /= 3.5;
    }

    pub fn set_initial_state(&mut self) {
        println!("initial set up.");
        self.v = vec2(INIT_X, INIT_Y);
        self.r = INIT_R;
        self.p = vec2(0.0, 0.0);
        self.status = BallStatus::Normal;
    }
}
