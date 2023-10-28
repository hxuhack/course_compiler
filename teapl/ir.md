## Intermediate Code of TeaPL

### Basics 
**Identifiers**
There are three types of identifiers:   
- global variables: @a, %b
- local variables: %a, %b, ...
- temporal variables: %r1, %r2, ..., %1, %2, ...(pure numbers should start from %0 and be continuous)

Requirements of naming:
- SSA form: each variable can be defined only once
- Continuous: pure numbers should start from %0 and be continuous

**Scala Types**
- void
- integer: i32, i8, i1
- pointer: i32*, i8*

**Compound Types**
- array: [i32 x 10]
- struct: 
```
%mystruct = type { i32, i32 }

```

**Global Variable Declaration and Initialization**
```
@g = global i32 10
```

**Local Variable Declaration**
```
%x = alloca i32
%ar = alloca [2 x i32]
%st = alloca %mystruct
```

**Data Load and Store**
```
store i32 %0, %x
%r1 = load i32, i32* %x
```

**Data Load and Store for Compound Types**
```
%ar = alloca [2 x i32]
%r1 = getelementptr [2 x i32], [2 x i32]* %ar, i32 0, i32 0
store i32 99, i32* %r1
%st = alloca %mystruct
%r2 = getelementptr %mystruct, %mystruct* %st, i32 0, i32 0
store i32 1, i32* %r2

```

**Type Conversion**

- data truncation: trunc
- zero extension: zext
```
%r2 = trunc i8 %r1, i1
%r3 = zext i1 %r2, i8
```

**Function Definition**
```
@define i32 @bar(i32 %r0){
    ...
    ret i32 %r1;
}
```


### Function Call
```
%2 = call i32 @test(i32 %1)
```

### Binary Operations
**Integer arithmetic** 
```
%r3 = add i32 %r2, 1
%r4 = sub i32 %r3, 2
%r5 = mul i32 %r3, 3
%r6 = sdiv i32 %r4, 4
```

### Comparison
```
%5 = icmp sgt i32 %4, 0
%6 = icmp sge i32 %4, 0
%7 = icmp slt i32 %4, 0
%8 = icmp sle i32 %4, 0
%9 = icmp eq i32 %4, 0
%10 = icmp ne i32 %4, 0
```

### Logical Operation
**Not**
```
%7 = xor i1 %6, true
```

### Control
**Direct Jump**
```
 br label %bb3
```

**Conditional Jump**
```
  %4 = icmp sgt i32 %3, 0
  br i1 %4, label %bb1, label %bb2
bb1: 
  
bb2:
```

**Conditional Data Flow**
```
bb1: 

bb2:

bb3:
  %r0 = phi i32 [ r2, %bb1 ], [ %r3, %bb2 ]
```



