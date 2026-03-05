
# 运行 examples/hello.x

## 当前问题

在尝试运行 `examples/hello.x` 时，我们遇到了几个问题：

### 1. 链接器冲突
- Git for Windows 附带了一个名为 `link.exe` 的工具，位于 `C:\Program Files\Git\usr\bin\link.exe`
- 这个工具是用于创建文件链接的 Unix 工具，而不是编译器链接器
- 它与 MSVC (Microsoft Visual C++) 的 `link.exe` 冲突

### 2. 缺少 Visual C++ Build Tools
- 系统上没有安装 Visual C++ Build Tools 或 Visual Studio 2022（带有 "使用 C++ 的桌面开发" 工作负载）
- Rust 需要 MSVC 链接器来构建 Windows 二进制文件

## 解决方案

### 步骤 1：安装 Visual C++ Build Tools

1. 从 https://visualstudio.microsoft.com/downloads/ 下载 "Build Tools for Visual Studio 2022"
2. 运行安装程序
3. 选择 "使用 C++ 的桌面开发" (Desktop development with C++) 工作负载
4. 点击 "安装" 并等待完成

### 步骤 2：从 Developer Command Prompt 运行

安装完成后：

1. 打开 "Developer Command Prompt for VS 2022"
2. 导航到项目目录：
   ```cmd
   cd C:\Users\Administrator\Documents\x-lang
   ```
3. 运行 hello.x：
   ```cmd
   cargo run -- run examples\hello.x
   ```

### 或者：使用 PowerShell 与 vcvarsall.bat

如果你更喜欢使用 PowerShell：

```powershell
# 找到 vcvarsall.bat 的路径（根据你的安装调整）
$vcvars = "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat"

# 创建一个临时批处理文件
$batch = @"
@echo off
call "$vcvars" x64
cd /d "C:\Users\Administrator\Documents\x-lang"
cargo run -- run examples\hello.x
"@

$batch | Out-File -FilePath "run_hello.bat" -Encoding ASCII
cmd /c "run_hello.bat"
Remove-Item "run_hello.bat"
```

## hello.x 的预期输出

当成功运行时，`examples/hello.x` 应该输出：

```
Hello, World!
```

## 程序说明

`examples/hello.x` 是一个简单的 X 语言程序：

```x
// Hello World - 最简单的 X 语言程序
// 用于测试 C23 后端流水线

function main() {
  print("Hello, World!")
}
```

它定义了一个 `main` 函数，该函数调用 `print` 函数来输出 "Hello, World!" 到标准输出。

## 替代方案：使用 Bazel

如果你想使用 Bazel 而不是 Cargo（不推荐，因为项目主要支持 Cargo），你需要：

1. 首先按照上述步骤安装 Visual C++ Build Tools
2. 确保你使用的 Bazel 版本与 rules_rust 0.63.0 兼容（Bazel 7.x 或 8.x 推荐，而不是 9.x+）
3. 从 Developer Command Prompt 运行：
   ```cmd
   bazel run //:x -- run examples/hello.x
   ```

## 验证安装

安装 Visual C++ Build Tools 后，你可以通过以下方式验证：

```cmd
link.exe /?
```

这应该显示 MSVC 链接器的帮助信息，而不是 Git 的 link 工具的帮助信息。

如果你仍然看到 Git 的 link 工具，你需要调整你的 PATH 环境变量，将 Git 的 `usr/bin` 目录移到后面，或者在构建时暂时重命名 Git 的 `link.exe`。
