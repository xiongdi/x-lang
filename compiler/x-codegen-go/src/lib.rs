//! Go 后端 - 生成 Go 源代码
//!
//! 面向云原生、微服务、网络编程

use std::path::PathBuf;
use x_codegen::{CodeGenerator, CodegenOutput, OutputFile};
use x_lir::Program as LirProgram;
use x_parser::ast::Program as AstProgram;

/// Go 后端配置
#[derive(Debug, Clone)]
pub struct GoBackendConfig {
    pub output_dir: Option<PathBuf>,
    pub optimize: bool,
    pub debug_info: bool,
    pub module_name: Option<String>,
}

impl Default for GoBackendConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            optimize: false,
            debug_info: true,
            module_name: None,
        }
    }
}

/// Go 后端
pub struct GoBackend {
    config: GoBackendConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum GoError {
    #[error("Go 代码生成错误: {0}")]
    GenerationError(String),
    #[error("未实现: {0}")]
    Unimplemented(String),
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}

impl GoBackend {
    pub fn new(config: GoBackendConfig) -> Self {
        Self { config }
    }
}

impl CodeGenerator for GoBackend {
    type Config = GoBackendConfig;
    type Error = GoError;

    fn new(config: Self::Config) -> Self {
        Self { config }
    }

    fn generate_from_ast(&mut self, _program: &AstProgram) -> Result<CodegenOutput, Self::Error> {
        Err(GoError::Unimplemented("Go 后端尚未实现".to_string()))
    }

    fn generate_from_hir(&mut self, _hir: &x_hir::Hir) -> Result<CodegenOutput, Self::Error> {
        Err(GoError::Unimplemented("Go 后端尚未实现".to_string()))
    }

    fn generate_from_lir(&mut self, _lir: &LirProgram) -> Result<CodegenOutput, Self::Error> {
        Err(GoError::Unimplemented("Go 后端尚未实现".to_string()))
    }
}
