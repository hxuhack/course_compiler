<center><h1>编译原理实验：Assignment 1</h1></center>

## 实验介绍

Assignment 1 的目标是实现 TeaPL 的语法分析最终得到抽象语法树。在本实验中，输入是 TeaPL 的源代码，输出是 TeaPL 的抽象语法树。我们将得到抽象语法树打印出来以检查程序的正确性。

### 实验工作流

本次实验的工作流在 `compiler.cpp` 中可以清楚的看到：我们首先调用 `yyparse` 来生成我们的抽象语法树（在 `TeaplAst.h` 中定义，这是一个 C 风格的定义（方便接入 lex 和 yacc）），然后我们调用 `aA_Program` 将抽象语法树转换为使用 STL 的定义（为了方便后续的实验），最后调用 `print_aA_Program` 将转换后的语法树打印出来。

### 文件介绍

- `compiler.cpp` 

    main 函数所在的文件，能够体现实验的总体流程

- `lexer.lex` 

    lexer，需要补全其中的代码以完成本次实验

- `Makefile` 

    输入 make 进行测试，输入 make clean 清除生成的文件

- `parser.yacc` 

    parser，需要补全其中的代码以完成本次实验

- `PrintTeaplaAst.h` 

    `PrintTeaplaAst.cpp` 的头文件，存放函数定义

- `PrintTeaplaAst.cpp` 

    用于输出语法树

- `TeaplaAst.h` 

    使用 STL 的语法树定义

- `TeaplaAst.cpp` 

    用于将 `TeaplAst.h` 中定义的语法树转换为 `TeaplaAst.h` 中定义的语法树

- `TeaplAst.h` 

    C 风格的语法树定义

- `TeaplAst.cpp` 

    用于构造 `TeaplAst.h` 中定义的语法树

## 实验要求

需要补全 `lexer.lex` 和 `parser.yacc` 中的代码以完成本次实验（补全代码后可以直接 `make` 进行测试）。

### 实验验收

3月28日实验课上验收，验收时执行 `make` 即可，助教会记录每位同学通过的测试点数目，按照通过的测试点数目进行给分。

### 关于提问

在微信群里提问或在[仓库](https://github.com/hxuhack/compiler_project)上提issue都可以（更推荐大家在仓库上提issue，这样更方便管理）

### 实验提交

本次实验除了验收还需要在elearning上提交代码，提交代码时直接将仓库打包即可，命名方式为学号-姓名，例如`20307110078-陈实立.zip`（组队的同学可命名为`20307110077-陈实力_20307110079-陈实丽.zip`）

### 关于迟交

本次（以及后续）作业的迟交得分标准为：迟交48小时以内在得分基础上\*90%，超过48小时但在72小时以内在得分基础上\*80%

迟交同学需要自行与负责对应实验的助教约定验收时间！