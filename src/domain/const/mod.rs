use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};

struct AConst {} // TODO

pub enum SetConst {
    Top,
    Base(HashSet<AConst>)
}

impl Display for SetConst {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SetConst::Top => write!(f, "~"),
            SetConst::Base(set) => match set.len() {
                0 => write!(f, "⊥"),
                _ => todo!(),
            }
        }
    }
}

impl SetConst {
    fn alpha(elems: HashSet<AConst>) -> Self {
        Self::Base(elems)
    }
}