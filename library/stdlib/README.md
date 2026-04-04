# X 语言标准库（源码）

本目录包含 X 语言的**标准库源代码**（`.x` 文件），由编译器在解析 `import` / 预置搜索路径时加载，**不是** Rust `Cargo` crate。

## 模块一览

| 文件 | 说明 |
|------|------|
| `prelude.x` | 预导入符号 |
| `types.x` | 核心类型与内建相关定义 |
| `collections.x` | 集合类型 |
| `io.x` / `fs.x` / `net.x` | I/O、文件系统、网络 |
| `math.x` / `random.x` / `time.x` | 数学、随机、时间 |
| `encoding.x` / `hash.x` | 编码与哈希 |
| `process.x` / `panic.x` / `unsafe.x` | 进程、恐慌、不安全操作 |

更完整的语言与标准库说明见仓库根目录下的 [spec/README.md](../../spec/README.md) 与文档站点中的标准库章节。
