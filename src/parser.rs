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
    Expression {
        span: Span,
        body: Box<ASTNode>,
    },
    Literal {
        span: Span,
        kind: String,
        raw: String,
    },
}

#[derive(Clone, Debug, Serialize)]
pub struct Program {
    #[serde(rename = "type")]
    kind: String,
    body: Vec<ASTNode>,
    #[serde(rename = "sourceType")]
    source_type: String,
}

impl Program {
    pub fn new(expressions: Vec<ASTNode>) -> Self {
        Program {
            kind: "Program".into(),
            body: expressions,
            source_type: "module".into(),
        }
    }
}
