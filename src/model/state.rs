use crate::model::*;

#[derive(Clone, Debug)]
pub struct State {
    pub level: Level,
    pub pos: Position,
    pub dir: Direction,
    pub party: Vec<Character>,
    pub inventory: Vec<Item>
}
