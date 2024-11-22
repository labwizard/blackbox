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
    pub base_luck: u32
}

impl Character {
    pub fn atk(&self) -> u32 {
        self.base_atk
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
}
