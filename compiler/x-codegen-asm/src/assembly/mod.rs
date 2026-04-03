//! 汇编生成模块
//!
//! 提供结构化的汇编生成支持，支持多种目标架构。
//!
//! # 架构支持
//!
//! - **x86_64**: NASM/GAS 语法
//! - **AArch64**: GNU 汇编语法
//! - **RISC-V**: GNU 汇编语法
//! - **Wasm**: WebAssembly 文本格式 (WAT)
//!
//! # 使用示例
//!
//! ```ignore
//! use x_codegen_native::assembly::{AssemblyGenerator, X86_64AssemblyGenerator};
//!
//! let mut generator = X86_64AssemblyGenerator::new(config);
//! let asm = generator.generate(&lir)?;
//! ```

mod x86_64;
mod aarch64;
mod riscv;
mod wasm;

pub use x86_64::X86_64AssemblyGenerator;
pub use aarch64::AArch64AssemblyGenerator;
pub use riscv::RiscVAssemblyGenerator;
pub use wasm::Wasm32AssemblyGenerator;

use std::collections::HashMap;
use crate::{NativeError, NativeResult};
use crate::{TargetArch, TargetOS};

/// 全局变量信息
#[derive(Debug, Clone)]
pub struct GlobalInfo {
    pub size: usize,
    pub initialized: bool,
    pub align: usize,
}

/// 汇编生成器共享上下文
///
/// 提供所有架构汇编生成器共享的状态和方法，减少代码重复。
pub struct AssemblyContext {
    /// 输出缓冲区
    pub output: String,
    /// 当前缩进级别
    pub indent: usize,
    /// 标签计数器
    pub label_counter: usize,
    /// 字符串字面量表
    pub string_literals: HashMap<String, String>,
    /// 全局变量表
    pub globals: HashMap<String, GlobalInfo>,
    /// 局部变量栈偏移
    pub local_offsets: HashMap<String, i32>,
    /// 当前栈帧大小
    pub stack_size: usize,
    /// 当前函数名
    pub current_function: String,
    /// 循环标签栈 - (continue_label, break_label)
    pub loop_labels: Vec<(String, String)>,
    /// 字段偏移表
    pub field_offsets: HashMap<String, usize>,
}

impl AssemblyContext {
    /// 创建新的汇编上下文
    pub fn new() -> Self {
        Self {
            output: String::with_capacity(4096),
            indent: 0,
            label_counter: 0,
            string_literals: HashMap::new(),
            globals: HashMap::new(),
            local_offsets: HashMap::new(),
            stack_size: 0,
            current_function: String::new(),
            loop_labels: Vec::new(),
            field_offsets: HashMap::new(),
        }
    }

    /// 清空上下文状态
    pub fn clear(&mut self) {
        self.output.clear();
        self.indent = 0;
        self.label_counter = 0;
        self.string_literals.clear();
        self.globals.clear();
        self.local_offsets.clear();
        self.stack_size = 0;
        self.current_function.clear();
        self.loop_labels.clear();
        self.field_offsets.clear();
    }

    /// 生成唯一标签名
    pub fn new_label(&mut self, prefix: &str) -> String {
        let label = format!("L_{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    /// 输出一行汇编（带缩进）
    pub fn emit_line(&mut self, line: &str) -> NativeResult<()> {
        use std::fmt::Write;
        writeln!(self.output, "{}{}", "    ".repeat(self.indent), line)
            .map_err(|e| NativeError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))
    }

    /// 输出原始文本（无缩进）
    pub fn emit_raw(&mut self, text: &str) -> NativeResult<()> {
        use std::fmt::Write;
        write!(self.output, "{}", text)
            .map_err(|e| NativeError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))
    }

    /// 增加缩进
    pub fn indent(&mut self) {
        self.indent += 1;
    }

    /// 减少缩进
    pub fn dedent(&mut self) {
        self.indent = self.indent.saturating_sub(1);
    }

    /// 获取当前输出
    pub fn output(&self) -> &str {
        &self.output
    }

    /// 获取输出所有权
    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.output)
    }
}

impl Default for AssemblyContext {
    fn default() -> Self {
        Self::new()
    }
}

/// 汇编生成器 trait
///
/// 所有架构的汇编生成器都要实现这个 trait。
pub trait AssemblyGenerator {
    /// 生成汇编代码
    ///
    /// 从 LIR 程序生成目标架构的汇编代码文本。
    fn generate(&mut self, lir: &x_lir::Program) -> NativeResult<String>;

    /// 获取目标架构
    fn arch(&self) -> TargetArch;

    /// 获取文件扩展名
    fn extension(&self) -> &'static str {
        match self.arch() {
            TargetArch::X86_64 => "asm",
            TargetArch::AArch64 => "s",
            TargetArch::RiscV64 => "s",
            TargetArch::Wasm32 => "wat",
        }
    }
}

/// 创建对应架构的汇编生成器
pub fn create_generator(arch: TargetArch, os: TargetOS) -> Box<dyn AssemblyGenerator> {
    match arch {
        TargetArch::X86_64 => Box::new(X86_64AssemblyGenerator::new(os)),
        TargetArch::AArch64 => Box::new(AArch64AssemblyGenerator::new(os)),
        TargetArch::RiscV64 => Box::new(RiscVAssemblyGenerator::new(os)),
        TargetArch::Wasm32 => Box::new(Wasm32AssemblyGenerator::new(os)),
    }
}
