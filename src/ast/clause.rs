//! Clauses
//!
//! See: [6.5 Clauses](http://erlang.org/doc/apps/erts/absform.html#id88135)
use ast;
use ast::pat;
use ast::guard;
use ast::expr;

#[derive(Debug,Clone)]
pub struct Clause {
    pub line: ast::LineNum,
    pub patterns: Vec<pat::Pattern>,
    pub guards: Vec<guard::OrGuard>,
    pub body: Vec<expr::Expression>,
}
impl_node!(Clause);
impl Clause {
    pub fn new(line: ast::LineNum,
               patterns: Vec<pat::Pattern>,
               guards: Vec<guard::OrGuard>,
               body: Vec<expr::Expression>)
               -> Self {
        Clause {
            line: line,
            patterns: patterns,
            guards: guards,
            body: body,
        }
    }
}
