use std::fmt::{Display, Formatter};
use crate::domain::flat::FlatDomain;

pub type FlatBigInt = FlatDomain<i128>;

impl Display for FlatBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FlatBigInt::Top => write!(f, "bigint"),
            FlatBigInt::Base(v) => write!(f, "{}n", v),
            FlatBigInt::Bot => write!(f, "⊥"),
        }
    }
}