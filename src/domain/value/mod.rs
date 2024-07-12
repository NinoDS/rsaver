use crate::domain::ast::FlatAST;
use crate::domain::cont::BasicCont;
use crate::domain::simple::BasicSimple;

pub struct BasicValue {
    cont: BasicCont,
    ast: FlatAST,
    simple: BasicSimple,
}