use std::fmt::{Display, Formatter};
use crate::domain::domain::Flat;
use crate::domain::domain::Flat::{FlatBot, FlatElem};
use crate::domain::undef::UndefDomain::{Bot, Top};

struct Undef;

#[derive(Copy, Clone)]
pub enum UndefDomain {
    Top,
    Bot
}

impl Display for UndefDomain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Top => write!(f, "undefined"),
            Bot => write!(f, "⊥"),
        }
    }
}

impl UndefDomain {
    // partial order
    fn leq(&self, other: &Self) -> bool {
        match (self, other) {
            (Bot, _) | (_, Top) => true,
            (_, Bot) | (Top, _) => false,
        }
    }

    // join operator
    fn join(&self, other: &Self) -> Self {
        use UndefDomain::*;
        match (self, other) {
            (Bot, _) | (_, Top) => *other,
            (_, Bot) | (Top, _) => *self,
        }
    }

    // meet operator
    fn meet(&self, other: &Self) -> Self {
        use UndefDomain::*;
        match (self, other) {
            (Bot, _) | (_, Top) => *self,
            (_, Bot) | (Top, _) => *other,
        }
    }

    // get single value
    fn get_single(&self) -> Flat<Undef> {
        match self {
            Bot => FlatBot,
            Top => FlatElem(Undef)
        }
    }
}