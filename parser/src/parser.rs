use lrpar::Span;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum ASTNode {
    VariableDeclaration {
        span: Span,
        kind: String,
        declarations: Vec<ASTNode>,
    },
    VariableDeclarator {
        span: Span,
        id: Box<ASTNode>,
        init: Box<ASTNode>,
    },
    Identifier {
        span: Span,
        name: String,
    },
    Literal {
        span: Span,
        kind: String,
        raw: String,
    },
}
