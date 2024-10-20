%start Program
%expect-unused Unmatched "UNMATCHED"
%avoid_insert "number"
%epp UNDEFINED "undefined"
%epp TRUE "true"
%epp FALSE "false"
%epp NULL "null"
%%
Program -> Result<Vec<ASTNode>, ()>:
    ProgramElement { Ok(vec![$1]) }
    | Program ProgramElement { Ok(combo($1?, $2)) }
    ;

ProgramElement -> ASTNode:
    VariableDeclaration { $1 }
    ;


VariableDeclaration -> ASTNode:
    VariableDeclarationKind VariableDeclaratorList { VariableDeclaration{ span: $span, kind: $1, declarations: $2 } }
    ;

VariableDeclarationKind -> String:
    'VAR' { $lexer.span_str($span).to_string() }
    | 'LET' { $lexer.span_str($span).to_string() }
    | 'CONST' { $lexer.span_str($span).to_string() }
    ;

VariableDeclaratorList -> Vec<ASTNode>:
    VariableDeclarator { vec![$1] }
    | VariableDeclaratorList ',' VariableDeclarator { combo($1, $3) }
    ;

VariableDeclarator -> ASTNode:
    Identifier '=' Literal { VariableDeclarator{ span: $span, id: Box::new($1), init: Box::new($3) } }
    ;

Identifier -> ASTNode:
    'identifier' { Identifier{ span: $span, name: $lexer.span_str($span).to_string() } }
    ;

Literal -> ASTNode:
    'number' { Literal{ span: $span, kind: "number".into(), raw: $lexer.span_str($span).to_string() } }
    | 'string' { Literal{ span: $span, kind: "string".into(), raw: $lexer.span_str($span).to_string() } }
    | 'NULL' { Literal{ span: $span, kind: "null".into(), raw: $lexer.span_str($span).to_string() } }
    | 'UNDEFINED' { Literal{ span: $span, kind: "undefined".into(), raw: $lexer.span_str($span).to_string() } }
    | 'TRUE' { Literal{ span: $span, kind: "boolean".into(), raw: $lexer.span_str($span).to_string() } }
    | 'FALSE' { Literal{ span: $span, kind: "boolean".into(), raw: $lexer.span_str($span).to_string() } }
    ;

Unmatched -> ():
  "UNMATCHED" { } 
  ;
%%

use crate::parser::ASTNode::*;
use crate::parser::ASTNode;

fn combo<T>(mut lhs: Vec<T>, rhs: T)
           -> Vec<T>
{
    lhs.push(rhs);
    lhs
}

