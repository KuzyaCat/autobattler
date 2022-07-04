use crate::units::unit::{ Unit };
use crate::behavior::range_attack_type::{ RangeAttackType };

#[test]
fn update_hp() {
    let mut unit = Unit::new(
        String::from("Melee creep"),
        5,
        1,
        0,
        RangeAttackType::Melee,
    );
    unit.set_hp(unit.hp - 2);
    assert_eq!(unit.hp, 3);
}
