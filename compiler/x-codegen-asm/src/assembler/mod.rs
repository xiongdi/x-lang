//! 汇编器集成模块
//!
//! 提供外部汇编器（MASM, NASM, GAS, Clang）集成和直接编码支持。
//!
//! # 支持的汇编器
//!
//! - **MASM (ml64)**: Microsoft Macro Assembler，Windows x64 首选，随 Visual Studio/Windows SDK 提供
//! - **NASM**: Netwide Assembler，跨平台汇编器
//! - **GAS**: GNU Assembler，用于 Linux/macOS
//! - **Clang**: LLVM 集成汇编器
//!
//! # Windows 平台优先级
//!
//! 在 Windows 上优先使用 MASM (ml64) + Microsoft Linker (link.exe)，
//! 这些工具随 Visual Studio/Windows SDK 提供，无需额外安装。
//!
//! # 使用示例
//!
//! ```ignore
//! use x_codegen_native::assembler::{Assembler, ExternalAssembler};
//!
//! // Windows 平台会自动选择 MASM
//! let assembler = ExternalAssembler::Masm;
//! if assembler.is_available() {
//!     assembler.assemble(&asm_code, &output_path)?;
//! }
//! ```

mod direct;
mod external;

pub use direct::DirectEncoder;
pub use external::{AssemblerConfig, ExternalAssembler, LinkerConfig, MicrosoftLinker, OutputObjectFormat};

use std::path::Path;

use crate::{NativeError, NativeResult, TargetArch};

/// 汇编器 trait
///
/// 定义汇编代码到目标文件的转换接口。
pub trait Assembler {
    /// 将汇编代码汇编为目标文件
    ///
    /// # 参数
    ///
    /// - `asm`: 汇编代码文本
    /// - `output`: 输出文件路径
    fn assemble(&self, asm: &str, output: &Path) -> NativeResult<()>;

    /// 获取汇编器名称
    fn name(&self) -> &'static str;

    /// 检查汇编器是否可用
    fn is_available(&self) -> bool;
}

/// 根据架构和操作系统创建合适的汇编器
pub fn create_assembler(arch: TargetArch, os: crate::TargetOS, config: AssemblerConfig) -> Box<dyn Assembler> {
    match arch {
        TargetArch::X86_64 => {
            // 根据操作系统选择优先的汇编器
            match os {
                crate::TargetOS::Windows => {
                    // Windows 优先使用 MASM (ml64)，随 Visual Studio 提供
                    if ExternalAssembler::Masm.is_available() {
                        Box::new(ExternalAssembler::Masm.with_config(config))
                    } else if ExternalAssembler::Nasm.is_available() {
                        Box::new(ExternalAssembler::Nasm.with_config(config))
                    } else {
                        // 使用直接编码作为最后的回退
                        Box::new(DirectEncoder::new(arch))
                    }
                }
                crate::TargetOS::Linux | crate::TargetOS::MacOS => {
                    // Linux/macOS 优先使用 GAS，然后是 NASM
                    if ExternalAssembler::Gas.is_available() {
                        Box::new(ExternalAssembler::Gas.with_config(config))
                    } else if ExternalAssembler::Nasm.is_available() {
                        Box::new(ExternalAssembler::Nasm.with_config(config))
                    } else {
                        Box::new(DirectEncoder::new(arch))
                    }
                }
            }
        }
        TargetArch::AArch64 | TargetArch::RiscV64 => {
            if ExternalAssembler::Gas.is_available() {
                Box::new(ExternalAssembler::Gas.with_config(config))
            } else {
                Box::new(DirectEncoder::new(arch))
            }
        }
        TargetArch::Wasm32 => {
            // Wasm 使用 wat2wasm 或直接编码
            Box::new(DirectEncoder::new(arch))
        }
    }
}
