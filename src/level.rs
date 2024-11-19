use crate::{
    Direction::{self, *},
    Position
};

pub const LEVEL_WIDTH: usize = 20;
pub const LEVEL_HEIGHT: usize = 20;

#[derive(Debug)]
pub struct Level {
    pub horiz_walls: [[Wall; LEVEL_HEIGHT + 1]; LEVEL_WIDTH],
    pub vert_walls: [[Wall; LEVEL_HEIGHT]; LEVEL_WIDTH + 1]
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Wall {
    None,
    Some,
    Door
}

impl Level {
    pub fn wall_towards(&self, pos: Position, dir: Direction) -> Wall {
        match dir {
            East => self.vert_walls[pos.x + 1][pos.y],
            South => self.horiz_walls[pos.x][pos.y + 1],
            West => self.vert_walls[pos.x][pos.y],
            North => self.horiz_walls[pos.x][pos.y]
        }
    }
}

impl Wall {
    pub fn is_passable(&self) -> bool {
        match self {
            Wall::None | Wall::Door => true,
            Wall::Some => false
        }
    }
}

impl Level {
    pub fn example_level() -> Self {
        let mut horiz_walls = [[Wall::None; LEVEL_HEIGHT + 1]; LEVEL_WIDTH];
        for y in [0, LEVEL_HEIGHT] {
            for x in 0..LEVEL_WIDTH {
                horiz_walls[x][y] = Wall::Some;
            }
        }
        let mut vert_walls = [[Wall::None; LEVEL_HEIGHT]; LEVEL_WIDTH + 1];
        for x in [0, LEVEL_WIDTH] {
            for y in 0..LEVEL_HEIGHT {
                vert_walls[x][y] = Wall::Some;
            }
        }
        vert_walls[4][4] = Wall::Door;
        Level {
            horiz_walls,
            vert_walls
        }
    }
}
