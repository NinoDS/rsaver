use std::fmt::{Display, Formatter};
use crate::domain::flat::FlatDomain;

pub struct AST {}

pub type FlatAST = FlatDomain<AST>;


impl Display for FlatAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FlatAST::Top => write!(f, "☊"),
            FlatAST::Base(_) => todo!(),
            FlatAST::Bot => write!(f, "⊥"),
        }
    }
}