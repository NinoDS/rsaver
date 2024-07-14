use FlatDomain::*;
use crate::domain::domain::Flat;
use crate::domain::domain::Flat::{FlatBot, FlatElem, FlatTop};

#[derive(Clone)]
pub enum FlatDomain<T> {
    Top,
    Base(T),
    Bot
}

impl<T: PartialEq + Clone> FlatDomain<T> {
    fn get_single(&self) -> Flat<T> {
        match self {
            Bot => FlatBot,
            Top => FlatTop,
            Base(elem) => FlatElem(elem.clone())
        }
    }

    fn contains(&self, elem: &T) -> bool {
        match self {
            Bot => false,
            Top => true,
            Base(x) => x == elem
        }
    }
}