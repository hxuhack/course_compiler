# Lex + Yacc进阶

## 1. Lex的状态

单纯使用正则表达式处理注释会非常复杂，因此 Lex 提供了状态机制。

Lex规则的完整形式为：`<状态>正则表达式 { c代码 }`。初始状态为 `INITIAL`，未标注状态的规则在所有状态下都生效（所以如果你希望规则仅在 `INITIAL` 状态下生效，需要在规则前面标注 `<INITIAL>`）。

状态在声明部分声明，格式为`%start 状态名`，例如声明两个状态用于处理两类注释：

```lex
%start COMMENT1  COMMENT2
```

转为某个状态的指令为`BEGIN 状态名`，例如遇到`//`，转入`COMMENT1` 状态处理注释：

```lex
"//" {  BEGIN COMMENT1;  }
```

注释结束后，返回`INITIAL`状态继续处理源代码：

```lex
<COMMENT1>"\n" {  BEGIN INITIAL;  }
```

    

## 2. Yacc的优先级和结合性

为了消除文法中的冲突，需定义规则的优先级与结合性。

在 `parser.yacc` 的声明部分添加：

```parser.yacc
%left '+' '-'
%left '*' '/'
%left UMINUS
```

`%`后面的结合性可以是： left 左结合、right 右结合、nonassoc 不结合；我们使用的一般都是左结合。

声明越靠下的符号优先级越高。没有运算符或运算符无法区分的规则，也可以通过在规则后增加`%prec`规定其优先级，例如负号和减法可如下处理：

```parser
exp '-' exp
{
    $$ = A_OpExp($1, A_minus, $3);
}
|
'-' exp %prec UMINUS
{
    $$ = A_OpExp(A_NumExp(0), A_minus, $2);
}
```

使用`yacc -v parser.yacc`指令编译，可以获得`y.output`文件，包含 Yacc 的状态信息。

        

## 3. 其他

1. Lex 中特殊符号的处理：代码中难免会包含空格、`\n`、`\t`等特殊符号，我们编写的 Lex 规则应接收这些符号，但不需要返回 token 给 Yacc，只更新位置信息即可。
