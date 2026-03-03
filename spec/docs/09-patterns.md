# 第9章 模式匹配

## 9.1 模式语法

### 模式分类
```
Pattern ::= LiteralPattern
          | VariablePattern
          | WildcardPattern
          | ConstructorPattern
          | TuplePattern
          | RecordPattern
          | OrPattern
          | GuardedPattern
          | AsPattern
```

**执行说明：**

1. **模式匹配用途**：
   - `when` 表达式/语句中的分支匹配
   - `let` 绑定中的解构
   - `for` 循环中的解构
   - 参数中的解构

---

## 9.2 字面量模式

### 字面量模式语法
```
LiteralPattern ::= IntegerLiteral
                 | FloatLiteral
                 | BooleanLiteral
                 | StringLiteral
                 | CharLiteral
                 | 'null' | 'none'
```

**执行说明：**
- 匹配等于该字面量的值
- 要求被匹配值与字面量类型相同

### 字面量匹配规则
```
matches(v, lit) = (v == ⟦lit⟧)
```

---

## 9.3 变量模式

### 变量模式语法
```
VariablePattern ::= Identifier
                  | 'mut' Identifier
```

**执行说明：**
- 匹配任何值
- 将值绑定到标识符
- `mut` 表示可变绑定

### 变量匹配规则
```
matches(v, x) = true
bindings(v, x) = {x ↦ v}

matches(v, mut x) = true
bindings(v, mut x) = {x ↦ mut v}
```

---

## 9.4 通配符模式

### 通配符模式语法
```
WildcardPattern ::= '_'
```

**执行说明：**
- 匹配任何值
- 不创建绑定
- 用于表示不关心的值

### 通配符匹配规则
```
matches(v, _) = true
bindings(v, _) = ∅
```

---

## 9.5 构造函数模式

### 构造函数模式语法
```
ConstructorPattern ::= Identifier '(' (Pattern (',' Pattern)*)? ')'
                     | 'some' '(' Pattern ')'
                     | 'ok' '(' Pattern ')'
                     | 'err' '(' Pattern ')'
```

**执行说明：**

1. **Option类型**：
   - `some(p)`：匹配 `Some(v)`，其中 `v` 匹配 `p`
   - `none`：匹配 `None`

2. **Result类型**：
   - `ok(p)`：匹配 `Ok(v)`，其中 `v` 匹配 `p`
   - `err(p)`：匹配 `Err(e)`，其中 `e` 匹配 `p`

3. **代数数据类型**：
   - 匹配自定义类型的构造函数

### 构造函数匹配规则
```
v = C(v₁, ..., vₙ)
matches(v₁, p₁) ... matches(vₙ, pₙ)
────────────────────────────────
matches(v, C(p₁, ..., pₙ)) = true

v = some(v₁)
matches(v₁, p)
────────────────
matches(v, some(p)) = true

v = ok(v₁)
matches(v₁, p)
──────────────
matches(v, ok(p)) = true

v = err(e₁)
matches(e₁, p)
───────────────
matches(v, err(p)) = true
```

---

## 9.6 元组与记录模式

### 元组模式语法
```
TuplePattern ::= '(' (Pattern (',' Pattern)*)? ')'
```

**执行说明：**
- 匹配元组值
- 每个位置的子模式匹配对应位置的元素

### 记录模式语法
```
RecordPattern ::= '{' (FieldPattern (',' FieldPattern)*)? '}'

FieldPattern ::= Identifier (':' Pattern)?
               | Identifier '=' Pattern
```

**执行说明：**
- 匹配记录值
- 按字段名匹配
- 字段顺序不重要
- 可以省略不关心的字段

### 元组与记录匹配规则
```
v = (v₁, ..., vₙ)
matches(v₁, p₁) ... matches(vₙ, pₙ)
────────────────────────────────
matches(v, (p₁, ..., pₙ)) = true

v = { l₁: v₁, ..., lₙ: vₙ }
matches(vᵢ, pᵢ) for each field in pattern
────────────────────────────────
matches(v, { l₁: p₁, ..., lₖ: pₖ }) = true
```

---

## 9.7 组合模式

### Or模式语法
```
OrPattern ::= Pattern '|' Pattern
```

**执行说明：**
- 匹配任一子模式
- 两个子模式必须绑定相同的变量

### Guarded模式语法
```
GuardedPattern ::= Pattern 'if' Expression
```

**执行说明：**
- 首先匹配模式
- 然后计算守卫条件（必须是布尔类型）
- 仅当两者都为真时匹配成功

### As模式语法
```
AsPattern ::= Pattern 'as' Identifier
```

**执行说明：**
- 匹配子模式
- 同时将整个值绑定到标识符

### 组合模式匹配规则
```
matches(v, p₁) ∨ matches(v, p₂)
FV(p₁) = FV(p₂)
────────────────────
matches(v, p₁ | p₂) = true

matches(v, p)
⟦e⟧ᵍ⁺ᵇⁱⁿᵈⁱⁿᵍˢ = true
────────────────────────
matches(v, p if e) = true

matches(v, p)
──────────────────
matches(v, p as x) = true
bindings(v, p as x) = bindings(v, p) ∪ {x ↦ v}
```

---

## 9.8 类型测试模式

### Type模式语法
```
TypePattern ::= Pattern 'is' Type
```

**执行说明：**
- 测试值的类型
- 如果类型匹配，则继续匹配子模式
- 通常用于向下转型

### Type匹配规则
```
typeof(v) = T
matches(v, p)
────────────────────
matches(v, p is T) = true
```

---

## 9.9 穷尽性检查

### 穷尽性规则
```
exhaustive(patterns, type): Bool
  = forall v: type, exists p ∈ patterns, matches(v, p)
```

**执行说明：**
- 编译器检查模式匹配是否覆盖所有可能情况
- 如果不能证明穷尽性，发出警告或错误
- 可以用 `_` 通配符处理剩余情况

---

**本章规范采用数学语言定义模式匹配语法，自然语言描述执行语义。**
