# 第10章 内存管理

## 10.1 Perceus 内存管理

### 定义
```
Perceus: Compile-time reference counting + reuse analysis
```

**执行说明：**

Perceus 是一种编译时内存管理技术：
- 在编译时插入 `dup`（复制）和 `drop`（释放）操作
- 进行重用分析，尽可能重用现有对象而不是分配新对象
- 无垃圾收集器，无运行时开销
- 线程安全

---

## 10.2 所有权与借用

### 所有权规则
```
OwnershipState ::= Owned | Borrowed | Moved

owns(variable): Bool = state(variable) == Owned
```

**执行说明：**

1. **所有权**：
   - 每个值有且仅有一个所有者
   - 所有者负责释放值
   - 当所有者超出作用域，值被释放

2. **移动语义**：
   - 赋值默认移动所有权
   - 原所有者变为 `Moved` 状态
   - 不能再使用已移动的变量

3. **复制**：
   - 使用 `dup` 显式复制值
   - 复制后两个所有者各自独立

### 所有权规则示例
```
let x = [1, 2, 3]  // x owns the array
let y = x            // ownership moves to y
// x can no longer be used

let z = dup y        // duplicate the array
// y and z both own separate arrays
```

---

## 10.3 编译时插入的操作

### Dup 操作
```
dup: Value → Value
dup(v) = v'  where v' is a fresh copy of v
```

**执行说明：**
- 当值需要被多个所有者使用时插入 `dup`
- 复制发生在编译时确定的位置
- 对于不可变值可以优化为引用计数递增

### Drop 操作
```
drop: Value → Unit
drop(v) = ()  where v is deallocated
```

**执行说明：**
- 当值的所有者超出作用域时插入 `drop`
- 释放值占用的内存
- 递归释放包含的子对象

### Dup/Drop 插入规则
```
For each variable x:
  if x is used after a move:
    insert dup(x) before the use
  at end of scope:
    if x is still owned:
      insert drop(x)
```

---

## 10.4 重用分析

### 重用定义
```
ReuseOpportunity = AllocationSite × DropSite

reuse(v, alloc) = v'  where v' is v reused instead of new allocation
```

**执行说明：**

1. **重用机会识别**：
   - 查找 `drop` 后紧跟同类型 `alloc` 的位置
   - 验证对象可以安全重用（类型兼容、无别名等）

2. **重用转换**：
   - 将 `drop(x); let y = alloc(T)` 转换为 `let y = reuse(x)`
   - 必要时重置对象内容

### 重用规则
```
Pattern:
  drop(x);
  let y = T(v₁, ..., vₙ);

Transform to (if x: T and no aliases):
  let y = reset(x, v₁, ..., vₙ);
```

---

## 10.5 线性类型

### 线性类型规则
```
LinearValue: must be used exactly once
AffineValue: can be used at most once
UnrestrictedValue: can be used any number of times
```

**执行说明：**
- 线性类型确保资源不泄漏也不重复使用
- X语言使用仿射类型（Affine）作为默认
- 可通过 `drop` 提前释放

---

## 10.6 安全保证

### 内存安全定理
```
Theorem (No Use-After-Free):
  For all programs that typecheck:
    there is no access to a dropped value

Theorem (No Double-Free):
  For all programs that typecheck:
    no value is dropped more than once

Theorem (No Memory Leaks - partial):
  For all programs that typecheck:
    all owned values are dropped exactly once
    (cyclic data structures may require manual handling)
```

**执行说明：**
编译时检查确保：
- 不会使用已释放的值
- 不会重复释放
- 不会泄漏内存（循环引用除外）

---

## 10.7 循环引用处理

### 弱引用
```
WeakRef(T): weak reference to T
  does not keep T alive
  can be upgraded to Option<Ref(T)>
```

**执行说明：**
- 使用弱引用打破循环
- 弱引用不会阻止对象被释放
- 访问前需要检查是否仍有效

---

**本章规范采用数学语言定义内存管理规则，自然语言描述执行语义。**

---

## 文档完成总结

已创建完整的分章节语言规范文档：

| 章节 | 主题 | 特点 |
|------|------|------|
| 01-lexical.md | 词法结构 | 数学语言定义 |
| 02-types.md | 类型系统 | 数学语言定义 |
| 03-expressions.md | 表达式 | 数学语言定义 |
| 04-statements.md | 语句 | 数学语言+自然语言 |
| 05-functions.md | 函数 | 数学语言+自然语言 |
| 06-classes.md | 面向对象 | 数学语言+自然语言 |
| 07-effects.md | 效果系统 | 数学语言+自然语言 |
| 08-modules.md | 模块系统 | 数学语言+自然语言 |
| 09-patterns.md | 模式匹配 | 数学语言+自然语言 |
| 10-memory.md | 内存管理 | 数学语言+自然语言 |

**总体原则**：定式用数学语言，简洁清晰明了；执行用自然语言，易读易懂。
