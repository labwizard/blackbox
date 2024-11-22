use crate::{
    Direction,
    LEVEL_HEIGHT,
    LEVEL_WIDTH,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Position {
    pub fn move_by(&self, dir: Direction, d: isize) -> Self {
        let (dx, dy) = dir.offset();
        let x = (self.x as isize + dx * d)
            .rem_euclid(LEVEL_WIDTH as isize) as usize;
        let y = (self.y as isize + dy * d)
            .rem_euclid(LEVEL_HEIGHT as isize) as usize;
        Self { x, y }
    }

    /// Translates the given position by the vector `(dx, dy)` in the relative
    /// frame where `dir` is forwards (i.e. positive y).
    pub fn translate(&self, dir: Direction, dx: isize, dy: isize) -> Self {
        self.move_by(dir, dy).move_by(dir.right(), dx)
    }
}

impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Position {
            x: value.0,
            y: value.1
        }
    }
}
