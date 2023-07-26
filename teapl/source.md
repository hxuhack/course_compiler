## Grammar of TeaPL

program := (varDeclStmt | fnDeclStmt | fnDef | comment)*

### Basic Identifiers and Values

id := [a-zA-Z] [a-zA-Z0-9]*   
num := [1-9] [0-9] (<.>[0-9]| $\epsilon$)  
expr :=  expr opL0 expr1 | expr1  
expr1 :=  expr1 opL1 expr2 | expr2  
expr2 :=  expr3 opL2 expr2 | expr3  
expr3 :=  num | id | <(> expr <)> 
opL0 := <+> | <->
opL1 := <*> | </> | <%>  
opL2 := <^>


### Variable Declaration Statement

Variable declaration samples supported in TeaPL.
```
let a:int; // declare a variable of type int; the type field can be ignored.
let b:int = 0; // define a variable of int with value 0.
let c[10]:int; // declear a variable of integer array.
let d[10]:int = {0};  // declear a variable of integer array.
```
The grammar is defined as follows.
 
varDeclStmt := <let> (varDecl | varDef) <;>   
varDecl := id <:> type  
varDef :=  id <:> type <=> expr | id <[> (id | num) <]><:> type <=> <{> num <}>  
type := nativeType | ptrType | id  
nativeType := <int> | <float> | <char>  
ptrType := <*> type  

### Function Declaration Statement

```
fn foo(a:int, b:int)->int;
fn foo();
```
fnDeclStmt := <fn> id <(> paramDecl <)> <;> | <fn> id <(> paramDecl <)> <=>> type <;>   
paramDecl := id <:> type (<,> id <:> type)*    

### Function Definition

```
fn foo(a:int, b:int)->int {return a + b;} 
```

fnDef := <fn> id <(> paramDecl <)> <{> stmts <}> | <fn> id <(> paramDecl <)> <=>> type <{> stmts <}>  
stmts :=  varDeclStmt | assignStmt | callStmt | ifStmt | matchStmt | forStmt | whileStmt  
assignStmt := leftVal <=> rightVal <;>  
leftVal := id | id <[> (id | num) <]> | fnCall  
rightVal := leftVal | num  
callStmt :=  

### Comment

comment :=  <//> _* | </*> _* <*/>  
