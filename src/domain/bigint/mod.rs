use std::fmt::{Display, Formatter};
use crate::domain::flat::FlatDomain;
use crate::domain::flat::FlatDomain::{Base, Bot, Top};

pub type FlatBigInt = FlatDomain<i128>; // :P

impl Display for FlatBigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Top => write!(f, "bigint"),
            Base(v) => write!(f, "{}n", v),
            Bot => write!(f, "⊥"),
        }
    }
}

impl FlatBigInt {
    // partial order
    fn leq(&self, other: &Self) -> bool {
        match (self, other) {
            (Bot, _) | (_, Top) => true,
            (_, Bot) | (Top, _) => false,
            (Base(l), Base(r)) => l == r
        }
    }

    // join operator
    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (Bot, _) | (_, Top) => other.clone(),
            (_, Bot) | (Top, _) => self.clone(),
            (Base(l), Base(r)) => if l == r { self.clone() } else { Top }
        }
    }

    // meet operator
    fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (Bot, _) | (_, Top) => self.clone(),
            (_, Bot) | (Top, _) => other.clone(),
            (Base(l), Base(r)) => if l == r { self.clone() } else { Bot }
        }
    }
}