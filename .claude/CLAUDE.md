# Local Claude Instructions

此文件包含本地/个人的 Claude Code 配置指令，会覆盖项目根目录的 `CLAUDE.md` 中的相应设置。

## 版本控制

默认使用 Jujutsu (jj) 进行版本控制操作：
- 使用 `jj` 命令替代 `git` 命令
- 提交：`jj commit -m "message"`
- 推送：`jj git push`
- 拉取：`jj git fetch`

## 开发偏好

- 包管理器：Cargo（Rust）
- 后端优先级：Zig 后端（最成熟）
- 测试运行：`cd compiler && cargo test`
