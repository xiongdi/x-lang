//! Java 后端 - 生成 Java 源代码
//!
//! 面向 JVM 平台，生成 Java 源代码

use std::path::PathBuf;
use x_codegen::{CodeGenerator, CodegenOutput, OutputFile, FileType};
use x_lir::Program as LirProgram;
use x_parser::ast::Program as AstProgram;

/// Java 后端配置
#[derive(Debug, Clone)]
pub struct JavaConfig {
    pub output_dir: Option<PathBuf>,
    pub optimize: bool,
    pub debug_info: bool,
}

impl Default for JavaConfig {
    fn default() -> Self {
        Self {
            output_dir: None,
            optimize: false,
            debug_info: true,
        }
    }
}

/// Java 后端
pub struct JavaBackend {
    config: JavaConfig,
}

#[derive(Debug, thiserror::Error)]
pub enum JavaError {
    #[error("Java 代码生成错误: {0}")]
    GenerationError(String),
    #[error("未实现: {0}")]
    Unimplemented(String),
    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),
}

impl JavaBackend {
    pub fn new(config: JavaConfig) -> Self {
        Self { config }
    }
}

impl CodeGenerator for JavaBackend {
    type Config = JavaConfig;
    type Error = JavaError;

    fn new(config: Self::Config) -> Self {
        Self { config }
    }

    fn generate_from_ast(&mut self, _program: &AstProgram) -> Result<CodegenOutput, Self::Error> {
        // TODO: 实现 AST -> Java 源码生成
        Err(JavaError::Unimplemented("Java 后端尚未实现".to_string()))
    }

    fn generate_from_hir(&mut self, _hir: &x_hir::Hir) -> Result<CodegenOutput, Self::Error> {
        Err(JavaError::Unimplemented("Java 后端尚未实现".to_string()))
    }

    fn generate_from_lir(&mut self, _lir: &LirProgram) -> Result<CodegenOutput, Self::Error> {
        // TODO: 实现 LIR -> Java 源码生成
        Err(JavaError::Unimplemented("Java 后端尚未实现".to_string()))
    }
}

// 保持向后兼容的别名
pub type JavaCodeGenerator = JavaBackend;
pub type JavaCodeGenError = JavaError;
pub type JavaResult<T> = Result<T, JavaError>;
