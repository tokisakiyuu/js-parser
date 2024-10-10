%start Program
%%
Program -> Result<Vec<ASTNode>, ()>:
    ExpressionList { Ok($1?) }
    ;

ExpressionList -> Result<Vec<ASTNode>, ()>:
    Expression { Ok(vec![$1?]) }
    | ExpressionList Expression { flatten($1, $2) }
    ;

Expression -> Result<ASTNode, ()>:
    VariableDeclaration { Ok(ASTNode::Expression{ span: $span, body: Box::new($1?) }) }
    ;


VariableDeclaration -> Result<ASTNode, ()>:
    VariableDeclarationKind VariableDeclaratorList { Ok(ASTNode::VariableDeclaration{ span: $span, kind: $1, declarations: $2? }) }
    ;

VariableDeclarationKind -> String:
    'VAR' { lexeme_str!($lexer, $1) }
    | 'LET' { lexeme_str!($lexer, $1) }
    | 'CONST' { lexeme_str!($lexer, $1) }
    ;

VariableDeclaratorList -> Result<Vec<ASTNode>, ()>:
    VariableDeclarator { Ok(vec![$1?]) }
    | VariableDeclaratorList ',' VariableDeclarator { flatten($1, $3) }
    ;

VariableDeclarator -> Result<ASTNode, ()>:
    Identifier '=' Literal { Ok(ASTNode::VariableDeclarator{ span: $span, id: Box::new($1?), init: Box::new($3?) }) }
    ;

Identifier -> Result<ASTNode, ()>:
    'identifier' { Ok(ASTNode::Identifier{ span: $span, name: $lexer.span_str($1.as_ref().unwrap().span()).to_string() }) }
    ;

Literal -> Result<ASTNode, ()>:
    'number' { Ok(ASTNode::Literal{ span: $span, kind: "number".into(), raw: lexeme_str!($lexer, $1) }) }
    | 'string' { Ok(ASTNode::Literal{ span: $span, kind: "string".into(), raw: lexeme_str!($lexer, $1) }) }
    | 'NULL' { Ok(ASTNode::Literal{ span: $span, kind: "null".into(), raw: lexeme_str!($lexer, $1) }) }
    | 'UNDEFINED' { Ok(ASTNode::Literal{ span: $span, kind: "undefined".into(), raw: lexeme_str!($lexer, $1) }) }
    | 'TRUE' { Ok(ASTNode::Literal{ span: $span, kind: "boolean".into(), raw: lexeme_str!($lexer, $1) }) }
    | 'FALSE' { Ok(ASTNode::Literal{ span: $span, kind: "boolean".into(), raw: lexeme_str!($lexer, $1) }) }
    ;
%%

use crate::parser::ASTNode;

fn flatten<T>(lhs: Result<Vec<T>, ()>, rhs: Result<T, ()>)
           -> Result<Vec<T>, ()>
{
    let mut flt = lhs?;
    flt.push(rhs?);
    Ok(flt)
}

macro_rules! lexeme_str {
    ($lexer:ident, $lexeme:ident) => {
        $lexer.span_str($lexeme.as_ref().unwrap().span()).to_string()       
    };
}
