use std::collections::HashMap;
use crate::domain::value::BasicValue;
use crate::ir::Id;

pub struct NodePoint {} // TODO

pub enum BasicCont {
    Bot,
    ConstElem {
        params: Vec<Id>,
        locals: HashMap<Id, BasicValue>,
        target: NodePoint
    }
}