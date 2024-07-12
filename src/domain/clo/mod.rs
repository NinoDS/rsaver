use std::collections::HashMap;
use crate::domain::value::BasicValue;
use crate::ir::Id;

pub struct Function {} // TODO

pub enum BasicClo {
    Bot,
    ConstElem {
        params: Vec<Id>,
        locals: HashMap<Id, BasicValue>,
        func: Function
    }
}