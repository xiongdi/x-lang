//! Swift 后端 - 生成 Swift 源代码
//!
//! 面向 Apple 生态（iOS、macOS、watchOS、tvOS）

use std::path::PathBuf;
use x_codegen::{CodeGenerator, CodegenOutput, OutputFile};
use x_lir::Program as LirProgram;
use x_parser::ast::Program as AstProgram;

/// Swift 后端配置
#[derive(Debug, Clone)]
pub struct SwiftBackendConfig {
    pub output_dir: Option<PathBuf>,
    pub optimize: bool,
    pub debug_info: bool,
    pub target: SwiftTarget,
}

/// Swift 编译目标
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SwiftTarget {
    #[default]
    MacOS,
    IOS,
    WatchOS,
    TvOS,
    Linux,
}

impl Default for SwiftBackendConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            optimize: false,
            debug_info: true,
            target: SwiftTarget::MacOS,
        }
    }
}

/// Swift 后端
pub struct SwiftBackend {
    config: SwiftBackendConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum SwiftError {
    #[error("Swift 代码生成错误: {0}")]
    GenerationError(String),
    #[error("未实现: {0}")]
    Unimplemented(String),
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}

impl SwiftBackend {
    pub fn new(config: SwiftBackendConfig) -> Self {
        Self { config }
    }
}

impl CodeGenerator for SwiftBackend {
    type Config = SwiftBackendConfig;
    type Error = SwiftError;

    fn new(config: Self::Config) -> Self {
        Self { config }
    }

    fn generate_from_ast(&mut self, _program: &AstProgram) -> Result<CodegenOutput, Self::Error> {
        Err(SwiftError::Unimplemented("Swift 后端尚未实现".to_string()))
    }

    fn generate_from_hir(&mut self, _hir: &x_hir::Hir) -> Result<CodegenOutput, Self::Error> {
        Err(SwiftError::Unimplemented("Swift 后端尚未实现".to_string()))
    }

    fn generate_from_lir(&mut self, _lir: &LirProgram) -> Result<CodegenOutput, Self::Error> {
        Err(SwiftError::Unimplemented("Swift 后端尚未实现".to_string()))
    }
}
