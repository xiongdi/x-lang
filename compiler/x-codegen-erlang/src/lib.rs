//! Erlang 后端 - 生成 Erlang 源代码
//!
//! 面向并发、分布式系统、高可用场景

use std::path::PathBuf;
use x_codegen::{CodeGenerator, CodegenOutput, OutputFile};
use x_lir::Program as LirProgram;
use x_parser::ast::Program as AstProgram;

/// Erlang 后端配置
#[derive(Debug, Clone)]
pub struct ErlangBackendConfig {
    pub output_dir: Option<PathBuf>,
    pub optimize: bool,
    pub debug_info: bool,
    pub module_name: Option<String>,
}

impl Default for ErlangBackendConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            optimize: false,
            debug_info: true,
            module_name: None,
        }
    }
}

/// Erlang 后端
pub struct ErlangBackend {
    config: ErlangBackendConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum ErlangError {
    #[error("Erlang 代码生成错误: {0}")]
    GenerationError(String),
    #[error("未实现: {0}")]
    Unimplemented(String),
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}

impl ErlangBackend {
    pub fn new(config: ErlangBackendConfig) -> Self {
        Self { config }
    }
}

impl CodeGenerator for ErlangBackend {
    type Config = ErlangBackendConfig;
    type Error = ErlangError;

    fn new(config: Self::Config) -> Self {
        Self { config }
    }

    fn generate_from_ast(&mut self, _program: &AstProgram) -> Result<CodegenOutput, Self::Error> {
        Err(ErlangError::Unimplemented("Erlang 后端尚未实现".to_string()))
    }

    fn generate_from_hir(&mut self, _hir: &x_hir::Hir) -> Result<CodegenOutput, Self::Error> {
        Err(ErlangError::Unimplemented("Erlang 后端尚未实现".to_string()))
    }

    fn generate_from_lir(&mut self, _lir: &LirProgram) -> Result<CodegenOutput, Self::Error> {
        Err(ErlangError::Unimplemented("Erlang 后端尚未实现".to_string()))
    }
}
