use std::fmt::{Display, Formatter};

enum FlatInt {
    Top,
    Base(i64),
    Bot,
}

impl Display for FlatInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FlatInt::Top => write!(f, "int"),
            FlatInt::Base(v) => write!(f, "{}i", v),
            FlatInt::Bot => write!(f, "⊥"),
        }
    }
}