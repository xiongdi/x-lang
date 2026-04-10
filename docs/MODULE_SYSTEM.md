# 多文件编译和模块系统

X 语言支持多文件编译和模块系统，允许将代码组织到多个文件中并通过 import/export 语句进行模块间的依赖管理。

## 模块声明

使用 `module` 关键字声明模块：

```x
module myapp.utils;
```

## 导出符号

使用 `export` 关键字导出符号：

```x
export add;
export multiply;

function add(a: integer, b: integer) -> integer {
    a + b
}

function multiply(a: integer, b: integer) -> integer {
    a * b
}
```

或者使用 `export` 修饰符：

```x
export function helper() -> integer {
    42
}
```

## 导入模块

### 导入整个模块

```x
import utils;
```

### 导入特定符号

```x
import utils.{add, multiply};
```

### 导入所有符号

```x
import utils.*;
```

### 使用别名

```x
import utils.add as plus;
```

## 模块解析规则

模块解析按以下顺序查找：

1. **标准库模块**（以 `std` 或 `std::` 开头）
   - 在 `library/stdlib/` 目录中查找
   
2. **相对路径模块**
   - 相对于当前文件查找
   - `import helpers.math` 会查找 `./helpers/math.x`
   
3. **搜索路径模块**
   - 在编译器搜索路径中查找

## 示例项目结构

```
examples/modules/
├── main.x              # 入口文件
├── utils.x             # 工具函数
├── types.x             # 类型定义
└── helpers/
    ├── math.x          # 数学辅助函数
    └── string.x        # 字符串辅助函数
```

### main.x
```x
import utils;
import types;
import helpers.math;
import helpers.string;

function main() {
    let sum = add(10, 20);
    println(sum);
    
    let sq = square(7);
    println(sq);
}
```

### utils.x
```x
module utils;

export add;
export multiply;

function add(a: integer, b: integer) -> integer {
    a + b
}

function multiply(a: integer, b: integer) -> integer {
    a * b
}
```

### helpers/math.x
```x
module helpers.math;

export square;
export factorial;

function square(x: integer) -> integer {
    x * x
}

function factorial(n: integer) -> integer {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
```

## 循环依赖检测

编译器会检测循环依赖并报告错误：

```x
// a.x
import b;  // 错误：检测到循环依赖

// b.x
import a;  // 错误：检测到循环依赖
```

## 编译顺序

编译器使用拓扑排序确定模块的编译顺序，确保依赖模块先于依赖它的模块编译。

例如，如果：
- `main` 依赖 `utils` 和 `types`
- `types` 依赖 `utils`

编译顺序为：`utils` → `types` → `main`

## API 参考

### ModuleResolver

解析模块路径和加载模块源代码。

```rust
let mut resolver = ModuleResolver::new()
    .with_stdlib(PathBuf::from("library/stdlib"));

let (path, source) = resolver.resolve_module("utils", Some(Path::new("main.x")))?;
```

### ModuleGraph

表示模块依赖图，支持拓扑排序和循环依赖检测。

```rust
let mut graph = ModuleGraph::new();
graph.add_module(module_info);

let order = graph.topological_sort()?;  // 返回编译顺序
```

### ModuleLoader

加载模块并构建模块图。

```rust
let loader = ModuleLoader::new();
let graph = loader.load_all_modules(Path::new("main.x"))?;
```

### MultiFileCompilationContext

多文件编译上下文，管理模块加载、解析和链接。

```rust
let ctx = MultiFileCompilationContext::new()?;
let program = ctx.compile_file(Path::new("examples/modules/main.x"))?;
```

## 测试

模块系统测试位于 `tests/integration/modules/` 目录：

- `simple_module/` - 简单的两文件项目
- `nested_imports/` - 嵌套模块导入
- `cyclic_deps/` - 循环依赖检测

运行测试：

```bash
cd compiler && cargo test -p x-parser module_resolver
```
