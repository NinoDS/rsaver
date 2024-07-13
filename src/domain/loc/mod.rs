use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};

struct Loc {} // TODO

pub enum SetLoc {
    Top,
    Base(HashSet<Loc>)
}

impl Display for SetLoc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SetLoc::Top => write!(f, "#"),
            SetLoc::Base(set) => match set.len() {
                0 => write!(f, "⊥"),
                _ => todo!(),
            }
        }
    }
}

impl SetLoc {
    fn alpha(elems: HashSet<Loc>) -> Self {
        Self::Base(elems)
    }
}