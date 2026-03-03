# 第3章 表达式

## 3.1 表达式语法

### 表达式语法定义
```
Expression → Literal
           | Variable
           | MemberAccess
           | FunctionCall
           | BinaryOperation
           | UnaryOperation
           | IfExpression
           | Lambda
           | Array
           | Dictionary
           | Record
           | Range
           | Pipe
           | Parenthesized
```

### 优先级与结合性
```
Precedence: 1 (highest) → 12 (lowest)

12: IfExpression, Lambda
11: Pipe
10: Range
 9: LogicalOr (or)
 8: LogicalAnd (and)
 7: Equality (==, !=)
 6: Comparison (<, >, <=, >=)
 5: BitwiseOr (|), BitwiseXor (^)
 4: BitwiseAnd (&)
 3: Shift (<<, >>)
 2: Additive (+, -)
 1: Multiplicative (*, /, %)

Associativity:
  Left: +, -, *, /, %, |, &, ^, <<, >>, |>
  Right: =, +=, -=, *=, /=, %=, ^=
```

## 3.2 字面量表达式

### 整数字面量
```
Literal ::= IntegerLiteral
Evaluation:
  ⟦n⟧ = n  where n ∈ ℤ
```

### 浮点数字面量
```
Literal ::= FloatLiteral
Evaluation:
  ⟦f⟧ = f  where f ∈ ℝ
```

### 布尔字面量
```
Literal ::= 'true' | 'false'
Evaluation:
  ⟦true⟧ = true
  ⟦false⟧ = false
```

### 字符串字面量
```
Literal ::= StringLiteral
Evaluation:
  ⟦"s"⟧ = s  where s ∈ Σ*
```

### 空字面量
```
Literal ::= 'null' | 'none'
Evaluation:
  ⟦null⟧ = null
  ⟦none⟧ = none
```

### 字面量求值规则
```
⟦l⟧ᵥ = v  where v is the value of literal l
```

## 3.3 变量表达式

### 变量引用
```
Expression ::= Identifier
Evaluation:
  ⟦x⟧ᵍ = g(x)  where g is the environment
```

### 环境查找
```
g: Identifier → Value
g(x) = v  if x is bound to v in g
```

### 自由变量
```
FV(x) = {x}
FV(e₁ op e₂) = FV(e₁) ∪ FV(e₂)
FV(λx.e) = FV(e) \ {x}
```

## 3.4 函数调用表达式

### 函数调用语法
```
Expression ::= Expression '(' (Expression (',' Expression)*)? ')'
```

### 函数调用求值
```
⟦f(e₁, ..., eₙ)⟧ᵍ = v
  where
    f' = ⟦f⟧ᵍ
    v₁ = ⟦e₁⟧ᵍ
    ...
    vₙ = ⟦eₙ⟧ᵍ
    v = apply(f', v₁, ..., vₙ)
```

### 函数应用
```
apply: (Value × Value*) → Value
apply(closure, v₁, ..., vₙ) = ⟦body⟧ᵍ[x₁↦v₁, ..., xₙ↦vₙ]
  where closure = (x₁, ..., xₙ) → body, g
```

## 3.5 二元运算表达式

### 二元运算语法
```
Expression ::= Expression BinaryOp Expression
BinaryOp ::= '+' | '-' | '*' | '/' | '%'
           | '==' | '!=' | '<' | '>' | '<=' | '>='
           | '&&' | '||'
           | '&' | '|' | '^' | '<<' | '>>'
```

### 算术运算
```
⟦e₁ + e₂⟧ᵍ = ⟦e₁⟧ᵍ + ⟦e₂⟧ᵍ
⟦e₁ - e₂⟧ᵍ = ⟦e₁⟧ᵍ - ⟦e₂⟧ᵍ
⟦e₁ * e₂⟧ᵍ = ⟦e₁⟧ᵍ × ⟦e₂⟧ᵍ
⟦e₁ / e₂⟧ᵍ = ⟦e₁⟧ᵍ ÷ ⟦e₂⟧ᵍ
⟦e₁ % e₂⟧ᵍ = ⟦e₁⟧ᵍ mod ⟦e₂⟧ᵍ
```

### 比较运算
```
⟦e₁ == e₂⟧ᵍ = (⟦e₁⟧ᵍ = ⟦e₂⟧ᵍ)
⟦e₁ != e₂⟧ᵍ = (⟦e₁⟧ᵍ ≠ ⟦e₂⟧ᵍ)
⟦e₁ < e₂⟧ᵍ = (⟦e₁⟧ᵍ < ⟦e₂⟧ᵍ)
⟦e₁ > e₂⟧ᵍ = (⟦e₁⟧ᵍ > ⟦e₂⟧ᵍ)
⟦e₁ <= e₂⟧ᵍ = (⟦e₁⟧ᵍ ≤ ⟦e₂⟧ᵍ)
⟦e₁ >= e₂⟧ᵍ = (⟦e₁⟧ᵍ ≥ ⟦e₂⟧ᵍ)
```

### 逻辑运算
```
⟦e₁ && e₂⟧ᵍ = ⟦e₁⟧ᵍ ∧ ⟦e₂⟧ᵍ
⟦e₁ || e₂⟧ᵍ = ⟦e₁⟧ᵍ ∨ ⟦e₂⟧ᵍ

Short-circuit:
  ⟦e₁ && e₂⟧ᵍ = if ⟦e₁⟧ᵍ = false then false else ⟦e₂⟧ᵍ
  ⟦e₁ || e₂⟧ᵍ = if ⟦e₁⟧ᵍ = true then true else ⟦e₂⟧ᵍ
```

## 3.6 一元运算表达式

### 一元运算语法
```
Expression ::= UnaryOp Expression
UnaryOp ::= '-' | '!' | '~'
```

### 一元运算求值
```
⟦-e⟧ᵍ = -⟦e⟧ᵍ
⟦!e⟧ᵍ = ¬⟦e⟧ᵍ
⟦~e⟧ᵍ = ~⟦e⟧ᵍ
```

## 3.7 条件表达式

### 条件表达式语法
```
Expression ::= 'if' Expression 'then' Expression 'else' Expression
```

### 条件表达式求值
```
⟦if e₁ then e₂ else e₃⟧ᵍ =
  if ⟦e₁⟧ᵍ = true then ⟦e₂⟧ᵍ else ⟦e₃⟧ᵍ
```

## 3.8 数组表达式

### 数组语法
```
Expression ::= '[' (Expression (',' Expression)*)? ']'
```

### 数组求值
```
⟦[e₁, e₂, ..., eₙ]⟧ᵍ = [⟦e₁⟧ᵍ, ⟦e₂⟧ᵍ, ..., ⟦eₙ⟧ᵍ]
```

## 3.9 成员访问表达式

### 成员访问语法
```
Expression ::= Expression '.' Identifier
```

### 成员访问求值
```
⟦e.l⟧ᵍ = get_member(⟦e⟧ᵍ, l)

get_member: Record × Identifier → Value
get_member({l₁: v₁, ..., lₙ: vₙ}, lᵢ) = vᵢ
```

## 3.10 管道表达式

### 管道语法
```
Expression ::= Expression '|>' Expression
```

### 管道求值
```
⟦e₁ |> e₂⟧ᵍ = ⟦e₂(e₁)⟧ᵍ
⟦e₁ |> e₂ |> e₃⟧ᵍ = ⟦(e₁ |> e₂) |> e₃⟧ᵍ
```

## 3.11 括号表达式

### 括号语法
```
Expression ::= '(' Expression ')'
```

### 括号求值
```
⟦(e)⟧ᵍ = ⟦e⟧ᵍ
```

---

**本章规范采用数学语言定义表达式，简洁清晰明了。**
