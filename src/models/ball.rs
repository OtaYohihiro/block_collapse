use nannou::app::App;
use nannou::geom::vector::{ Vector2, vec2 };
use crate::Model;
use crate::models::player::Player;

pub const INIT_X: f32 = 60.0;
pub const INIT_Y: f32 = 120.0;
pub const INIT_R: f32 = 10.0;
#[derive(Clone)]
pub struct Ball {
    pub p: Vector2,
    pub v: Vector2,
    pub r: f32,
}

impl Ball {
    pub fn new(p: Vector2, v: Vector2, r: f32) -> Ball {
      Ball {p, v, r}
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

        if self.p.y <= win.bottom() && self.v.y <= 0.0 {
            self.v.y *= -1.0;
            return true
        }
        if win.top() <= self.p.y && self.v.y >= 0.0 {
            self.v.y *= -1.0;
            return true
        }

        // playerとの当たり判定

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

    pub fn set_initial_speed(&mut self) {
        println!("initial speed set up.");
        self.v = vec2(INIT_X, INIT_Y);
    }
}
