# 实验二：类型检查

## 实验目标

Assignment2 的目标是对 Assignment 1 给出的 AST 进行类型检查。检测到错误时，输出错误所在的位置和错误内容。

## 实验环境

本次实验的环境和实验二相同。__你需要首先将你在实验二中实现的 `parser.yacc`、`lexer.lex` 文件 copy 到本目录__ ，然后在本目录下运行

```
make
```

编译所有文件，并对所有 testcase 进行测试。如果编译成功且输出了 `TypeCheck Passed`，就说明你的环境已经准备好了。

## 上手

在 `compiler.cpp` 里可以看到，生成 AST 后调用 `check_Prog` 进行类型检查，你在 `check_Prog` 函数中拿到的即是整个程序的 AST 的 root。本次实验的内容即是 __补全 `TypeCheck.h` 和 `TypeCheck.cpp` 文件以实现类型检查__；这两个文件里已经定义了一些数据结构和函数，给出了类型检查的整体框架；你可能还需要额外定义自己的数据结构。如果检查到了类型错误，请调用 `error_print` 输出错误的位置和原因，这个函数定义在 `TypeCheck.cpp` 里。

实验一中的 `PrintTeaplaAst.cpp` 能帮你更直观地分析程序，定位可能出现问题的位置，提高你编码和debug的效率。
```
A_program 
	|--A_programElement 
		|--A_fnDef 
			|--A_fnDecl fn foo1(
				|--A_paramDecl 
					|--A_varDecl 
						|--A_varDeclScalar a:
						|--A_type int, 
					|--A_varDecl 
						|--A_varDeclScalar b:
						|--A_type int)->
			|--A_type int{

			|--A_codeBlockStmt 
				|--A_returnStmt ret 
				|--A_rightVal 
					|--A_arithExpr 
						|--A_exprUnit 
							|--A_fnCall foo2(
							|--A_rightVal 
								|--A_arithExpr 
									|--A_exprUnit b, 
							|--A_rightVal 
								|--A_arithExpr 
									|--A_exprUnit a);
```

如果需要额外的数据结构或者函数定义，可以自行设计。

## 类型检查规则
以下是本次实验的测试用例遵循的类型检查规则。规则里未提到的，均视为未定义行为，可以自行选择如何实现，不会作为评分依据。

### 类型
1. 原生类型只有 int。由于我们在实验一中实现的语法里支持布尔表达式，实际上还会派生出 bool 类型；但 bool 不会成为变量的类型，即
```
let a:bool = 2 > 1;
```
不会出现。

2. 其它类型只有用户自定义的 struct 类型，如
```
struct MyStruct{
	int a;
	int b;
}
```

### 变量
1. 声明时类型推断。实验一中实现的 TeaPL 语法允许在声明变量时省略类型，变量的类型将在它第一次被赋值时确定。如
```
let a = 5;
```
和
```
let a;
let b:int = 10;
a = b;
```
应当推断出变量 a 的类型为 int。

2. 赋值。所有赋值语句，左值和右值必须 __严格相同__。如
```
let a[10]:int;
a = 10;
```
应当报错。在 C 语言中这可能并不会报错，因为 C 语言有许多隐式类型转换规则，且允许使用地址直接访问内存；但在本次实验中，我们要求赋值语句的左值和右值类型严格相同。
数组的长度不属于类型严格相同的范围，即
```
let a[2]:int = {0, 1};
let b[3]:int = {0, 1, 2};
a = b;
```
是合法的；但是在使用列表对数组进行初始化时，列表的长度应当与数组的声明一致，即
```
let a[2]:int = {0, 1, 2};
```
应当报错。列表的每一个 entry 都要与数组的类型一致。
TeaPL 的语法不支持使用列表对已经声明的数组进行赋值，只能在声明时使用列表初始化，所以数组的赋值操作不需要考虑列表。

### 函数
1. 函数要么【只声明不定义】，要么【先声明后定义】，要么【直接定义】。无论哪种，只能出现一次。
```
fn foo(a:int, b:int)->int{
	ret b - a;
}

fn foo(a:int, b:int)->int{
    ret a + b;
}
```
上面这段代码应当报告重复定义的错误。同理即不支持函数重载，如
```
fn foo(a:int, b:int, c:int)->int{
	ret b - a + c;
}

fn foo(a:int, b:int)->int{
    ret a + b;
}
```

2. 函数的声明和定义应当一致，如
```
fn foo(a:int, b:int)->int;
fn foo(a:int)->int{
	......
}
```
应当报错。参数列表的参数名可以改变。

3. 函数的 return 语句，应当与函数声明的返回值相同。
```
fn foo(a:int)->MyStruct{
	ret 2 * a;
}
```
应当报错。函数声明没有返回值的 testcase 不会出现，可以自行决定如何实现。

### 作用域
1. 本次实验的测试单位是文件，不需要考虑跨文件作用域的问题。文件为全局，函数声明、if/while codeblock 等为局部。文件中直接声明的变量为全局变量；TeaPL 语法中函数只能在全局声明，所以无需考虑子作用域中声明函数的问题。
2. 函数和全局变量可以在任何作用域中生效，无论它在哪里声明。即，下述代码应当通过类型检查：
```
fn foo1(a:int, b:int)->int{
    ret foo2(b, a);
}

fn foo2(a:int, b:int)->int{
    ret a * x + b;
}

let x:int = 25;
```

3. 全局变量引用了其他全局变量时，被引用的全局变量必须在被引用之前声明。即下述代码
```
let x:int = a + 5;
let a:int = 5;
```
应当报告变量未定义错误。全局变量的声明不会依赖函数。

4. 函数的参数可以与全局变量名重复；在函数内部，函数的参数优先于全局变量。
```
let a : MyStruct;
fn foo(a : int)->int{
	ret 2 * a;
}
```
应当通过类型检查。

5. 局部变量不能与全局变量重名，如
```
1 let a:int = 203;
2 let b:int = 713;
3
4 fn foo(a:int)->int{
5     let b:int = 10;
6     ret a + b;
7 }
```
应当在第 5 行报告重复定义变量的问题。

6. 局部变量不能与其外部的局部变量重名，如
```
fn foo(a:int)->int{
	let i:int = 10;
	if (a > 0){
		let i:int = 10;
	}
	ret i + a;
}
```
和
```
fn foo(a:int)->int{
	let a:int = 10;
	ret a * a;
}
```
应当报错。而退出一个作用域时，其局部变量名即可以被重新使用，如
```
fn foo(a:int)->int{
	if (a > 0){
		let i = 0;
		while (i < a){
			......
		}
	}

	if (a < 0){
		let i = 0;
		while (i > a){
			......
		}
	}
}
```
应当通过类型检查。


### 测试

测试样例在 `tests/` 文件中，其中每个文件的末尾都以注释的形式提供了参考输出。直接在工程目录下运行 `make` 即可测试所有测试用例。

### 提交和评分

以学号命名提交压缩包，如 `22110240012.zip`。评分时会使用更多测试用例进行测试，按通过的测试数量进行评分。
