use nannou::geom::vector::{Vector2, vec2};
use nannou::geom::point::Point2;

pub enum Direction {
    Front,
    Back,
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
        self.xy += vec2(5.0, 0.0) * direction as f32
    }
}
