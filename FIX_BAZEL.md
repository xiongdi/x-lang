
# 修复 Bazel 并运行 @examples/hello.x

## 当前状态

我们在尝试使用 Bazel 和 Cargo 时遇到了两个主要问题：

1. **Bazel 问题**：
   - Bazel 服务器锁问题
   - rules_rust 与 Bazel 9.0+ 的兼容性问题
   - Bzlmod 配置问题

2. **Cargo 问题**：
   - 缺少 Visual C++ Build Tools（link.exe 未找到）
   - Git 的 link.exe 与 MSVC 的 link.exe 冲突

## 在您的机器上修复的步骤

### 选项 1：使用 Cargo（推荐，更简单）

1. **安装 Visual Studio Build Tools**：
   - 从 https://visualstudio.microsoft.com/downloads/ 下载
   - 安装时选择"使用 C++ 的桌面开发"（Desktop development with C++）工作负载

2. **从 Developer Command Prompt 运行**：
   - 打开"Developer Command Prompt for VS 2022"（或您安装的任何版本）
   - 导航到项目目录
   - 运行：
     ```cmd
     cargo run -- run examples\hello.x
     ```

### 选项 2：使用 Bazel

1. **首先确保 Cargo 能正常工作**（按照选项 1 的步骤操作）

2. **修复 Bazel 配置**：
   - 确认使用的 Bazel 版本与 rules_rust 兼容
   - 可能需要：
     - 降级 Bazel 到 7.x 或 8.x（与 rules_rust 兼容性更好）
     - 或使用更新的 rules_rust 版本（0.52.0+）与正确的配置
     - 或使用传统的 WORKSPACE 文件代替 Bzlmod

3. **从正确的环境运行 Bazel**：
   - 同样，使用 Developer Command Prompt
   - 运行：
     ```cmd
     bazel run //:x -- run examples/hello.x
     ```

## @examples/hello.x 的预期输出

当成功运行时，程序应输出：
```
Hello, World!
```
