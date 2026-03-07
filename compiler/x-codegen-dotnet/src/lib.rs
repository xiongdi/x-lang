// .NET CIL生成器 - .NET后端
// 这个 crate 实现了 x-codegen 的 CodeGenerator trait

use std::path::PathBuf;
use x_parser::ast::Program;
use x_codegen::{CodeGenerator, CodegenOutput, CodeGenResult, Target};

/// .NET代码生成器配置
#[derive(Debug, Clone)]
pub struct DotNetConfig {
    pub output_dir: Option<PathBuf>,
    pub optimize: bool,
    pub debug_info: bool,
}

impl Default for DotNetConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            optimize: false,
            debug_info: true,
        }
    }
}

/// .NET代码生成器
pub struct DotNetCodeGenerator {
    config: DotNetConfig,
}

impl CodeGenerator for DotNetCodeGenerator {
    type Config = DotNetConfig;
    type Error = DotNetCodeGenError;

    fn new(config: Self::Config) -> Self {
        Self { config }
    }

    fn generate_from_ast(&mut self, _program: &Program) -> Result<CodegenOutput, Self::Error> {
        Err(DotNetCodeGenError::Unimplemented(".NET backend not yet implemented".to_string()))
    }

    fn generate_from_hir(&mut self, _hir: &()) -> Result<CodegenOutput, Self::Error> {
        Err(DotNetCodeGenError::Unimplemented(".NET backend not yet implemented".to_string()))
    }

    fn generate_from_pir(&mut self, _pir: &()) -> Result<CodegenOutput, Self::Error> {
        Err(DotNetCodeGenError::Unimplemented(".NET backend not yet implemented".to_string()))
    }
}

/// .NET代码生成错误
#[derive(thiserror::Error, Debug)]
pub enum DotNetCodeGenError {
    #[error("代码生成错误: {0}")]
    GenerationError(String),

    #[error("未实现: {0}")]
    Unimplemented(String),

    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<x_codegen::CodeGenError> for DotNetCodeGenError {
    fn from(err: x_codegen::CodeGenError) -> Self {
        match err {
            x_codegen::CodeGenError::GenerationError(msg) => DotNetCodeGenError::GenerationError(msg),
            x_codegen::CodeGenError::IoError(e) => DotNetCodeGenError::IoError(e),
            _ => DotNetCodeGenError::GenerationError(format!("{:?}", err)),
        }
    }
}
