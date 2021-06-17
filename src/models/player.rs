use nannou::geom::vector::{ Vector2, vec2 };
use nannou::geom::point::{ Point2, pt2 };

pub const P_Y: f32 = -280.0;
pub const P_SIZE: f32 = 40.0;

pub enum Direction {
    Front,
    Left,
    Right
}

pub struct Player {
    pub xy: Point2,
    pub wh: Vector2,
    pub dir: Direction,
}

impl Player {
    pub fn new(xy: Point2, wh: Vector2, dir: Direction) -> Player {
        Player {xy, wh, dir}
    }

    pub fn go(&mut self, direction: i8) {
        self.xy += vec2(10.0, 0.0) * direction as f32
    }

    pub fn set_initial_state(&mut self) {
        self.xy = pt2(0.0, P_Y);
        self.wh = vec2(P_SIZE, P_SIZE);
        self.dir = Direction::Front;
    }
}
