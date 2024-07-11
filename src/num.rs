use std::fmt::{Display, Formatter};

enum FlatNum {
    Top,
    Base(f64),
    Bot,
}

impl Display for FlatNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FlatNum::Top => write!(f, "num"),
            FlatNum::Base(v) => write!(f, "{}", v),
            FlatNum::Bot => write!(f, "⊥"),
        }
    }
}