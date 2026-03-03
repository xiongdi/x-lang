# 第4章 语句

## 4.1 语句语法

### 语句分类
```
Statement → Declaration
           | ExpressionStatement
           | Assignment
           | IfStatement
           | WhileStatement
           | ForStatement
           | ReturnStatement
           | Block
           | TryStatement
           | WhenStatement
```

### 表达式语句
```
ExpressionStatement ::= Expression ';'
```

**执行说明：**
计算表达式的值，然后丢弃结果。仅用于表达式的副作用。

---

## 4.2 声明语句

### 变量声明语法
```
Declaration ::= 'let' Identifier (':' Type)? ('=' Expression)? ';'
              | 'let' 'mut' Identifier (':' Type)? ('=' Expression)? ';'
```

**执行说明：**

1. **不可变绑定 (`let`)**：
   - 在当前作用域中创建一个新的不可变绑定
   - 如果提供了初始化表达式，则计算并将值绑定到标识符
   - 后续不能重新赋值给该标识符

2. **可变绑定 (`let mut`)**：
   - 在当前作用域中创建一个新的可变绑定
   - 如果提供了初始化表达式，则计算并将值绑定到标识符
   - 后续可以通过赋值语句修改该绑定的值

3. **类型注解**：
   - 如果提供了类型注解，则初始化表达式的类型必须与注解类型兼容
   - 如果没有提供类型注解，则通过初始化表达式推断类型

### 变量声明求值规则
```
⟦let x: T = e⟧ᵍ = (), g'
  where
    v = ⟦e⟧ᵍ
    g' = g[x ↦ v]

⟦let mut x: T = e⟧ᵍ = (), g'
  where
    v = ⟦e⟧ᵍ
    g' = g[x ↦ mut v]
```

---

## 4.3 赋值语句

### 赋值语法
```
Assignment ::= LValue '=' Expression ';'
            | LValue AssignmentOp Expression ';'

AssignmentOp ::= '+=' | '-=' | '*=' | '/=' | '%=' | '^='

LValue ::= Identifier
         | LValue '.' Identifier
         | LValue '[' Expression ']'
```

**执行说明：**

1. **简单赋值 (`=`)**：
   - 计算右侧表达式的值
   - 将值存储到左侧的左值位置
   - 左侧必须是可变绑定

2. **复合赋值 (`op=`)**：
   - 读取左值的当前值
   - 与右侧表达式的值进行指定的运算
   - 将结果存回左值位置
   - 等价于 `lvalue = lvalue op expression`

### 赋值求值规则
```
⟦l = e⟧ᵍ = (), g'
  where
    v = ⟦e⟧ᵍ
    g' = g[l ↦ v]

⟦l += e⟧ᵍ = ⟦l = l + e⟧ᵍ
⟦l -= e⟧ᵍ = ⟦l = l - e⟧ᵍ
⟦l *= e⟧ᵍ = ⟦l = l * e⟧ᵍ
⟦l /= e⟧ᵍ = ⟦l = l / e⟧ᵍ
⟦l %= e⟧ᵍ = ⟦l = l % e⟧ᵍ
```

---

## 4.4 块语句

### 块语法
```
Block ::= '{' Statement* '}'
```

**执行说明：**
- 创建一个新的词法作用域
- 按顺序执行块内的所有语句
- 块内声明的变量在块结束后超出作用域
- 块的值是最后一个表达式语句的值（如果有），否则是单位值 `()`

### 块求值规则
```
⟦{ s₁; s₂; ...; sₙ }⟧ᵍ = v, gₙ
  where
    (_, g₁) = ⟦s₁⟧ᵍ
    (_, g₂) = ⟦s₂⟧ᵍ₁
    ...
    (v, gₙ) = ⟦sₙ⟧ᵍₙ₋₁
```

---

## 4.5 条件语句

### If语句语法
```
IfStatement ::= 'if' Expression Block ('else' (Block | IfStatement))?
```

**执行说明：**
- 计算条件表达式的值（必须是布尔类型）
- 如果为 `true`，执行 `then` 分支的块
- 如果为 `false`，执行 `else` 分支（如果存在）
- 可以链式使用 `else if` 进行多分支判断

### If语句求值规则
```
⟦if e then b₁ else b₂⟧ᵍ =
  if ⟦e⟧ᵍ = true then ⟦b₁⟧ᵍ else ⟦b₂⟧ᵍ

⟦if e then b⟧ᵍ =
  if ⟦e⟧ᵍ = true then ⟦b⟧ᵍ else (), g
```

---

## 4.6 循环语句

### While语句语法
```
WhileStatement ::= 'while' Expression Block
```

**执行说明：**
- 重复执行：
  1. 计算条件表达式（必须是布尔类型）
  2. 如果为 `true`，执行循环体，然后回到步骤1
  3. 如果为 `false`，退出循环
- 条件在每次迭代前检查

### While语句求值规则
```
⟦while e do b⟧ᵍ = ⟦loop(e, b)⟧ᵍ
  where
    loop(e, b) =
      if ⟦e⟧ᵍ = true
      then let (_, g') = ⟦b⟧ᵍ in loop(e, b) with g'
      else (), g
```

### For语句语法
```
ForStatement ::= 'for' Identifier 'in' Expression Block
```

**执行说明：**
- 遍历可迭代对象（如数组、范围）
- 对于每个元素：
  - 将元素绑定到循环变量
  - 执行循环体
- 循环变量在每次迭代结束后重新绑定

### For语句求值规则
```
⟦for x in e do b⟧ᵍ = ⟦loop(xs, x, b)⟧ᵍ
  where
    xs = ⟦e⟧ᵍ
    loop([], x, b) = (), g
    loop(v:vs, x, b) =
      let g' = g[x ↦ v] in
      let (_, g'') = ⟦b⟧ᵍ' in
      loop(vs, x, b) with g''
```

---

## 4.7 返回语句

### Return语句语法
```
ReturnStatement ::= 'return' Expression? ';'
```

**执行说明：**
- 立即从当前函数返回
- 如果提供了表达式，则计算其值作为返回值
- 如果没有表达式，返回单位值 `()`
- 跳过后续所有语句

### Return语句求值规则
```
⟦return e⟧ᵍ = Return(v)
  where v = ⟦e⟧ᵍ

⟦return⟧ᵍ = Return(())
```

---

## 4.8 When语句（模式匹配）

### When语句语法
```
WhenStatement ::= 'when' Expression '{' WhenBranch* '}'

WhenBranch ::= Pattern ('if' Expression)? '=>' Block
```

**执行说明：**
- 按顺序尝试将 scrutinee 与每个分支的模式匹配
- 第一个匹配成功的分支被执行
- 如果分支有 `if` 守卫，则守卫条件必须也为 `true`
- 至少需要一个分支匹配，否则抛出运行时错误

### When语句求值规则
```
⟦when e of { p₁ => b₁; p₂ => b₂; ... }⟧ᵍ =
  let v = ⟦e⟧ᵍ in
  match v with
  | p₁ → ⟦b₁⟧ᵍ₁ where g₁ = g + bindings(p₁, v)
  | p₂ → ⟦b₂⟧ᵍ₂ where g₂ = g + bindings(p₂, v)
  | ...
```

---

## 4.9 Try语句（异常处理）

### Try语句语法
```
TryStatement ::= 'try' Block ('catch' Pattern '=>' Block)* ('finally' Block)?
```

**执行说明：**
- 执行 `try` 块
- 如果 `try` 块正常完成，继续执行
- 如果 `try` 块抛出异常：
  - 按顺序尝试匹配 `catch` 分支
  - 第一个匹配的 `catch` 块被执行
- `finally` 块总是在最后执行，无论是否发生异常

### Try语句求值规则
```
⟦try b catch p => c finally f⟧ᵍ =
  let result = ⟦b⟧ᵍ in
  match result with
  | Ok(v, g') → let (_, g'') = ⟦f⟧ᵍ' in (v, g'')
  | Err(e, g') →
      if matches(p, e)
      then let (v, g'') = ⟦c⟧ᵍ'[p↦e] in
           let (_, g''') = ⟦f⟧ᵍ'' in (v, g''')
      else let (_, g'') = ⟦f⟧ᵍ' in Err(e, g'')
```

---

## 4.10 作用域与生命周期

### 作用域规则
```
Scope ::= GlobalScope | LocalScope(Scope)

lookup: Scope × Identifier → Value
lookup(LocalScope(parent), x) =
  if x defined in LocalScope
  then get(LocalScope, x)
  else lookup(parent, x)
```

**执行说明：**
- 变量绑定遵循词法作用域
- 内部作用域可以 shadow（遮蔽）外部作用域的同名变量
- 变量在声明点之前不可访问
- 块结束时，块内声明的变量被销毁

---

**本章规范采用数学语言定义语句语法，自然语言描述执行语义。**
