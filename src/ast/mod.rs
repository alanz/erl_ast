macro_rules! impl_from {
    ($to:ident :: $constructor:ident ($from:ty)) => {
        impl ::std::convert::From<$from> for $to {
            fn from(x: $from) -> Self {
                $to::$constructor(::std::convert::From::from(x))
            }
        }
    }
}

macro_rules! impl_node {
    ($x:ident <$a:ident, $b:ident>) => {
        impl <$a, $b> ::ast::Node for $x<$a, $b> {
            fn line(&self) -> ::ast::LineNum {
                self.line
            }
        }
    };
    ($x:ident <$a:ident>) => {
        impl <$a> ::ast::Node for $x<$a> {
            fn line(&self) -> ::ast::LineNum {
                self.line
            }
        }
    };
    ($x:ty) => {
        impl ::ast::Node for $x {
            fn line(&self) -> ::ast::LineNum {
                self.line
            }
        }
    };
}

pub mod form;
pub mod literal;
pub mod pat;
pub mod expr;
pub mod clause;
pub mod guard;
pub mod ty;
pub mod common;

pub type LineNum = i32;
pub type Arity = u32;

pub trait Node {
    fn line(&self) -> LineNum;
}

#[derive(Debug,Clone)]
pub struct ModuleDecl {
    pub forms: Vec<form::Form>,
}
