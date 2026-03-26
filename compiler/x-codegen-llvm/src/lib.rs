//! LLVM 后端 - 直接生成 LLVM IR
//!
//! 通过 LLVM 进行深度优化，生成高质量的原生代码

use std::path::PathBuf;
use x_codegen::{CodeGenerator, CodegenOutput, OutputFile};
use x_lir::Program as LirProgram;
use x_parser::ast::Program as AstProgram;

/// LLVM 后端配置
#[derive(Debug, Clone)]
pub struct LlvmBackendConfig {
    pub output_dir: Option<PathBuf>,
    pub optimize: bool,
    pub debug_info: bool,
    pub target_triple: Option<String>,
}

impl Default for LlvmBackendConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            optimize: false,
            debug_info: true,
            target_triple: None,
        }
    }
}

/// LLVM 后端
pub struct LlvmBackend {
    config: LlvmBackendConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum LlvmError {
    #[error("LLVM 错误: {0}")]
    LlvmError(String),
    #[error("未实现: {0}")]
    Unimplemented(String),
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}

impl LlvmBackend {
    pub fn new(config: LlvmBackendConfig) -> Self {
        Self { config }
    }
}

impl CodeGenerator for LlvmBackend {
    type Config = LlvmBackendConfig;
    type Error = LlvmError;

    fn new(config: Self::Config) -> Self {
        Self { config }
    }

    fn generate_from_ast(&mut self, _program: &AstProgram) -> Result<CodegenOutput, Self::Error> {
        Err(LlvmError::Unimplemented("LLVM 后端尚未实现".to_string()))
    }

    fn generate_from_hir(&mut self, _hir: &x_hir::Hir) -> Result<CodegenOutput, Self::Error> {
        Err(LlvmError::Unimplemented("LLVM 后端尚未实现".to_string()))
    }

    fn generate_from_lir(&mut self, _lir: &LirProgram) -> Result<CodegenOutput, Self::Error> {
        Err(LlvmError::Unimplemented("LLVM 后端尚未实现".to_string()))
    }
}
