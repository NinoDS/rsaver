use std::collections::HashSet;
use std::fmt::{Debug, Display, Formatter};

const MAX_SIZE: usize = 5;

pub enum SetStr {
    Top,
    Base(HashSet<String>)
}

impl Display for SetStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SetStr::Top => write!(f, "str"),
            SetStr::Base(set) => match set.len() {
                0 => write!(f, "⊥"),
                1 => write!(f, "\"{}\"", set.iter().next().unwrap()),
                _ => write!(f, "{:?}", set),
            }
        }
    }
}

impl SetStr {
    fn alpha(elems: HashSet<String>) -> Self {
        if (elems.len() > MAX_SIZE) {
            return Self::Top
        }
        Self::Base(elems)
    }
}