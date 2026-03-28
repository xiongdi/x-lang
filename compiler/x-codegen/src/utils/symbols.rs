//! 符号表工具
//!
//! 提供代码生成过程中符号管理的通用工具。符号表用于跟踪变量、函数、标签等符号的信息，
//! 支持作用域管理和标签生成。

use std::collections::HashMap;

/// 符号类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolType {
    /// 函数
    Function,
    /// 变量
    Variable,
    /// 常量
    Constant,
    /// 标签
    Label,
    /// 外部符号
    External,
}

/// 符号作用域
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolScope {
    /// 局部符号
    Local,
    /// 全局符号
    Global,
    /// 外部符号
    External,
}

/// 符号信息
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    /// 符号名称
    pub name: String,
    /// 符号类型
    pub symbol_type: SymbolType,
    /// 作用域
    pub scope: SymbolScope,
    /// 偏移量（用于栈变量）
    pub offset: Option<i64>,
    /// 大小（字节）
    pub size: Option<usize>,
    /// 是否已定义
    pub defined: bool,
}

impl SymbolInfo {
    /// 创建新的符号信息
    pub fn new(name: &str, symbol_type: SymbolType, scope: SymbolScope) -> Self {
        Self {
            name: name.to_string(),
            symbol_type,
            scope,
            offset: None,
            size: None,
            defined: false,
        }
    }

    /// 创建函数符号
    pub fn function(name: &str, scope: SymbolScope) -> Self {
        Self::new(name, SymbolType::Function, scope)
    }

    /// 创建变量符号
    pub fn variable(name: &str, scope: SymbolScope) -> Self {
        Self::new(name, SymbolType::Variable, scope)
    }

    /// 创建标签符号
    pub fn label(name: &str) -> Self {
        Self::new(name, SymbolType::Label, SymbolScope::Local)
    }

    /// 创建外部符号
    pub fn external(name: &str) -> Self {
        Self::new(name, SymbolType::External, SymbolScope::External)
    }

    /// 设置偏移量
    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// 设置大小
    pub fn with_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    /// 标记为已定义
    pub fn mark_defined(mut self) -> Self {
        self.defined = true;
        self
    }
}

/// 符号表
///
/// 用于在代码生成过程中跟踪符号信息。
/// 支持符号查找、标签生成和作用域管理。
pub struct SymbolTable {
    /// 符号映射
    symbols: HashMap<String, SymbolInfo>,
    /// 标签计数器
    label_counter: usize,
    /// 当前栈偏移
    stack_offset: i64,
}

impl SymbolTable {
    /// 创建新的符号表
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
            label_counter: 0,
            stack_offset: 0,
        }
    }

    /// 添加符号
    pub fn add(&mut self, info: SymbolInfo) {
        self.symbols.insert(info.name.clone(), info);
    }

    /// 获取符号
    pub fn get(&self, name: &str) -> Option<&SymbolInfo> {
        self.symbols.get(name)
    }

    /// 检查符号是否存在
    pub fn contains(&self, name: &str) -> bool {
        self.symbols.contains_key(name)
    }

    /// 生成唯一标签名
    pub fn generate_label(&mut self, prefix: &str) -> String {
        let label = format!("L_{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    /// 生成唯一临时变量名
    pub fn generate_temp(&mut self) -> String {
        let temp = format!("T{}", self.label_counter);
        self.label_counter += 1;
        temp
    }

    /// 分配栈空间并返回偏移量
    pub fn allocate_stack(&mut self, size: i64) -> i64 {
        self.stack_offset -= size;
        self.stack_offset
    }

    /// 获取当前栈偏移
    pub fn stack_offset(&self) -> i64 {
        self.stack_offset
    }

    /// 重置栈偏移
    pub fn reset_stack(&mut self) {
        self.stack_offset = 0;
    }

    /// 重置标签计数器
    pub fn reset_labels(&mut self) {
        self.label_counter = 0;
    }

    /// 清空符号表
    pub fn clear(&mut self) {
        self.symbols.clear();
        self.label_counter = 0;
        self.stack_offset = 0;
    }

    /// 获取所有符号
    pub fn symbols(&self) -> impl Iterator<Item = &SymbolInfo> {
        self.symbols.values()
    }

    /// 获取所有全局符号
    pub fn global_symbols(&self) -> impl Iterator<Item = &SymbolInfo> {
        self.symbols.values().filter(|s| s.scope == SymbolScope::Global)
    }

    /// 获取所有外部符号
    pub fn external_symbols(&self) -> impl Iterator<Item = &SymbolInfo> {
        self.symbols.values().filter(|s| s.scope == SymbolScope::External)
    }

    /// 获取符号数量
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_info_creation() {
        let info = SymbolInfo::function("main", SymbolScope::Global);
        assert_eq!(info.name, "main");
        assert_eq!(info.symbol_type, SymbolType::Function);
        assert_eq!(info.scope, SymbolScope::Global);
        assert!(!info.defined);
    }

    #[test]
    fn test_symbol_info_with_offset() {
        let info = SymbolInfo::variable("x", SymbolScope::Local)
            .with_offset(-8)
            .with_size(8);
        assert_eq!(info.offset, Some(-8));
        assert_eq!(info.size, Some(8));
    }

    #[test]
    fn test_symbol_table_add_get() {
        let mut table = SymbolTable::new();
        table.add(SymbolInfo::function("main", SymbolScope::Global));

        assert!(table.contains("main"));
        assert!(table.get("main").is_some());
        assert!(!table.contains("unknown"));
    }

    #[test]
    fn test_symbol_table_generate_label() {
        let mut table = SymbolTable::new();

        let label1 = table.generate_label("if");
        let label2 = table.generate_label("if");
        let label3 = table.generate_label("while");

        assert_eq!(label1, "L_if_0");
        assert_eq!(label2, "L_if_1");
        assert_eq!(label3, "L_while_2");
    }

    #[test]
    fn test_symbol_table_stack_allocation() {
        let mut table = SymbolTable::new();

        let offset1 = table.allocate_stack(8);
        let offset2 = table.allocate_stack(8);

        assert_eq!(offset1, -8);
        assert_eq!(offset2, -16);
        assert_eq!(table.stack_offset(), -16);
    }

    #[test]
    fn test_symbol_table_reset() {
        let mut table = SymbolTable::new();
        table.add(SymbolInfo::function("main", SymbolScope::Global));
        table.allocate_stack(16);
        table.generate_label("test");

        table.clear();

        assert!(table.is_empty());
        assert_eq!(table.stack_offset(), 0);
    }

    #[test]
    fn test_symbol_table_iterators() {
        let mut table = SymbolTable::new();
        table.add(SymbolInfo::function("main", SymbolScope::Global));
        table.add(SymbolInfo::variable("x", SymbolScope::Local));
        table.add(SymbolInfo::external("printf"));

        let globals: Vec<_> = table.global_symbols().collect();
        let externals: Vec<_> = table.external_symbols().collect();

        assert_eq!(globals.len(), 1);
        assert_eq!(externals.len(), 1);
    }
}
