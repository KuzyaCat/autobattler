use crate::behavior::range_attack_type::{ RangeAttackType };

#[derive(Debug)]
pub struct Unit {
    pub name: String,
    pub hp: i16,
    pub attack: i16,
    pub armor: i16,
    pub range_attack_type: RangeAttackType,
}

impl Unit {
    pub fn new(
        name: String,
        hp: i16,
        attack: i16,
        armor: i16,
        range_attack_type: RangeAttackType
    ) -> Self {
        Self {
            name,
            hp,
            attack,
            armor,
            range_attack_type,
        }
    }

    pub fn set_hp(&mut self, hp: i16) -> i16 {
        self.hp = hp;
        self.hp
    }
}
