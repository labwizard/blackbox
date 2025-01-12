use ::ggez::{
    Context,
    GameResult,
    event::EventHandler,
    input::keyboard::KeyInput
};
use crate::{
    *,
    model::{
        *,
        CharacterClass::*,
        Direction::*,
        Item::*
    },
    scene::{
        *,
        Scene::*
    }
};

pub struct Game {
    pub resources: Resources,
    pub state: State,
    pub scene: Scene,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let mut inventory = Vec::new();
        for _ in 0..3 {
            inventory.push(BronzeSword);
            inventory.push(IronSword);
            inventory.push(SteelSword);
        }
        Ok(Game {
            resources: Resources::new(ctx)?,
            state: State {
                level: Level::example_level(),
                pos: (0, 0).into(),
                dir: South,
                party: vec![
                    Character {
                        name: "TELLURIUS".to_string(),
                        class: Warrior,
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
                        weapon: Some(IronSword),
                        shield: None,
                        armor: None
                    },
                    Character {
                        name: "MERCUTIO".to_string(),
                        class: Priest,
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
                        class: Magician,
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
            },
            scene: Explore(ExploreScene {
                anim: None,
                selected: None
            })
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
        match &self.scene {
            Explore(_)
                => explore::key_down_event(ctx, input, repeated, self),
            ViewCharacter(_)
                => view_character::key_down_event(ctx, input, repeated, self),
            ViewInventory(_)
                => view_inventory::key_down_event(ctx, input, repeated, self),
            Default
                => Ok(())
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        match &self.scene {
            Explore(_)
                => explore::update(ctx, self),
            ViewCharacter(_)
                => view_character::update(ctx, self),
            ViewInventory(_)
                => view_inventory::update(ctx, self),
            Default
                => Ok(())
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        match &self.scene {
            Explore(_)
                => explore::draw(ctx, &*self),
            ViewCharacter(_)
                => view_character::draw(ctx, &*self),
            ViewInventory(_)
                => view_inventory::draw(ctx, &*self),
            Default
                => Ok(())
        }
    }
}
