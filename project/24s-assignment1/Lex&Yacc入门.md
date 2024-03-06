# Lex + Yacc入门

Lex + Yacc 是一套代码生成器，可生成 c代码；该 c 代码的输入是 target language 的源代码，会按你写的规则来解析源代码并构建出 AST。

词法分析的文件是 `lexer.lex`，语法分析的文件是 `parser.yacc`。虽然编译的步骤是先进行词法分析，但编写解析规则时却不是先写 `lexer.lex`。

这两个文件都分为三个部分，两个部分之间用两个百分号 `%%` 隔开：

```yacc
声明部分
%%
规则部分
%%
辅助函数
```

本文档将使用以下文法作为例子：

```cfg
identifier -> [a-z_A-Z][a-z_A-Z0-9]*
number -> [1-9][0-9]* | 0
expr -> number | identifier | (expr + expr) | (expr * expr)
```

## Step 0 环境配置

这里默认大家已经有一个Ubuntu环境，打开终端，执行以下命令：

```shell
$ sudo apt-get install flex bison
```

部分系统可能还需要手动安装gcc、make等，但这里默认大家已经使用过这些工具。

完成安装后可以通过以下命令进行测试：

```shell
$ lex --version # 查看版本，若出现版本信息则说明安装成功
$ yacc --version # 查看版本，若出现版本信息则说明安装成功    
```

## Step 1   确定终结符、非终结符 、token

根据文法，可以确定终结符与非终结符。对于上述文法，一位数字、字母和运算符是终结符，`identifier`、`number`和`expr`是非终结符。

`lexer.lex`以终结符为输入，向`parser.yacc`返回`token`，可以把`token`理解为“词”。`parser.yacc`则以`token`为输入，根据每个非终结符的规则进行解析，最后构建出 AST。

但 `lexer.lex` 解析的`token`可以不是终结符，比如一个`identifier`可以是一个 `token`，但你也可以把字母当作`token`。推荐把`identifier`和`number`当作`token`，这样可以在 lex 中完成组装数字或变量的工作，让 yacc 更简洁。

## Step 2   yacc 声明

`token`在`parser.yacc`的声明部分定义，因此应该先写 yacc 的声明部分，再写 lex ，并且`lexer.lex`的开头也要导入 yacc 编译出的`y.tab.h`库。

给 token 和非终结符分类，可以是数字类或抽象类，单个字符可以也不定义而直接作为 char 返回。大多数情况下，导入`slp.h`，然后使用AST节点的类就是不错的选择。

将它们的名字和类别名按以下的格式写入 `parser.y` 的声明部分：

```parser.yacc
%union
{
    int token; // 例：数字类
    A_exp expr; // 例：自定义的类
}

// token的类
// %token <name in union> token_name_1 token_name_2
%token <token> OP_PLUS OP_MULTIPLY
%token <token> NUMBER
%token <expr> IDENTIFIER

// 非终结符的类
%type <expr> EXPR
```

## Step 3   编写 lexer 规则

在 `lexer.lex` 的规则部分添加 lexer 规则。

每条规则的形式：`正则表达式 { c 代码 }`

这部分的 c 代码一般做两件事：

- 对 `yylval` 进行赋值。`yylval` 的类型是上面写的 union，用来记录这个 token 的内容。
- `return token_name;` 返回 token 的名字。

你可以在用户自定义函数部分定义新的函数，例如用来拼接数字的`calc`函数。

```lexer.lex
%{

#include "util.h"
#include "slp.h"
#include "y.tab.h"

int c;

%}

%%

" " ;
"+" { yylval.token = OP_PLUS; return OP_PLUS; }
"*" { yylval.token = OP_MULTIPLY; return OP_MULTIPLY; }
"(" | ")" {
    c = yytext[0];
    return(c);
}
[a-z_A-Z][a-z_A-Z0-9]* { 
    yylval.expr = A_IdExp(String(yytext));
    return IDENTIFIER;
}
[1-9][0-9]* {
    yylval.token = calc(yytext, yyleng);
    return NUMBER;
}
0 {
    yylval.token = 0;
    return NUMBER;
}
. {
    printf("Illegal input \"%c\"\n", yytext[0]);
}
%%


int calc(char *s, int len) {
    int ret = 0;
    for(int i = 1; i < len; i++)
        ret = ret * 10 + (s[i] - '0');
    return ret;
}
```

需要注意两点：

- lex 规则有优先级：优先匹配最长的，再匹配靠前的规则。
- 这里用了 `yytext` 和 `yyleng`。从上下文可以看出，它们分别表示匹配到的子串的内容和长度。还有一些 yy 开头的变量，你可以在 `lex lexer.lex -o lexer.c` 后查看（现在还不能执行这个）。

## Step 4   编写 yacc 规则

在 `parser.y` 的规则部分添加 yacc 规则。

yacc 规则的形式是：`非终结符名字 : 导出式右部 { 带变量 $$ 和 $1 的 c 代码} | 更多的导出式右部与处理`

其中：

- `$$` 表示返回值，没有显式的 `return` 语句。返回值类型需与非终结符名字的类型一致。
- `$1` 表示导出式的第几部分，从1开始编号。

`parser.yacc` 代码续：

```parser
%%
EXPR: NUMBER
      {
          $$ = A_NumExp($1)
      } 
      | 
      IDENTIFIER 
      {
          $$ = $1
      }
      |
      "(" EXPR OP_PLUS EXPR ")" 
      {
          $$ = A_OpExp($2, A_plus, $4);
      } 
      | 
      "(" EXPR OP_MULTIPLY EXPR ")" 
      {
          $$ = A_OpExp($2, A_times, $4);
      }
%%
```

## Step 5   yacc库中其他接口

编译`parser.yacc`时，yacc会根据我们写的规则生成 C 语言的 parser 代码，其中可能涉及几个常用的 yacc 库接口。为了保证编译成功，我们需要在 yacc 的声明部分声明以下外部接口：

```c
extern int yylex();
extern void yyerror(char*);
extern int  yywrap();
```

再在结尾的辅助函数部分增加以下代码：

```c
void yyerror(s)
char *s;
{
  fprintf(stderr, "%s\n",s);
}

int yywrap()
{
  return(1);
}
```

## Step 6   编译parser和lexer

```shell
yacc -d parser.yacc  # 用yacc生成parser代码
yacc --help          # 查看yacc使用帮助
lex lexer.lex        # 用lex生成lexer代码，需要确保lexer中已经添加了parser库 
```

## Step 7   使用这些生成的代码

在主程序中调用 `yyparse()`。`yyparse()` 从 `stdin` 中读取源代码，然后执行 lexer 和 parser 的工作。这个函数不会返回 AST 的 root，所以需要在 `parser.yacc` 中添加一个 `extern A_exp root;` 然后在 yacc 规则中赋值使用。

如果你的 `main` 从 `argv` 中读取文件名，可以用 `freopen(filename, "r", stdin);` 。

## Step 8   lex和yacc的debug

lex的debug可以在结尾增加这样一个通配的正则表达式，将未被识别的字符打印出来：

```lexer.lex
. {
    printf("Illegal input \"%c\"\n", yytext[0]);
}
```

yacc的debug可以使用自带的debug模式：

1. 在yacc开头的定义部分增加`extern int yydebug=1`

2. 运行`yacc a.yacc -d -v --debug`，这一步相当于生成了一个debug版本的C代码，所以要回到正常模式，使用`yacc a.yacc`等语句正常编译，或删除yydebug标识重新编译即可。

3. 正常make
