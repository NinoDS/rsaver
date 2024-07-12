use std::fmt::{Display, Formatter};
use crate::domain::flat::FlatDomain;

type FlatBool = FlatDomain<bool>;

impl Display for FlatBool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FlatBool::Top => write!(f, "bool"),
            FlatBool::Base(v) => write!(f, "{}", v),
            FlatBool::Bot => write!(f, "⊥"),
        }
    }
}

impl FlatBool {}
