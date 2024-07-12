use std::fmt::{Display, Formatter};
use crate::domain::flat::FlatDomain;

pub type FlatNum = FlatDomain<f64>;

impl Display for FlatNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FlatNum::Top => write!(f, "num"),
            FlatNum::Base(v) => write!(f, "{}", v),
            FlatNum::Bot => write!(f, "⊥"),
        }
    }
}