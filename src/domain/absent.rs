use std::fmt::{Display, Formatter};

pub enum AbsentDomain {
    Top,
    Bot
}

impl Display for AbsentDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AbsentDomain::Top => write!(f, "absent"),
            AbsentDomain::Bot => write!(f, "⊥"),
        }
    }
}