use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::domain::value::BasicValue;

pub struct CompResult {
    value: BasicValue,
    target: BasicValue,
}

struct BasicComp { pub map: HashMap<String, CompResult> }