## Intermidiate Code of TeaPL

### Identifiers 
There are three types of identifiers:   
- temporal variables: %1, %2, ...
- local variables: %a, %b, ...
- global variables: @a, %b

### Function Definition And Call
**Definition**
```
@define fn void @foo(i32 %x){
    ...
    ret;
}
@define fn i32 @bar(i32 %y){
    ...
    ret i32 %1;
}
```
**Function Call**
```
call void @foo(1)
%1 = load %a
%2 = call i32 @test(i32 %1)
```

### Binary Operations
**Integer arithmatic**: The IR supports five types of integer operators, add/sub/mul/div/rem. The operands should be the same type as the return value. For example, 
```
%1 = add i32, %a, %b;
%1 = mul i32, %1, 2;
```
**Floating-point arithmatic**: Similarly, the IR supports four types of floating-point arithmatic operations, fadd/fsub/fmul/fdiv.
```
%1 = fadd f32, %a, %b;
%1 = fmul f32, %1, 2.1;
```

**Bitwise operations**: and/or/xor/shl/ashr/lshr

### Type Conversion

Data truncatation
```
%1 = trunc i64 %0, i32
%8 = fptrunc double %7, float
```

Data extension
```
%4 = sext i32 %0, i64
%5 = zext i32 %0, i64
```

Integer to float/Float to integer
```
%2 = sitofp i32 %0, f32
%3 = uitofp i32 %0, f32
%6 = fptoui f32 %2, i32
%7 = fpext f32 %2, f64
```

Integer to pointer/Pointer to integer
```
%9 = inttoptr i64 %4, i8*
%10 = ptrtoint i8* %9, i32
```

### Data Load/Store
Allocate a memory unit for a local variable on stack, and initialize it with value 1.
```
%a = salloc i32; 
store i32 1, %a;
```

Load data from local variables on stack, perform an add operation and store the results back.
```
%1 = load i32, %a;
%2 = add i32 %1, 1;
store i32 %2, %a;
```

### Addressing

```
%a = salloc [2 x i32];
%1 = getptr i32*, %a, 0, 1;
store i32 99, %1;
```

```
%struct.st = type { i32, f32 };
%s = salloc %struct.st;
%1 = getptr %struct.st.i*, %s, 0, 0;
store i32 1, %1;
```

### Comparison
```
%0 = cmp eq i32 4, 5;
%1 = cmp ne f32 0.1, 0.2;
%2 = cmp ult i32  4, 5;
%3 = cmp sgt i32  4, 5;
%4 = cmp ule i32 -4, 5;
%5 = cmp sge i32  4, 5;
```

### Control
**Direct jump**
```
jmp %BB1;
```

**Indirect jump**
```
jmp %1;
```

**Conditional jump**
```
cjmp %0, %BB1, %BB2
```

**Multiple branches**
```
match i32 %0, %BBdefault [ 
  i32 0, %BB1
  i32 1, %BB2
  i32 2, %BB3 ]
```




