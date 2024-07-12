use std::fmt::{Display, Formatter};
use crate::domain::flat::FlatDomain;

struct AST {}

type FlatAST = FlatDomain<AST>;


impl Display for FlatAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FlatAST::Top => write!(f, "☊"),
            FlatAST::Base(_) => todo!(),
            FlatAST::Bot => write!(f, "⊥"),
        }
    }
}