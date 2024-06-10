use lexgen_util::Loc;
use std::fmt::Debug;

#[derive(Clone, Copy, Debug)]
pub struct Range(pub Loc, pub Loc);

#[derive(Debug)]
pub struct Program {
    pub range: Range,
    pub body: Vec<VariableDeclaration>,
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub kind: VariableDeclarationKind,
    pub range: Range,
    pub declarations: Vec<VariableDeclarator>,
}

#[derive(Debug)]
pub enum VariableDeclarationKind {
    Var,
    Let,
    Const,
}

#[derive(Debug)]
pub struct VariableDeclarator {
    pub range: Range,
    pub id: Identifier,
    pub init: VariableDeclaratorInit,
}

#[derive(Debug)]
pub struct Identifier {
    pub range: Range,
    pub name: String,
}

#[derive(Debug)]
pub enum VariableDeclaratorInit {
    Literal(Literal),
}

#[derive(Debug)]
pub struct Literal {
    pub range: Range,
    pub value: JsValue,
}

#[derive(Debug)]
pub enum JsValue {
    String(String),
    Number(f64),
}
