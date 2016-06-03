//! Atomic Literals
//!
//! See: [6.2 Atomic Literals](http://erlang.org/doc/apps/erts/absform.html#id87074)
use num::bigint::BigUint;
use num::traits::ToPrimitive;
use ast;

#[derive(Debug,Clone)]
pub struct Integer {
    pub line: ast::LineNum,
    pub value: BigUint,
}
impl_node!(Integer);
impl Integer {
    pub fn new(line: ast::LineNum, value: BigUint) -> Self {
        Integer {
            line: line,
            value: value,
        }
    }
    pub fn to_u64(&self) -> Option<u64> {
        self.value.to_u64()
    }
}

#[derive(Debug,Clone)]
pub struct Char {
    pub line: ast::LineNum,
    pub value: char,
}
impl_node!(Char);
impl Char {
    pub fn new(line: ast::LineNum, value: char) -> Self {
        Char {
            line: line,
            value: value,
        }
    }
}

#[derive(Debug,Clone)]
pub struct Float {
    pub line: ast::LineNum,
    pub value: f64,
}
impl_node!(Float);
impl Float {
    pub fn new(line: ast::LineNum, value: f64) -> Self {
        Float {
            line: line,
            value: value,
        }
    }
}

#[derive(Debug,Clone)]
pub struct Str {
    pub line: ast::LineNum,
    pub value: String,
}
impl_node!(Str);
impl Str {
    pub fn new(line: ast::LineNum, value: String) -> Self {
        Str {
            line: line,
            value: value,
        }
    }
}

#[derive(Debug,Clone)]
pub struct Atom {
    pub line: ast::LineNum,
    pub value: String,
}
impl_node!(Atom);
impl Atom {
    pub fn new(line: ast::LineNum, value: String) -> Self {
        Atom {
            line: line,
            value: value,
        }
    }
}
