use nannou::geom::vector::{ Vector2, vec2 };
// use nannou::geom::point::{ Point2, pt2 };

pub const P_Y: f32 = -280.0;
pub const P_SIZE: f32 = 40.0;
pub const PACE: f32 = 15.0;

pub enum Direction {
    Front,
    Left,
    Right
}

pub struct Player {
    pub p: Vector2,
    pub pace: f32,
    pub r: f32,
    pub dir: Direction,
}

impl Player {
    pub fn new(p: Vector2, pace: f32, r: f32, dir: Direction) -> Player {
        Player {p, pace, r, dir}
    }

    pub fn go(&mut self, direction: i8) {
        self.p.x += self.pace * direction as f32
    }

    pub fn set_initial_state(&mut self) {
        self.p = vec2(0.0, P_Y);
        self.pace = PACE;
        self.r = P_SIZE;
        self.dir = Direction::Front;
    }
}
