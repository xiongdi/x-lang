// LLVM代码生成器 - Native后端
// 这个 crate 实现了 x-codegen 的 CodeGenerator trait

mod lower;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::targets::{FileType, RelocMode, CodeModel, Target as LlvmTarget, TargetMachine};
use std::path::PathBuf;
use x_parser::ast::Program;
use x_codegen::{CodeGenerator, CodegenOutput, CodegenResult, Target, FileType as XFileType};

pub use lower::generate_code;

/// LLVM代码生成器配置
#[derive(Debug, Clone)]
pub struct LlvmConfig {
    pub target: LlvmTargetKind,
    pub output_dir: Option<PathBuf>,
    pub optimize: bool,
    pub debug_info: bool,
}

impl Default for LlvmConfig {
    fn default() -> Self {
        Self {
            target: LlvmTargetKind::Native,
            output_dir: None,
            optimize: false,
            debug_info: true,
        }
    }
}

/// LLVM支持的目标类型
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LlvmTargetKind {
    /// 本地机器码
    Native,
    /// WebAssembly
    Wasm,
    /// LLVM IR文本
    LlvmIr,
}

/// LLVM代码生成器
pub struct LlvmCodeGenerator {
    config: LlvmConfig,
    context: Option<Context>,
}

impl CodeGenerator for LlvmCodeGenerator {
    type Config = LlvmConfig;
    type Error = LlvmCodeGenError;

    fn new(config: Self::Config) -> Self {
        Self {
            config,
            context: None,
        }
    }

    fn generate_from_ast(&mut self, program: &Program) -> Result<CodegenOutput, Self::Error> {
        let codegen_config = x_codegen::CodeGenConfig {
            target: match self.config.target {
                LlvmTargetKind::Native => Target::Native,
                LlvmTargetKind::Wasm => Target::Wasm,
                LlvmTargetKind::LlvmIr => Target::LlvmIr,
            },
            output_dir: self.config.output_dir.clone(),
            optimize: self.config.optimize,
            debug_info: self.config.debug_info,
        };

        let bytes = generate_code(program, &codegen_config)?;

        let output_file = CodegenOutput {
            files: vec![],
            dependencies: vec![],
        };

        Ok(output_file)
    }

    fn generate_from_hir(&mut self, _hir: &()) -> Result<CodegenOutput, Self::Error> {
        Err(LlvmCodeGenError::Unsupported("HIR generation not implemented yet".to_string()))
    }

    fn generate_from_pir(&mut self, _pir: &()) -> Result<CodegenOutput, Self::Error> {
        Err(LlvmCodeGenError::Unsupported("PerceusIR generation not implemented yet".to_string()))
    }
}

/// LLVM代码生成错误
#[derive(thiserror::Error, Debug)]
pub enum LlvmCodeGenError {
    #[error("代码生成错误: {0}")]
    GenerationError(String),

    #[error("不支持的特性: {0}")]
    Unsupported(String),

    #[error("LLVM初始化错误: {0}")]
    LlvmInitError(String),

    #[error("目标机器创建错误: {0}")]
    TargetMachineError(String),

    #[error("IO错误: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<x_codegen::CodeGenError> for LlvmCodeGenError {
    fn from(err: x_codegen::CodeGenError) -> Self {
        match err {
            x_codegen::CodeGenError::GenerationError(msg) => LlvmCodeGenError::GenerationError(msg),
            x_codegen::CodeGenError::UnsupportedFeature(msg) => LlvmCodeGenError::Unsupported(msg),
            x_codegen::CodeGenError::IoError(e) => LlvmCodeGenError::IoError(e),
            _ => LlvmCodeGenError::GenerationError(format!("{:?}", err)),
        }
    }
}
