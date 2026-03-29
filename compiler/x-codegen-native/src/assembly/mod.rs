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

use crate::{NativeError, NativeResult};
use crate::{TargetArch, TargetOS};

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
