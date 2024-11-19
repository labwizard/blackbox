use Direction::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    East,
    South,
    West,
    North
}

impl Direction {
    pub fn offset(&self) -> (isize, isize) {
        match self {
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
            North => (0, -1)
        }
    }

    pub fn left(&self) -> Self {
        match self {
            East => North,
            South => East,
            West => South,
            North => West
        }
    }

    pub fn right(&self) -> Self {
        match self {
            East => South,
            South => West,
            West => North,
            North => East
        }
    }

    pub fn rev(&self) -> Self {
        match self {
            East => West,
            South => North,
            West => East,
            North => South
        }
    }
}
