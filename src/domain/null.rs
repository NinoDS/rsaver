use std::fmt::{Display, Formatter};

pub enum NullDomain {
    Top,
    Bot
}

impl Display for NullDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NullDomain::Top => write!(f, "null"),
            NullDomain::Bot => write!(f, "⊥"),
        }
    }
}