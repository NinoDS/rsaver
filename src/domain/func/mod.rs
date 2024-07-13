use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};

struct AFunc {} // TODO

pub enum SetFunc {
    Top,
    Base(HashSet<AFunc>)
}

impl Display for SetFunc {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SetFunc::Top => write!(f, "λ"),
            SetFunc::Base(set) => match set.len() {
                0 => write!(f, "⊥"),
                _ => todo!(),
            }
        }
    }
}

impl SetFunc {
    fn alpha(elems: HashSet<AFunc>) -> Self {
        Self::Base(elems)
    }
}