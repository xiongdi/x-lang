# 第8章 模块系统

## 8.1 模块声明

### 模块语法
```
ModuleDeclaration ::= 'module' Identifier ('{' ModuleItem* '}')?
                    | 'module' Identifier ';'

ModuleItem ::= Declaration
             | ImportDeclaration
             | ExportDeclaration
             | SubModuleDeclaration
```

**执行说明：**

1. **模块定义**：
   - 每个源文件隐式是一个模块
   - 可以显式声明模块名称
   - 模块可以嵌套（子模块）

2. **模块内容**：
   - 声明（函数、类型、类等）
   - 导入语句
   - 导出语句
   - 子模块

---

## 8.2 导入与导出

### Import语法
```
ImportDeclaration ::= 'import' ImportPath ('as' Identifier)? ';'

ImportPath ::= Identifier ('.' Identifier)*

ImportSelector ::= '{' ImportItem (',' ImportItem)* '}'

ImportItem ::= Identifier ('as' Identifier)?
             | '*' ('as' Identifier)?
```

**执行说明：**

1. **导入形式**：
   - 单一导入：`import foo.Bar`
   - 选择导入：`import foo.{Bar, Baz}`
   - 通配导入：`import foo.*`
   - 重命名：`import foo.Bar as MyBar`

### Export语法
```
ExportDeclaration ::= 'export' Declaration
                    | 'export' '*' 'from' ImportPath ';'
                    | 'export' '{' ExportItem (',' ExportItem)* '}' ';'

ExportItem ::= Identifier ('as' Identifier)?
```

**执行说明：**

1. **导出形式**：
   - 内联导出：`export fun f() = ...`
   - 批量导出：`export { f, g }`
   - 重新导出：`export * from foo`

### 导入导出规则
```
Module M exports x: T
────────────────────
⊢ import M.x: T

Module M contains N as submodule
────────────────────────
⊢ M.N is a valid module path
```

---

## 8.3 模块路径与解析

### 模块路径
```
ModulePath ::= AbsolutePath | RelativePath

AbsolutePath ::= '::' Identifier ('.' Identifier)*

RelativePath ::= '.' Identifier ('.' Identifier)*
              | '..' ('.' Identifier)*
```

**执行说明：**

1. **绝对路径**：从根模块开始 `::std.collections`
2. **相对路径**：从当前模块开始 `.utils` 或 `..parent`
3. **模块解析**：按文件系统目录结构或包管理配置查找

### 模块解析规则
```
resolve(Package, Path) = Module
  where
    Package is the current package
    Path is the import path
    Module is the resolved module

lookup(Module, Name) = Declaration
  where Declaration is exported by Module
```

---

## 8.4 包管理

### Package定义
```
Package ::= 'package' Identifier Version? ';'

Version ::= StringLiteral

Dependency ::= 'use' PackageName VersionConstraint? ';'

VersionConstraint ::= StringLiteral
```

**执行说明：**

1. **包声明**：
   - `package myapp "1.0.0"`
   - 声明当前包的名称和版本

2. **依赖声明**：
   - `use std "^2.0"`
   - 声明依赖的包和版本约束

3. **版本约束**：
   - `^1.2.3`：兼容版本（1.x.x）
   - `~1.2.3`：补丁版本（1.2.x）
   - `=1.2.3`：精确版本

---

## 8.5 可见性控制

### 可见性修饰符
```
Visibility ::= 'public' | 'private' | 'internal'
```

**执行说明：**

| 修饰符 | 同类 | 同模块 | 同包 | 外部 |
|--------|------|--------|------|------|
| public | ✓ | ✓ | ✓ | ✓ |
| internal | ✓ | ✓ | ✓ | ✗ |
| private | ✓ | ✓ | ✗ | ✗ |

### 可见性规则
```
Declaration D in module M has visibility V
access from location L is allowed iff
  V = public ∨
  (V = internal ∧ same_package(M, L)) ∨
  (V = private ∧ same_module(M, L))
```

---

**本章规范采用数学语言定义模块系统语法，自然语言描述执行语义。**
