//! Patterns
//!
//! See: [6.3 Patterns](http://erlang.org/doc/apps/erts/absform.html#id87135)
use ast;
use ast::literal;

pub type Var = ast::Variable;
pub type Nil = ast::Nil;
pub type Match = ast::Match<Pattern, Pattern>;
pub type Tuple = ast::Tuple<Pattern>;
pub type Cons = ast::Cons<Pattern>;
pub type Binary = ast::Binary<Pattern>;
pub type UnaryOp = ast::UnaryOp<Pattern>;
pub type BinaryOp = ast::BinaryOp<Pattern>;
pub type Record = ast::Record<Pattern>;
pub type RecordIndex = ast::RecordIndex<Pattern>;
pub type Map = ast::Map<Pattern>;

#[derive(Debug)]
pub enum Pattern {
    Integer(Box<literal::Integer>),
    Float(Box<literal::Float>),
    String(Box<literal::Str>),
    Char(Box<literal::Char>),
    Atom(Box<literal::Atom>),
    Var(Box<Var>),
    Match(Box<Match>),
    Tuple(Box<Tuple>),
    Nil(Box<Nil>),
    Cons(Box<Cons>),
    Binary(Box<Binary>),
    UnaryOp(Box<UnaryOp>),
    BinaryOp(Box<BinaryOp>),
    Record(Box<Record>),
    RecordIndex(Box<RecordIndex>),
    Map(Box<Map>),
}
impl_from!(Pattern::Integer(literal::Integer));
impl_from!(Pattern::Float(literal::Float));
impl_from!(Pattern::String(literal::Str));
impl_from!(Pattern::Char(literal::Char));
impl_from!(Pattern::Atom(literal::Atom));
impl_from!(Pattern::Var(Var));
impl_from!(Pattern::Match(Match));
impl_from!(Pattern::Tuple(Tuple));
impl_from!(Pattern::Nil(Nil));
impl_from!(Pattern::Cons(Cons));
impl_from!(Pattern::Binary(Binary));
impl_from!(Pattern::UnaryOp(UnaryOp));
impl_from!(Pattern::BinaryOp(BinaryOp));
impl_from!(Pattern::Record(Record));
impl_from!(Pattern::RecordIndex(RecordIndex));
impl_from!(Pattern::Map(Map));