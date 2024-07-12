use std::fmt::{Display, Formatter};

pub enum UndefDomain {
    Top,
    Bot
}

impl Display for UndefDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UndefDomain::Top => write!(f, "undefined"),
            UndefDomain::Bot => write!(f, "⊥"),
        }
    }
}