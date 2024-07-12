use std::collections::HashMap;
use crate::domain::value::BasicValue;

pub struct Id(pub usize);

pub struct NodePoint {} // TODO

pub enum BasicCont {
    Bot,
    ConstElem {
        params: Vec<Id>,
        locals: HashMap<Id, BasicValue>,
        target: NodePoint
    }
}