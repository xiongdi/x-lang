
# 运行 examples/hello.x - 最终设置指南

## 我们目前已完成的工作

1. ✅ 已通过 winget 安装 Visual Studio Build Tools 2022
2. ✅ 已找到安装位置：`C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools`
3. 🔄 **正在进行中**：安装 "使用 C++ 的桌面开发" 工作负载（安装程序现在应该正在运行）

## 接下来的步骤

### 步骤 1：等待安装完成
- Visual Studio 安装程序目前应该正在运行
- 等待其完成（可能需要 10-20 分钟，具体取决于你的网络连接）
- 如果安装程序尚未运行，请手动运行：
  ```cmd
  "C:\Program Files (x86)\Microsoft Visual Studio\Installer\setup.exe" modify --installPath "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools" --add Microsoft.VisualStudio.Workload.VCTools;includeRecommended --passive --wait
  ```

### 步骤 2：验证 vcvarsall.bat 是否存在
安装完成后，检查以下文件是否存在：
```
C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvarsall.bat
```

### 步骤 3：运行 hello.x
使用我们创建的批处理文件：
```cmd
C:\Users\Administrator\Documents\x-lang\run_hello.bat
```

或者手动操作：
1. 打开 "Developer Command Prompt for VS 2022"（或者运行 vcvarsall.bat）
2. 导航到项目目录
3. 运行：
   ```cmd
   cargo run -- run examples\hello.x
   ```

## 预期输出

成功运行后，`examples/hello.x` 将输出：
```
Hello, World!
```

## 程序说明

`examples/hello.x` 包含：
```x
// Hello World - 最简单的 X 语言程序
// 用于测试 C23 后端流水线

function main() {
  print("Hello, World!")
}
```

这个简单的程序：
1. 定义了一个 `main` 函数（程序入口点）
2. 调用 `print` 函数，并传入字符串 "Hello, World!"
3. `print` 函数将字符串输出到标准输出（console）

## 如果仍然遇到问题

### 问题 1：Git 的 link.exe 干扰
如果你仍然遇到链接错误，提示 "link: extra operand"，你需要暂时重命名 Git 的 link.exe：
```cmd
rename "C:\Program Files\Git\usr\bin\link.exe" "C:\Program Files\Git\usr\bin\link.exe.backup"
```
构建完成后，可以将其重命名为原名。

### 问题 2：PATH 顺序问题
确保 Git 的 bin 目录在 PATH 中位于 Visual Studio 的目录**之后**。

### 问题 3：验证链接器
运行以下命令验证你使用的是正确的 link.exe：
```cmd
link.exe /?
```
这应该显示 Microsoft (R) Incremental Linker 版权信息，而不是 Git 的 link 工具帮助信息。

## 关于 Bazel 的说明

用户最初要求使用 Bazel，但：
1. 项目主要使用 Cargo（如 CLAUDE.md 中所指定）
2. Bazel 配置存在兼容性问题（rules_rust 与 Bazel 9.0+）
3. Cargo 是构建此项目的推荐且更简单的方式

如果你确实想使用 Bazel，请参阅 FIX_BAZEL.md 获取指导，但建议先使用 Cargo，因为它设置起来更简单。
