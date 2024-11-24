use ::ggez::{
    Context,
    GameResult,
    event::EventHandler,
    input::keyboard::KeyInput
};
use ::std::{
    default::Default,
    time::Duration
};
use crate::{
    *,
    GameState::*
};

pub struct Game {
    pub resources: Resources,
    pub state: GameState,
    pub level: Level,
    pub pos: Position,
    pub dir: Direction,
    pub party: Vec<Character>,
    pub inventory: Vec<Item>
}

pub enum GameState {
    Default,
    Exploring {
        anim: Option<ExploreAnimation>,
        selected: Option<usize>
    },
    ViewingCharacter {
        i: usize,
        selected: ItemSlot
    },
    ViewingInventory {
        i: usize,
        requester: Box<GameState>,
        condition: Box<dyn Fn(Item, &Game) -> bool>
    }
}

#[derive(Clone, Debug)]
pub enum ExploreAnimation {
    StepBackward(Duration),
    StepForward(Duration),
    StepLeft(Duration),
    StepRight(Duration)
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut inventory = Vec::new();
        for _ in 0..3 {
            inventory.push(Item::BronzeSword);
            inventory.push(Item::IronSword);
            inventory.push(Item::SteelSword);
        }
        Ok(Game {
            resources: Resources::new(ctx)?,
            state: GameState::Exploring {
                anim: None,
                selected: None
            },
            level: Level::example_level(),
            pos: (0, 0).into(),
            dir: Direction::South,
            party: vec![
                Character {
                    name: "TELLURIUS".to_string(),
                    class: CharacterClass::Warrior,
                    lvl: 1,
                    hp: 9,
                    max_hp: 9,
                    mp: 0,
                    max_mp: 0,
                    base_atk: 6,
                    base_def: 5,
                    base_matk: 0,
                    base_mdef: 0,
                    base_agi: 1,
                    base_luck: 0,
                    weapon: Some(Item::IronSword),
                    shield: None,
                    armor: None
                },
                Character {
                    name: "MERCUTIO".to_string(),
                    class: CharacterClass::Priest,
                    lvl: 1,
                    hp: 8,
                    max_hp: 8,
                    mp: 4,
                    max_mp: 4,
                    base_atk: 4,
                    base_def: 3,
                    base_matk: 4,
                    base_mdef: 3,
                    base_agi: 2,
                    base_luck: 0,
                    weapon: None,
                    shield: None,
                    armor: None
                },
                Character {
                    name: "LEUTHERIA".to_string(),
                    class: CharacterClass::Magician,
                    lvl: 1,
                    hp: 6,
                    max_hp: 6,
                    mp: 7,
                    max_mp: 7,
                    base_atk: 1,
                    base_def: 0,
                    base_matk: 6,
                    base_mdef: 5,
                    base_agi: 3,
                    base_luck: 1,
                    weapon: None,
                    shield: None,
                    armor: None
                }
            ],
            inventory
        })
    }
}

impl EventHandler for Game {
    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        repeated: bool,
    ) -> GameResult {
        match &mut self.state {
            Exploring { .. }
                => exploring::key_down_event(ctx, input, repeated, self),
            ViewingCharacter { .. }
                => viewing_character::key_down_event(ctx, input, repeated, self),
            ViewingInventory { .. }
                => viewing_inventory::key_down_event(ctx, input, repeated, self),
            Default
                => Ok(())
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.state {
            Exploring { .. }
                => exploring::update(ctx, self),
            ViewingCharacter { .. }
                => viewing_character::update(ctx, self),
            ViewingInventory { .. }
                => viewing_inventory::update(ctx, self),
            Default
                => Ok(())
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &mut self.state {
            Exploring { .. }
                => exploring::draw(ctx, &*self),
            ViewingCharacter { .. }
                => viewing_character::draw(ctx, &*self),
            &mut ViewingInventory { .. }
                => viewing_inventory::draw(ctx, &*self),
            Default
                => Ok(())
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Default
    }
}
