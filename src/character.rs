use crate::{
    *,
    Item::*,
    ItemSlot::*
};
use CharacterClass::*;

#[derive(Clone, Debug)]
pub struct Character {
    pub name: String,
    pub class: CharacterClass,
    pub lvl: u32,
    pub hp: u32,
    pub max_hp: u32,
    pub mp: u32,
    pub max_mp: u32,
    pub base_atk: u32,
    pub base_def: u32,
    pub base_matk: u32,
    pub base_mdef: u32,
    pub base_agi: u32,
    pub base_luck: u32,
    pub weapon: Option<Item>,
    pub shield: Option<Item>,
    pub armor: Option<Item>
}

impl Character {
    pub fn atk(&self) -> u32 {
        let mut atk = self.base_atk;
        match self.weapon {
            Some(BronzeSword) => atk += 3,
            Some(IronSword) => atk += 5,
            Some(SteelSword) => atk += 7,
            _ => {}
        }
        atk
    }

    pub fn def(&self) -> u32 {
        self.base_def
    }

    pub fn matk(&self) -> u32 {
        self.base_matk
    }

    pub fn mdef(&self) -> u32 {
        self.base_mdef
    }

    pub fn agi(&self) -> u32 {
        self.base_agi
    }

    pub fn luck(&self) -> u32 {
        self.base_luck
    }

    pub fn item(&self, slot: ItemSlot) -> Option<Item> {
        match slot {
            Weapon => self.weapon,
            Shield => self.shield,
            Armor => self.armor
        }
    }

    pub fn item_mut(&mut self, slot: ItemSlot) -> &mut Option<Item> {
        match slot {
            Weapon => &mut self.weapon,
            Shield => &mut self.shield,
            Armor => &mut self.armor
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CharacterClass {
    Warrior,
    Priest,
    Magician
}

impl CharacterClass {
    pub fn name(&self) -> &'static str {
        match self {
            Warrior => "WARRIOR",
            Priest => "PRIEST",
            Magician => "MAGICIAN"
        }
    }

    pub fn start_base_atk(&self) -> u32 {
        match self {
            Warrior => 2,
            Priest => 0,
            Magician => 0
        }
    }

    pub fn start_base_def(&self) -> u32 {
        match self {
            Warrior => 1,
            Priest => 0,
            Magician => 0
        }
    }

    pub fn start_base_matk(&self) -> u32 {
        match self {
            Warrior => 0,
            Priest => 1,
            Magician => 3
        }
    }

    pub fn start_base_mdef(&self) -> u32 {
        match self {
            Warrior => 1,
            Priest => 0,
            Magician => 0
        }
    }
}
