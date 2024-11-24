use crate::{
    *,
    CharacterClass::*,
};
use Item::*;
use ItemSlot::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Item {
    BronzeSword,
    IronSword,
    SteelSword
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ItemSlot {
    Weapon,
    Shield,
    Armor
}


impl Item {
    pub fn name(&self) -> &'static str {
        match self {
            BronzeSword => "BRONZE SWORD",
            IronSword => "IRON SWORD",
            SteelSword => "STEEL SWORD"
        }
    }

    pub fn equippable_to(&self, by: &Character, slot: ItemSlot) -> bool {
        match self {
            BronzeSword | IronSword | SteelSword
                => by.class == Warrior && slot == Weapon
        }
    }

    pub fn desc(self) -> &'static [&'static str] {
        match self {
            BronzeSword => &[
                "[1 HAND, WARRIOR REQ.]",
                "GIVES +3 ATK."
            ],
            IronSword => &[
                "[1 HAND, WARRIOR REQ.]",
                "GIVES +5 ATK."
            ],
            SteelSword => &[
                "[1 HAND, WARRIOR REQ.]",
                "GIVES +7 ATK."
            ]
        }
    }
}
