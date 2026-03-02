# 第5章 函数

## 5.1 函数声明

### 函数语法
```
FunctionDeclaration ::= 'fun' Identifier Parameters (':' Type)? FunctionBody

Parameters ::= '(' (Parameter (',' Parameter)*)? ')'

Parameter ::= Identifier (':' Type)? ('=' Expression)?

FunctionBody ::= '=' Expression ';'
               | Block
```

**执行说明：**

1. **函数声明**：
   - 在当前作用域中创建一个函数绑定
   - 函数名是一个标识符，遵循标识符命名规则
   - 参数列表定义函数的输入
   - 返回类型可以显式声明，也可以从函数体推断

2. **参数**：
   - 每个参数有一个名称和可选的类型注解
   - 参数可以有默认值，调用时可以省略该参数
   - 有默认值的参数必须放在参数列表的后面

3. **函数体**：
   - 简洁形式：`= expression`，直接返回表达式的值
   - 块形式：`{ statements }`，执行块内语句，通过 `return` 返回值

### 函数类型规则
```
Γ, x₁: T₁, ..., xₙ: Tₙ ⊢ e: R
────────────────────────────────
Γ ⊢ fun f(x₁: T₁, ..., xₙ: Tₙ) = e: (T₁, ..., Tₙ) → R
```

---

## 5.2 匿名函数（Lambda）

### Lambda语法
```
Lambda ::= Parameters '->' Expression
         | Parameters '=>' Expression
```

**执行说明：**
- 创建一个匿名函数值
- 可以捕获周围作用域的变量（闭包）
- 捕获的变量在 lambda 创建时被绑定

### Lambda求值规则
```
⟦(x₁, ..., xₙ) → e⟧ᵍ = closure((x₁, ..., xₙ), e, g)

apply(closure(params, body, env), v₁, ..., vₙ) =
  ⟦body⟧ᵉⁿᵛ[x₁↦v₁, ..., xₙ↦vₙ]
```

---

## 5.3 函数调用

### 函数调用语法
```
FunctionCall ::= Expression '(' (Argument (',' Argument)*)? ')'

Argument ::= Expression
           | Identifier '=' Expression
```

**执行说明：**

1. **位置参数**：
   - 按顺序与函数参数匹配
   - 数量和类型必须兼容

2. **命名参数**：
   - 通过参数名指定，可以任意顺序
   - 必须在位置参数之后使用

3. **默认参数**：
   - 调用时可以省略有默认值的参数
   - 省略时使用声明时的默认值

### 函数调用求值规则
```
⟦f(e₁, ..., eₙ)⟧ᵍ = v
  where
    f_val = ⟦f⟧ᵍ
    v₁ = ⟦e₁⟧ᵍ
    ...
    vₙ = ⟦eₙ⟧ᵍ
    v = apply(f_val, v₁, ..., vₙ)
```

---

## 5.4 高阶函数

### 函数作为参数
```
Definition:
A higher-order function is a function that takes another function as
a parameter or returns a function as a result.
```

**执行说明：**
- 函数可以作为参数传递给其他函数
- 函数可以作为返回值从函数返回
- 支持柯里化（Currying）和部分应用

### 高阶函数示例规则
```
⟦map(f, [])⟧ᵍ = []
⟦map(f, x:xs)⟧ᵍ = ⟦f(x)⟧ᵍ : ⟦map(f, xs)⟧ᵍ

⟦compose(f, g)(x)⟧ᵍ = ⟦f(g(x))⟧ᵍ
```

---

## 5.5 递归函数

### 递归声明
```
RecursiveFunction ::= 'fun' 'rec'? Identifier Parameters FunctionBody
```

**执行说明：**
- 函数可以在其体内引用自身
- `rec` 关键字是可选的，主要用于明确表示递归
- 递归必须有终止条件，否则会无限递归

### 递归求值规则
```
⟦let rec f = λx. e⟧ᵍ = (), g'
  where g' = g[f ↦ fix(λf. λx. e)]

fix(F) = F(fix(F))  // Y combinator
```

---

## 5.6 多态函数

### 类型参数语法
```
TypeParameters ::= '[' TypeParameter (',' TypeParameter)* ']'

TypeParameter ::= Identifier (':' TypeConstraint)?
```

**执行说明：**
- 函数可以有类型参数，实现泛型编程
- 类型参数在调用时根据实参类型推断，或显式指定
- 类型约束限制类型参数必须满足的条件

### 多态函数类型规则
```
Γ, α₁, ..., αₙ, x₁: T₁, ..., xₘ: Tₘ ⊢ e: R
────────────────────────────────────────
Γ ⊢ fun f[α₁, ..., αₙ](x₁: T₁, ..., xₘ: Tₘ) = e:
     ∀α₁...αₙ. (T₁, ..., Tₘ) → R
```

---

## 5.7 效果系统

### 效果注解语法
```
EffectAnnotation ::= 'throws' EffectList

EffectList ::= Effect (',' Effect)*
```

**执行说明：**
- 函数可以声明可能产生的效果（异常、IO等）
- 调用者必须处理或声明传播这些效果
- 效果检查在编译时进行

### 效果类型规则
```
Γ ⊢ f: (T₁, ..., Tₙ) → R ⟨E₁, ..., Eₖ⟩
Γ ⊢ e₁: T₁  ...  Γ ⊢ eₙ: Tₙ
────────────────────────────────
Γ ⊢ f(e₁, ..., eₙ): R ⟨E₁, ..., Eₖ⟩
```

---

## 5.8 闭包与环境捕获

### 闭包定义
```
Closure = (Parameters × Body × Environment)

Environment = Identifier → Value
```

**执行说明：**
- 闭包捕获定义时的环境
- 捕获的变量保持与外部作用域的关联
- 可变变量的修改在闭包内外都可见

### 闭包捕获规则
```
FV(λx.e) = FV(e) \ {x}

capture(g, vars) = { x ↦ g(x) | x ∈ vars }

⟦λx.e⟧ᵍ = closure((x), e, capture(g, FV(λx.e)))
```

---

**本章规范采用数学语言定义函数语法，自然语言描述执行语义。**
