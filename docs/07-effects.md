# 第7章 效果系统

## 7.1 效果定义

### 效果语法
```
Effect ::= EffectName
          | EffectName '[' Type (',' Type)* ']'

EffectName ::= 'throws' | 'io' | 'state' | 'async' | 'atomic'
             | Identifier  // 用户自定义效果
```

**执行说明：**

1. **效果系统**：
   - 显式声明函数可能产生的副作用
   - 编译器检查效果是否被正确处理
   - 支持效果多态和效果处理

2. **内置效果**：
   - `throws E`：可能抛出异常类型 E
   - `io`：执行输入输出操作
   - `state S`：读写状态 S
   - `async`：异步执行
   - `atomic`：原子操作

---

## 7.2 效果注解

### 效果注解语法
```
FunctionType ::= '(' (Type (',' Type)*)? ')' '->' Type EffectSet?

EffectSet ::= '⟨' Effect (',' Effect)* '⟩'
            | 'where' EffectConstraint (',' EffectConstraint)*
```

**执行说明：**
- 函数类型可以携带效果集合
- 表示调用该函数可能产生的所有效果
- 效果必须被处理或传播

### 效果类型规则
```
Γ ⊢ f: (T₁, ..., Tₙ) → R ⟨E₁, ..., Eₖ⟩
Γ ⊢ e₁: T₁  ...  Γ ⊢ eₙ: Tₙ
────────────────────────────────
Γ ⊢ f(e₁, ..., eₙ): R ⟨E₁, ..., Eₖ⟩
```

---

## 7.3 异常效果

### 异常声明语法
```
FunctionDeclaration ::= 'fun' Identifier Parameters (':' Type)?
                         ('throws' Type (',' Type)*)? FunctionBody

ThrowStatement ::= 'throw' Expression ';'

TryStatement ::= 'try' Block ('catch' Pattern '=>' Block)* ('finally' Block)?
```

**执行说明：**

1. **抛出异常**：
   - 使用 `throw` 语句抛出异常值
   - 异常类型必须在函数的 `throws` 列表中声明

2. **捕获异常**：
   - 使用 `try-catch` 语句捕获异常
   - 可以按类型模式匹配多个异常
   - `finally` 块总是执行

### 异常效果规则
```
Γ ⊢ e: E
────────────────────────────
Γ ⊢ throw e: T ⟨throws E⟩

Γ ⊢ b: R ⟨throws E₁, ..., Eₙ⟩
Γ, x: Eᵢ ⊢ cᵢ: R ⟨Δ⟩  for each i
────────────────────────────────
Γ ⊢ try b catch Eᵢ => cᵢ: R ⟨Δ⟩
```

---

## 7.4 需求与给定（Needs/Given）

### Needs/Given语法
```
FunctionDeclaration ::= 'fun' Identifier Parameters
                         ('needs' Effect (',' Effect)*)?
                         ('given' Effect (',' Effect)*)?
                         (':' Type)? FunctionBody

EffectApplication ::= 'given' Effect '=' Expression 'in' Block
```

**执行说明：**

1. **Needs**：
   - 声明函数需要的效果环境
   - 调用者必须提供这些效果

2. **Given**：
   - 声明函数提供的效果处理
   - 可以为被调用者提供效果环境

### Needs/Given规则
```
Γ, needs Δ₁, given Δ₂ ⊢ e: R ⟨Δ₃⟩
Δ₂ ⊇ Δ₃
────────────────────────────────
Γ ⊢ fun f() needs Δ₁ given Δ₂ = e:
     () → R ⟨Δ₁⟩

Γ ⊢ handler: Handler(E)
Γ, given E ⊢ b: R ⟨∅⟩
────────────────────────────
Γ ⊢ given E = handler in b: R ⟨∅⟩
```

---

## 7.5 异步效果

### Async语法
```
AsyncFunction ::= 'async' 'fun' Identifier Parameters (':' Type)? FunctionBody

AwaitExpression ::= 'wait' Expression

TogetherExpression ::= 'together' '{' (Identifier '=' Expression)* '}'

RaceExpression ::= 'race' '{' (Identifier '=' Expression)* '}'

TimeoutExpression ::= 'timeout' Expression 'after' Expression
```

**执行说明：**

1. **Async函数**：
   - 标记为 `async` 的函数返回一个异步任务
   - 可以在内部使用 `wait` 等待其他异步操作

2. **并发组合**：
   - `together`：并行等待所有任务完成
   - `race`：等待任一任务完成
   - `timeout`：设置超时时间

### Async效果规则
```
Γ ⊢ e: T ⟨async⟩
──────────────────
Γ ⊢ wait e: T

Γ ⊢ e₁: T₁ ⟨async⟩  ...  Γ ⊢ eₙ: Tₙ ⟨async⟩
────────────────────────────────────────
Γ ⊢ together { x₁ = e₁, ..., xₙ = eₙ }:
     (T₁, ..., Tₙ) ⟨async⟩

Γ ⊢ e: T ⟨async⟩
Γ ⊢ timeout: Duration
────────────────────────────
Γ ⊢ timeout e after timeout:
     Option<T> ⟨async⟩
```

---

## 7.6 原子效果

### Atomic语法
```
AtomicBlock ::= 'atomic' Block

RetryStatement ::= 'retry' Expression? ';'
```

**执行说明：**

1. **原子块**：
   - `atomic` 块内的操作作为一个原子事务执行
   - 要么全部成功，要么全部回滚
   - 可以使用 `retry` 重试整个原子块

2. **STM（软件事务内存）**：
   - 原子读取和写入共享状态
   - 冲突检测和自动重试
   - 死锁免

### Atomic效果规则
```
Γ ⊢ b: T ⟨state S⟩
────────────────────
Γ ⊢ atomic b: T ⟨atomic⟩

Γ, in_atomic ⊢ retry: Never ⟨atomic⟩
```

---

## 7.7 效果处理

### 效果处理语法
```
EffectHandler ::= 'handle' Effect 'with' HandlerBlock

HandlerBlock ::= '{' HandlerCase* '}'

HandlerCase ::= Pattern '=>' Expression
```

**执行说明：**
- 可以拦截和处理效果
- 类似于异常处理，但更通用
- 可以继续计算或返回替代结果

### 效果处理规则
```
handler: E → T
Γ, handle E with handler ⊢ b: R
────────────────────────────────
Γ ⊢ handle E with handler in b: R
```

---

**本章规范采用数学语言定义效果系统语法，自然语言描述执行语义。**
