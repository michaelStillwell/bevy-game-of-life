pub struct Vec2 {
    pub x: i8,
    pub y: i8,
}

impl Vec2 {
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
}

pub struct Bounds2 {
    pub position: Vec2,
    pub size: Vec2,
}

impl Bounds2 {
    pub fn in_bounds(&self, coords: Vec2) -> bool {
        coords.x >= self.position.x
            && coords.y >= self.position.y
            && coords.x <= self.position.x + self.size.x
            && coords.y <= self.position.y + self.size.y
    }
}
