use std::collections::HashSet;
use crate::str::SetStr;

mod bool;
mod int;
mod absent;
mod undef;
mod null;
mod str;
mod bigint;
mod num;

fn main() {
    let mut s = SetStr::Top;
    println!("{}", s);
}
