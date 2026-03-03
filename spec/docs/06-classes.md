# 第6章 面向对象

## 6.1 类声明

### 类语法
```
ClassDeclaration ::= 'class' Identifier TypeParameters?
                     ('extends' Type)?
                     ('implements' Type (',' Type)*)?
                     ClassBody

ClassBody ::= '{' ClassMember* '}'

ClassMember ::= FieldDeclaration
              | MethodDeclaration
              | ConstructorDeclaration
              | StaticDeclaration
```

**执行说明：**

1. **类声明**：
   - 定义一个新的类型，包含字段和方法
   - 可以继承另一个类（单继承）
   - 可以实现多个 trait（接口）

2. **类成员**：
   - 字段：存储对象状态
   - 方法：定义对象行为
   - 构造函数：初始化新对象
   - 静态成员：属于类而不是实例

---

## 6.2 字段声明

### 字段语法
```
FieldDeclaration ::= Visibility? 'let' 'mut'? Identifier (':' Type)? ('=' Expression)? ';'

Visibility ::= 'public' | 'private' | 'protected'
```

**执行说明：**

1. **可见性**：
   - `public`：可以从任何地方访问
   - `private`：只能在类内部访问（默认）
   - `protected`：可以在类和子类中访问

2. **可变性**：
   - `let`：不可变字段，初始化后不能修改
   - `let mut`：可变字段，可以重新赋值

### 字段类型规则
```
C has field f: T with visibility V
────────────────────────────────
obj: C ⊢ obj.f: T
  (if V allows access)
```

---

## 6.3 方法声明

### 方法语法
```
MethodDeclaration ::= Visibility? 'fun' Identifier Parameters (':' Type)? FunctionBody
```

**执行说明：**
- 方法的第一个参数隐式是 `this`，指向当前实例
- 可以访问 `this` 的字段和其他方法
- 可以被子类重写（除非声明为 `final`）

### 方法类型规则
```
Γ, this: C, x₁: T₁, ..., xₙ: Tₙ ⊢ e: R
────────────────────────────────────────
Γ ⊢ C::m(x₁: T₁, ..., xₙ: Tₙ): (T₁, ..., Tₙ) → R

obj: C, C has method m: (T₁, ..., Tₙ) → R
────────────────────────────────────────
Γ ⊢ obj.m(e₁, ..., eₙ): R
```

---

## 6.4 构造函数

### 构造函数语法
```
ConstructorDeclaration ::= 'new' Parameters ConstructorBody

ConstructorBody ::= '=' 'this' '(' FieldInitializer (',' FieldInitializer)* ')' ';'
                  | Block

FieldInitializer ::= Identifier '=' Expression
```

**执行说明：**

1. **构造函数调用**：
   - 使用 `new ClassName(args)` 创建新实例
   - 分配内存并初始化所有字段
   - 调用构造函数体

2. **字段初始化**：
   - 如果字段有初始值表达式，在构造时执行
   - 构造函数参数可以用来初始化字段

### 构造函数求值规则
```
⟦new C(e₁, ..., eₙ)⟧ᵍ = obj
  where
    v₁ = ⟦e₁⟧ᵍ
    ...
    vₙ = ⟦eₙ⟧ᵍ
    obj = allocate(C)
    obj = initialize_fields(obj, v₁, ..., vₙ)
    obj = run_constructor(obj, v₁, ..., vₙ)
```

---

## 6.5 继承

### 继承语法
```
ClassDeclaration ::= 'class' Identifier 'extends' Type ClassBody
```

**执行说明：**

1. **单继承**：
   - 一个类只能继承一个父类
   - 继承父类的所有非私有字段和方法
   - 可以重写父类的方法

2. **重写（Override）**：
   - 子类可以提供父类方法的新实现
   - 签名必须兼容（参数类型 contravariant，返回类型 covariant）
   - 使用 `override` 关键字明确表示重写

### 继承类型规则
```
⊢ C extends D
─────────────
⊢ C <: D

Γ ⊢ obj: C, C <: D
──────────────────
Γ ⊢ obj: D  // 向上转型

m in C overrides m in D
param_types(C::m) <: param_types(D::m)
return_type(D::m) <: return_type(C::m)
──────────────────────────────────────
// 重写合法
```

---

## 6.6 Trait（接口）

### Trait语法
```
TraitDeclaration ::= 'trait' Identifier TypeParameters? TraitBody

TraitBody ::= '{' TraitMember* '}'

TraitMember ::= MethodSignature
              | DefaultMethod
              | TypeRequirement

MethodSignature ::= 'fun' Identifier Parameters (':' Type)? ';'

DefaultMethod ::= 'fun' Identifier Parameters (':' Type)? FunctionBody
```

**执行说明：**

1. **Trait定义**：
   - 定义一组方法签名
   - 可以提供默认实现
   - 一个类可以实现多个 trait

2. **Trait实现**：
   - 类必须提供所有未实现的方法
   - 可以使用默认方法，也可以重写

### Trait类型规则
```
C implements T
C provides all methods required by T
──────────────────────────────────
⊢ C <: T

trait T {
  fun m(x: A): B
  fun n(x: C): D = e
}
class C implements T {
  fun m(x: A): B = e₁  // 必须实现
  // n 可以使用默认或重写
}
```

---

## 6.7 多态与动态分发

### 动态分发
```
Dispatch: Type × MethodName → MethodImplementation

dispatch(obj, m) = most_specific_implementation(type_of(obj), m)
```

**执行说明：**
- 方法调用在运行时根据对象的实际类型选择实现
- 使用单分派（基于 `this` 的类型）
- 遵循最具体实现原则

### 动态分发规则
```
⟦obj.m(e₁, ..., eₙ)⟧ᵍ = v
  where
    C = dynamic_type(obj)
    m_impl = lookup_method(C, m)
    v = apply(m_impl, obj, v₁, ..., vₙ)
    v₁ = ⟦e₁⟧ᵍ
    ...
    vₙ = ⟦eₙ⟧ᵍ
```

---

## 6.8 访问控制

### 访问控制规则
```
Accessibility = Public | Private | Protected | Internal

accessible(from: Location, member: Member): Bool =
  case member.visibility of
    Public → true
    Private → from == member.declaring_class
    Protected → from ∈ member.declaring_class ∪ subclasses
    Internal → from.same_module_as(member)
```

**执行说明：**

| 修饰符 | 同类 | 子类 | 同模块 | 外部 |
|--------|------|------|--------|------|
| public | ✓ | ✓ | ✓ | ✓ |
| protected | ✓ | ✓ | ✓ | ✗ |
| internal | ✓ | ✓ | ✓ | ✗ |
| private | ✓ | ✗ | ✗ | ✗ |

---

**本章规范采用数学语言定义面向对象语法，自然语言描述执行语义。**
