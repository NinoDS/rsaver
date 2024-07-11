use std::fmt::{Display, Formatter};

enum FlatBigInt {
    Top,
    Base(i128), // :P
    Bot,
}

impl Display for FlatBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FlatBigInt::Top => write!(f, "bigint"),
            FlatBigInt::Base(v) => write!(f, "{}n", v),
            FlatBigInt::Bot => write!(f, "⊥"),
        }
    }
}