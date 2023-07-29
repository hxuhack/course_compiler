## Grammar of TeaPL

Each program is composed of variable declarations, function declarations, function definitions, and comments.

```
program := (varDeclStmt | strctDef | fnDeclStmt | fnDef | comment | <;>)*
```

### Basic Identifiers, Values, Expressions, and Assignments

Each identifier begins with an alphabat and contains only alphabats and digits, e.g., alice, a0.

```
id := [a-zA-Z] [a-zA-Z0-9]*   
```

TeaPL allows integers and floating-point numbers, e.g., 123, 3.14
```
num := [1-9] [0-9] (<.> [0-9] | ϵ)
```

**Arithmatic Expressions**
An expression is a composd of identifiers, values,  and operators, e.g., 1+2, a*(b+c). For simplicity, we donot support unary operators, such as ++, +=.
```
arithExpr :=  arithExpr addSub arithExpr1 | arithExpr1  
arithExpr1 :=  arithExpr1 mulDiv arithExpr2 | arithExpr2  
arithExpr2 :=  exprUnit expOp arithExpr2 | exprUnit  
exprUnit :=  num | id | <(> arithExpr <)> | fnCall | ptrDeref
addSub := <+> | <->
mulDiv := <*> | </> | <%>  
expOp := <^>
```

**Condition Expressions**
```
condExpr := condExpr andOr condUnit | condUnit
condUnit := exprUnit comOp exprUnit | <(> condExpr <)> | <~>(condExpr)// we restrict the operands of comparison operators to be exprUnit instead of expr to avoid confusing the precedence.
andOr := <&&> | <||>
comOp := <>> | <<> | <>=> | <<=>
```

**Bitwise Expressions**
```
bitExpr := bitUnit | bitExpr <&> bitUnit | bitExpr <|> bitUnit
bitUnit := exprUnit | ~exprUnit | <(> bitExpr <)> | <~>(bitExpr)
```

**Assignment**
We restrict neither the left value nor right value can be assignments.
```
assignStmt := leftVal <=> rightVal <;>  
leftVal := id | id <[> (id | num) <]> | ptrDeref | fnCall  
rightVal := arithExpr | condExpr | bitExpr 
```

**Function Call**
```
fnCall := id <(> (rightVal (<,> rightVal)*) | ϵ<)>
```

**Pointer Dereference**
```
ptrDeref := <star> exprUnit
```

### Variable Declarations

TeaPL allows declaring one variable each time, which can be either a primitive or array type. Developers can initializate the variable during declaration. For example, it supports the following variable declaration samples.

**Primitive Types**
```
let a:int; // declare a variable of type int; the type field can be ignored.
let b:int = 0; // declare a variable of int and init it with value 0.
let p:*int = &a; // declare a variable of pointer and init it with the address of a.
```
**One-level Array**
```
let c[10]:int; // declear a variable of integer array.
let d[10]:int = {0};  // declear a variable of integer array and initialize it with zero.
```

**Two-level Array**
```
let c[10][10]:int; // declear a variable of integer array.
```

The grammar is defined as follows.
 ```
varDeclStmt := <let> (varDecl | varDef) <;>   
varDecl := id <:> type  
varDef :=  id <:> type <=> expr  //primitive type
         | id <[> expr <]><:> type <=> <{> expr <}> //array
type := nativeType | ptrType | structType 
nativeType := <int> | <float> | <char>  
ptrType := <*> type  
structType := <struct> id
```

### Define A New Structure
Developers can define new customized types with the preserved keyword struct, e.g., 
```
struct MyStruct { 
    node:*int, 
    len:int  
}
```

The grammar is defined as follows.
 ```
structDef := <struct> <{> (varDecl) (<,> varDecl)* <}>
```

### Function Declaration and Definition

Each function declaration starts with the keyword fn.
```
fn foo(a:int, b:int)->int;
fn foo();
```

The grammar is defined as follows.
```
fnDeclStmt := fnDecl <;>
fnDecl := <fn> id <(> paramDecl <)> <;>  //without return value
            | <fn> id <(> paramDecl <)> <=>> type <;> //with return value
paramDecl := id <:> type (<,> id <:> type)*    
```

**Function Definition**
We can also define a function while declaring it.
```
fn foo(a:int, b:int)->int {
    return a + b;
} 
```

The grammar is specified as follows.
```
fnDef := fnDecl codeBlock  
codeBlock :=  <{> (varDeclStmt | assignStmt | callStmt | ifStmt | matchStmt | forStmt | whileStmt)* <}> 
```

We have already defined the grammar of varDeclStmt and assignStmt. The callStmt is simply a function call terminated with an colon.
```
callStmt := fnCall <;>
```
Next, we define the grammar of each rest statement type.


### Control Flows
**If-Else Statement**
The condition should be surrounded with a paired parenthesis, and we further restrict the  body should be within a paired bracket. The following shows an example.
```
if (x>0) {
    if (y>0) {
        x++;
    }
    else {
        x--;
    }
} else {

}

```

Besides, we restrict the condition expression to be explicit logical operations, e.g., x>0; we donot allow implicit expressions like x, which means.  We define the grammar as follows.
```
ifStmt := <if> <(> condExpr <)> codeBlock ( <else> codeBlock | ϵ )
```

**Match Statemet**

Example:
```
match(x) { 
    0 => { x++; }
    1 => { x--; }
    _ => {  } //default
}

```
Definition:
```
matchStmt := <match> <(> id <)> <{> (num | id) <=>> block <}>
```

**For Statemet**

We want to restrict its representability in two aspects: 1) the index is updated automatically with an interval of one; 2) the loop index should be immutable within the body of for.

Example:
```
for i in 1..100 {
    x[i] = i;
}
```
Definition:
```
forStmt := <for> <(> id <in> range <)> block
range := (id|num) <..> (id|num)
```

**While Statemet**

Used for the representability of complicated loops.

Example:
```
while (x > 0) {
    x--;
}
```

Definition:
```
whileStmt := <while> <(> cond <)> block
```

### Code Comments 

Similar to most programming languages, TeaPL allows line comments with "//" and scope comments with "/* ... */".
```
int a = 0; // this is a line comment.

/*
    Feature: this is a scope comment
*/  
fn foo(){
    ...
}
```

```
comment :=  <//> _* | </*> _* <*/>  
```
