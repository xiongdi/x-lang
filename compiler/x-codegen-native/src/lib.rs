//! Native 后端 - LIR 直译机器码
//!
//! 无需外部编译器，直接生成可执行机器码
//! 支持 x86_64、AArch64、RISC-V、Wasm32 等架构

use std::path::PathBuf;
use x_codegen::{CodeGenerator, CodegenOutput, OutputFile};
use x_lir::Program as LirProgram;
use x_parser::ast::Program as AstProgram;

/// Native 后端配置
#[derive(Debug, Clone)]
pub struct NativeBackendConfig {
    pub output_dir: Option<PathBuf>,
    pub optimize: bool,
    pub debug_info: bool,
    pub arch: TargetArch,
    pub format: OutputFormat,
}

/// 目标架构
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TargetArch {
    #[default]
    X86_64,
    AArch64,
    RiscV64,
    Wasm32,
}

/// 输出格式
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Executable,
    ObjectFile,
    Assembly,
}

impl Default for NativeBackendConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            optimize: false,
            debug_info: true,
            arch: TargetArch::X86_64,
            format: OutputFormat::Executable,
        }
    }
}

/// Native 后端
pub struct NativeBackend {
    config: NativeBackendConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum NativeError {
    #[error("机器码生成错误: {0}")]
    CodegenError(String),
    #[error("不支持的架构: {0}")]
    UnsupportedArch(String),
    #[error("未实现: {0}")]
    Unimplemented(String),
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}

impl NativeBackend {
    pub fn new(config: NativeBackendConfig) -> Self {
        Self { config }
    }
}

impl CodeGenerator for NativeBackend {
    type Config = NativeBackendConfig;
    type Error = NativeError;

    fn new(config: Self::Config) -> Self {
        Self { config }
    }

    fn generate_from_ast(&mut self, _program: &AstProgram) -> Result<CodegenOutput, Self::Error> {
        Err(NativeError::Unimplemented("Native 后端尚未实现，请从 LIR 生成".to_string()))
    }

    fn generate_from_hir(&mut self, _hir: &x_hir::Hir) -> Result<CodegenOutput, Self::Error> {
        Err(NativeError::Unimplemented("Native 后端尚未实现，请从 LIR 生成".to_string()))
    }

    fn generate_from_lir(&mut self, _lir: &LirProgram) -> Result<CodegenOutput, Self::Error> {
        match self.config.arch {
            TargetArch::X86_64 => {
                // TODO: 实现 x86_64 机器码生成
                Err(NativeError::Unimplemented("x86_64 机器码生成尚未实现".to_string()))
            }
            TargetArch::AArch64 => {
                Err(NativeError::Unimplemented("AArch64 机器码生成尚未实现".to_string()))
            }
            TargetArch::RiscV64 => {
                Err(NativeError::Unimplemented("RISC-V 机器码生成尚未实现".to_string()))
            }
            TargetArch::Wasm32 => {
                Err(NativeError::Unimplemented("Wasm32 机器码生成尚未实现".to_string()))
            }
        }
    }
}
