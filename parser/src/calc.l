%%
var "VAR"
let "LET"
const "CONST"
null "NULL"
undefined "UNDEFINED"
true "TRUE"
false "FALSE"
= "="
; ";"
, ","
[0-9]+ "number"
"(?:[^"\\]|\\.)*"|'(?:[^'\\]|\\.)*' "string"
[_A-Za-z$][_A-Za-z0-9-]*(?:\.[_A-Za-z][_A-Za-z0-9-]*)* "identifier"
[\t\n; ]+ ;
. "UNMATCHED"
