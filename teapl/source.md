## Grammar of TeaPL

```
program := (varDeclStmt | fnDeclStmt | fnDef | comment)*
```

### Variable Declaration Statement

Variable declaration samples supported in TeaPL.
```
let a:int; // declare a variable of type int; the type field can be ignored.
let b:int = 0; // define a variable of int with value 0.
let c:int, d:float; // declear two variables; you can also define two variables with one let.
let e[n]:int;  // declear a variable of integer array.
```
The grammar is defined as follows.
```
varDeclStmt := \<LET\> (varDecl | varDef) (\<COMMA\> (varDecl | varDef))* \<SEMI\>   
varDecl := Id <COLON> Id   
ID := [a-zA-Z][a-zA-Z0-9]*  
varDef := Id <COLON> Id <EQ> Expr
Expr := Id | Var | (Id | Var) OP (Id | Var)   
```

### Function Declaration Statement

### Function Definition

### Comment
