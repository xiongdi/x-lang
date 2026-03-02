# 第2章 类型系统

## 2.1 类型定义

### 类型集合
```
Type = PrimitiveType
     | CompositeType
     | FunctionType
     | EffectType
     | TypeVariable
     | TypeConstructor
```

### 类型环境
```
TypeEnv = Identifier → Type
Δ ∈ TypeEnv
```

### 子类型关系
```
⊢ T <: U  // T是U的子类型
```

## 2.2 基本类型

### 定义
```
PrimitiveType = IntType
              | FloatType
              | BoolType
              | StringType
              | CharType
              | UnitType
              | NeverType
```

### 详细定义

#### 整数类型
```
IntType: ℤ
Value: { ..., -2, -1, 0, 1, 2, ... }
```

#### 浮点数类型
```
FloatType: ℝ
Value: IEEE 754双精度浮点数
```

#### 布尔类型
```
BoolType: 𝔹
Value: { true, false }
```

#### 字符串类型
```
StringType: Σ*
Value: { c₁c₂...cₙ | n ≥ 0, cᵢ ∈ Unicode }
```

#### 字符类型
```
CharType: Σ
Value: Unicode码点
```

#### 单位类型
```
UnitType: ()
Value: { () }
```

#### 永无类型
```
NeverType: ⊥
Value: ∅
```

## 2.3 复合类型

### 定义
```
CompositeType = ArrayType(Type)
              | DictionaryType(Type, Type)
              | TupleType(Type*)
              | RecordType(Identifier × Type*)
              | UnionType(Type*)
              | OptionType(Type)
              | ResultType(Type, Type)
```

### 详细定义

#### 数组类型
```
ArrayType(T): T*
Value: [v₁, v₂, ..., vₙ | n ≥ 0, vᵢ ∈ T]
```

#### 字典类型
```
DictionaryType(K, V): K → V
Value: { k₁: v₁, k₂: v₂, ..., kₙ: vₙ | kᵢ ∈ K, vᵢ ∈ V }
```

#### 元组类型
```
TupleType(T₁, T₂, ..., Tₙ): T₁ × T₂ × ... × Tₙ
Value: (v₁, v₂, ..., vₙ)  where vᵢ ∈ Tᵢ
```

#### 记录类型
```
RecordType(l₁: T₁, l₂: T₂, ..., lₙ: Tₙ)
Value: { l₁: v₁, l₂: v₂, ..., lₙ: vₙ }
       where vᵢ ∈ Tᵢ, lᵢ are distinct labels
```

#### 联合类型
```
UnionType(T₁, T₂, ..., Tₙ): T₁ ∪ T₂ ∪ ... ∪ Tₙ
Value: v where v ∈ Tᵢ for some i
```

#### 可选类型
```
OptionType(T): T ∪ { None }
Value: Some(v) where v ∈ T
     | None
```

#### 结果类型
```
ResultType(T, E): T ∪ E
Value: Ok(v) where v ∈ T
     | Err(e) where e ∈ E
```

## 2.4 函数类型

### 定义
```
FunctionType = (T₁, T₂, ..., Tₙ) → R ⟨Effects⟩
```

### 效果类型
```
Effects = ∅ | {e₁, e₂, ..., eₙ}
```

### 函数类型规则
```
Γ, x₁: T₁, ..., xₙ: Tₙ ⊢ e: R, Δ
────────────────────────────────────
Γ ⊢ (x₁: T₁, ..., xₙ: Tₙ) → e: (T₁, ..., Tₙ) → R ⟨Δ⟩
```

## 2.5 类型操作

### 类型等价
```
T ≡ U  // T和U等价
```

### 类型组合
```
T ∧ U  // 类型交
T ∨ U  // 类型并
¬T     // 类型补
```

## 2.6 类型推理

### 推理规则
```
Γ ⊢ e: T
```

### 变量查找
```
x: T ∈ Γ
────────
Γ ⊢ x: T
```

### 函数应用
```
Γ ⊢ f: (T₁, ..., Tₙ) → R ⟨Δ⟩
Γ ⊢ e₁: T₁  ...  Γ ⊢ eₙ: Tₙ
─────────────────────────────────
Γ ⊢ f(e₁, ..., eₙ): R, Δ
```

### 条件表达式
```
Γ ⊢ e₁: Bool
Γ ⊢ e₂: T
Γ ⊢ e₃: T
──────────────
Γ ⊢ if e₁ then e₂ else e₃: T
```

## 2.7 类型约束

### 变量约束
```
∀x. TypeConstraint(x)
```

### 子类型约束
```
⊢ T <: U
```

---

**本章规范采用数学语言定义类型系统，简洁清晰明了。**
