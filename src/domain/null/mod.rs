use std::fmt::{Display, Formatter};
use crate::domain::domain::Flat;
use crate::domain::domain::Flat::{FlatBot, FlatElem};
use crate::domain::null::NullDomain::{Bot, Top};

struct Null;

#[derive(Copy, Clone)]
pub enum NullDomain {
    Top,
    Bot
}

impl Display for NullDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Top => write!(f, "null"),
            Bot => write!(f, "⊥"),
        }
    }
}

impl NullDomain {
    // partial order
    fn leq(&self, other: &Self) -> bool {
        match (self, other) {
            (Bot, _) | (_, Top) => true,
            (_, Bot) | (Top, _) => false,
        }
    }

    // join operator
    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (Bot, _) | (_, Top) => *other,
            (_, Bot) | (Top, _) => *self,
        }
    }

    // meet operator
    fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (Bot, _) | (_, Top) => *self,
            (_, Bot) | (Top, _) => *other,
        }
    }

    // get single value
    fn get_single(&self) -> Flat<Null> {
        match self {
            Bot => FlatBot,
            Top => FlatElem(Null)
        }
    }
}