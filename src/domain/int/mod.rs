use std::fmt::{Display, Formatter};
use crate::domain::flat::FlatDomain;
use crate::domain::flat::FlatDomain::*;

pub type FlatInt = FlatDomain<i64>;

impl Display for FlatInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Top => write!(f, "int"),
            Base(v) => write!(f, "{}i", v),
            Bot => write!(f, "⊥"),
        }
    }
}

impl FlatInt {
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