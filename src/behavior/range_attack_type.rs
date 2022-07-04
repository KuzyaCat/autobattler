use std::fmt;

pub enum RangeAttackType {
    Melee,
    Range,
}

impl fmt::Debug for RangeAttackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
