### Compiler Principles and Techniques 
This is the course webpage of COMP130014.02 Compiler Principles and Techniques of Spring 2024 for undergraduate students at Fudan University. 

**Classroom**: HGX207 (1:30pm-4:10pm)

### Lecture Notes

| Date | Week | Topic | Materials |
|:---------:|:---------:|:---------:|:------------------:|
| Feb 26 |  1  | 课程入门  | [slides](slides/L1-课程入门.pdf), [notes](notes/l1-intro.pdf) |
| Mar 4  |  2  | 词法分析 | [slides](slides/L2-词法分析.pdf), [notes](notes/l2-lex.pdf) |
| Mar 11 |  3  | 上下文无关文法 | [slides](slides/L3-上下文无关文法.pdf), [notes](notes/l3-cfg.pdf), [TeaPL语法标准](notes/appendix-teapl.pdf) |
| Mar 18 |  4  | 自顶向下解析 | [slides](slides/L4-自顶向下解析.pdf), [notes](notes/l4-topdown.pdf) |
| Mar 25 |  5  | 自底向上解析 | [slides](slides/L5-自底向上解析.pdf), [notes](notes/l5-bottomup.pdf) |
| Apr 1  |  6   | 类型推导 | [slides](slides/L6-类型推导.pdf), [notes](notes/l6-typecheck.pdf) |
| Apr 8  |  7   | 线性IR | [slides](slides/L7-线性IR.pdf), [notes](notes/l7-linearIR.pdf)  |
| Apr 15 |  8   | 静态单赋值 | [slides](slides/L8-静态单赋值.pdf), [notes](notes/l8-ssa.pdf) |
| Apr 22 |  9   | 过程内优化  | [slides](slides/L9-过程内优化.pdf), [notes](notes/l9-intraopt.pdf) |
| Apr 29 |  10   | 过程间优化  | [slides](slides/L10-过程间优化.pdf), [notes](notes/l10-interopt.pdf)|
| May 6  |  11   | 指令选择  | [slides](slides/L11-指令选择.pdf), [notes](notes/l11-instsel.pdf) |
| May 13 |  12  | 寄存器分配  | [slides](slides/L12-寄存器分配.pdf), [notes](notes/l12-regalloc.pdf) |
| May 20 |  13  | Guest Lecture by 张宏波 | |
| May 27 |  14  |   | |
| Jun 3    |  15  |   | |
| Jun 10  |  16  | 端午节 No Class  | |
| Jun 24  | 18  | Open Book Exam  | |

### Project

**Classroom**: H逸夫楼302, 305 (8:00am-9:40am)

**Teaching Assistant**: 陈实立(Head TA)、王兆瀚、柏露、董方、张涵星

[**Repository**](https://github.com/hxuhack/compiler_project)

| Date | Week | Topic | Materials | Responsible TA |
|:---------:|:---------:|:---------:|:------------------:|:----------:|
| Feb 29 |  1  | 通过LLVM了解编译器 | [slides](project/PJ0_LLVM.pdf) | 董方 |
| Mar 7   |  2  | Make a Parser: From Source Code to AST | [slides](project/24s-assignment1/Compiler_Assignment1.pdf), [Lex&Yacc-1](project/24s-assignment1/Lex&Yacc入门.md), [Lex&Yacc-2](project/24s-assignment1/Lex&Yacc进阶.md), [assignment1](project/24s-assignment1/Compiler_Assignment1.md), branch: 24s-assignment1 | 陈实立 |
| Mar 14 |  3  | 实验一补说明 | [slides](project/24s-assignment1/Compiler_Assignment1-补充.pdf) | 陈实立 |
| Mar 21 |  4  | 实验一答疑  | | 陈实立 |
| Mar 28 |  5  | 实验一验收、实验二介绍：Type check  | [slides](project/24s-assignment2/Assignment2_typecheck.pdf) | 陈实立、董方 |
| Apr 4   |  6   | 假期停课 | - | - |
| Apr 11 |  7   | 实验三介绍：Linear lR Gen | [assignment3](project/24s-assignment3/assignment3.pdf),[genLinearIR](project/24s-assignment3/genLinearIR.pdf),[LLVMIR](project/24s-assignment3/LLVMIR.pdf) | 柏露 |
| Apr 18 |  8   | 实验四介绍：SSA |[assignment4](project/24s-assignment4/assignment4.pdf) | 王兆瀚  |
| Apr 25 |  9   | 实验二验收 | | 董方 |
| Apr 28 |  10   | 答疑 | | 柏露、王兆瀚 |
| May 9  |  11   | 实验三验收 | | 柏露 |
| May 16 |  12  | 实验四验收  | | 王兆瀚  |
| May 23 |  13  | 实验五 | | 柏露、张涵星 |
| May 30 |  14  | 实验五答疑 | | 柏露、张涵星 |
| Jun 6    |  15  | 实验五答疑 |  | 柏露、张涵星|
| Jun 13  |  16  | 实验五验收 | | 柏露、张涵星 |
<!--
| Date | Week | Topic | Materials |
|:---------:|:---------:|:------------------:|:----------------------------------:|
| Sep 15 | 2 | 使用解析工具开发计算器 | [文件](project/Assignment-1.zip) |
| Sep 29 | 4 | No Class 中秋节假期 |  |
| Oct 13 | 6 | TeaPL编译器开发：语法解析 | [Link](https://github.com/hxuhack/compiler_project/tree/assignment-2) |
| Oct 27 | 8 | TeaPL编译器开发：类型检查 | [Link](https://github.com/hxuhack/compiler_project/tree/ass-3-typecheck) |
| Nov 10 | 10 | TeaPL编译器开发：线性IR | [Link](https://github.com/hxuhack/compiler_project/blob/assignment4/src/assignment4.md) |
| Nov 24 | 12 | TeaPL编译器开发：SSA | [Link](https://github.com/hxuhack/compiler_project/tree/assignment5) |
| Dec 8   | 14 | TeaPL编译器开发：指令选择 | [Link](https://github.com/hxuhack/compiler_project/tree/assignment6) |
| Dec 22 | 16 | | |
| Jan 5 | Week 18 | Open Book Exam (15:30-17:30) |  |

The specifications of the programming languages and intermidiate code employed in this course can be found [here](teapl/README.md). 
-->



