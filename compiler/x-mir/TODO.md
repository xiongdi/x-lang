# TODO - x-mir

## 待完成功能

### 高优先级

- [ ] 调试信息生成

### 中优先级

- [ ] 增量 MIR 生成

### 测试

- [ ] 补充增量 MIR 生成测试用例

---

## 完成状态

- [x] MIR 结构定义（CFG、基本块、指令）
- [x] **HIR 到 MIR 转换**
- [x] **Perceus 内存管理分析**
  - [x] dup 分析
  - [x] drop 分析
  - [x] 重用分析
- [x] **优化 passes**
  - [x] 常量传播（const_prop）
  - [x] 死代码消除（dead_code）
- [x] 与类型检查器集成