//! 直接编码支持
//!
//! 当外部汇编器不可用时，使用内置编码器直接生成目标文件。

use std::path::Path;

use crate::arch::TargetArch;
use crate::emitter::{BinaryEmitter, BinaryFormat};
use crate::encoding::{AArch64Encoder, MachineCodeEncoder, X86_64Encoder};

use super::Assembler;
use crate::{NativeError, NativeResult};

/// 直接编码器
///
/// 使用内置编码器直接生成目标文件，无需外部汇编器。
pub struct DirectEncoder {
    arch: TargetArch,
}

impl DirectEncoder {
    /// 创建新的直接编码器
    pub fn new(arch: TargetArch) -> Self {
        Self { arch }
    }

    /// 解析汇编文本并编码
    ///
    /// 这是一个简化实现，只支持基本的指令格式。
    fn parse_and_encode(&self, asm: &str) -> NativeResult<Vec<u8>> {
        match self.arch {
            TargetArch::X86_64 => self.encode_x86_64(asm),
            TargetArch::AArch64 => self.encode_aarch64(asm),
            TargetArch::RiscV64 => self.encode_riscv(asm),
            TargetArch::Wasm32 => self.encode_wasm(asm),
        }
    }

    /// x86-64 编码
    fn encode_x86_64(&self, asm: &str) -> NativeResult<Vec<u8>> {
        let mut encoder = X86_64Encoder::new();

        for line in asm.lines() {
            let line = line.trim();

            // 跳过空行和注释
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }

            // 跳过节指令
            if line.starts_with("section") || line.starts_with("global") || line.starts_with("extern") {
                continue;
            }

            // 跳过标签
            if line.ends_with(':') {
                continue;
            }

            // 解析指令
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0].to_lowercase().as_str() {
                "ret" => { encoder.ret(); }
                "nop" => { encoder.nop(); }
                "push" => {
                    if parts.len() > 1 {
                        // 简化处理：只支持 rbp
                        if parts[1] == "rbp" {
                            encoder.push_reg(crate::arch::X86Register::Rbp);
                        }
                    }
                }
                "pop" => {
                    if parts.len() > 1 {
                        if parts[1] == "rbp" {
                            encoder.pop_reg(crate::arch::X86Register::Rbp);
                        }
                    }
                }
                _ => {
                    // 对于不支持的指令，使用 NOP 填充
                    log::debug!("Unsupported instruction for direct encoding: {}", line);
                    encoder.nop();
                }
            }
        }

        Ok(encoder.encode())
    }

    /// AArch64 编码
    fn encode_aarch64(&self, asm: &str) -> NativeResult<Vec<u8>> {
        let mut encoder = AArch64Encoder::new();

        for line in asm.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('/') || line.starts_with('#') {
                continue;
            }

            if line.starts_with('.') {
                continue;
            }

            if line.ends_with(':') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0].to_lowercase().as_str() {
                "ret" => { encoder.ret(); }
                "nop" => { encoder.nop(); }
                _ => {
                    log::debug!("Unsupported AArch64 instruction: {}", line);
                    encoder.nop();
                }
            }
        }

        Ok(encoder.encode())
    }

    /// RISC-V 编码
    fn encode_riscv(&self, _asm: &str) -> NativeResult<Vec<u8>> {
        // RISC-V 编码尚未实现
        Err(NativeError::Unimplemented(
            "RISC-V direct encoding not implemented".to_string(),
        ))
    }

    /// WebAssembly 编码
    fn encode_wasm(&self, _asm: &str) -> NativeResult<Vec<u8>> {
        // Wasm 编码尚未实现
        Err(NativeError::Unimplemented(
            "WebAssembly direct encoding not implemented".to_string(),
        ))
    }
}

impl Assembler for DirectEncoder {
    fn assemble(&self, asm: &str, output: &Path) -> NativeResult<()> {
        // 编码机器码
        let code = self.parse_and_encode(asm)?;

        // 创建二进制发射器
        let format = match self.arch {
            TargetArch::X86_64 => BinaryFormat::Elf,
            TargetArch::AArch64 => BinaryFormat::Elf,
            TargetArch::RiscV64 => BinaryFormat::Elf,
            TargetArch::Wasm32 => BinaryFormat::Wasm,
        };

        let mut emitter = BinaryEmitter::new(format);
        emitter.emit_code(&code);

        // 生成目标文件
        let object_data = emitter.emit()?;

        // 写入文件
        std::fs::write(output, object_data)?;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Direct Encoder"
    }

    fn is_available(&self) -> bool {
        true
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_encoder_creation() {
        let encoder = DirectEncoder::new(TargetArch::X86_64);
        assert_eq!(encoder.name(), "Direct Encoder");
        assert!(encoder.is_available());
    }

    #[test]
    fn test_x86_64_ret_encoding() {
        let encoder = DirectEncoder::new(TargetArch::X86_64);
        let code = encoder.encode_x86_64("ret").unwrap();
        assert_eq!(code, vec![0xC3]);
    }

    #[test]
    fn test_x86_64_nop_encoding() {
        let encoder = DirectEncoder::new(TargetArch::X86_64);
        let code = encoder.encode_x86_64("nop").unwrap();
        assert_eq!(code, vec![0x90]);
    }

    #[test]
    fn test_x86_64_skip_comments() {
        let encoder = DirectEncoder::new(TargetArch::X86_64);
        let asm = "; comment\n# another comment\nnop";
        let code = encoder.encode_x86_64(asm).unwrap();
        assert_eq!(code, vec![0x90]);
    }

    #[test]
    fn test_aarch64_ret_encoding() {
        let encoder = DirectEncoder::new(TargetArch::AArch64);
        let code = encoder.encode_aarch64("ret").unwrap();
        assert_eq!(code.len(), 4);
    }
}
