use crate::model::CharacterClass::*;

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
