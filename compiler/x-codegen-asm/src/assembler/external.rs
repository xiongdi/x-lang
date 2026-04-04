//! 外部汇编器集成
//!
//! 提供 MASM (ml64)、NASM、GAS 和 Clang 的集成支持。
//!
//! # Windows 平台
//!
//! Windows 使用 ml64 (Microsoft Macro Assembler) 和 link.exe (Microsoft Linker)，
//! 这些工具随 Visual Studio/Windows SDK 提供，无需额外安装。

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{NativeError, NativeResult, TargetOS};

use super::Assembler;

/// 支持的外部汇编器
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalAssembler {
    /// MASM (ml64) - Windows x64 首选
    Masm,
    /// NASM (Netwide Assembler) - 跨平台
    Nasm,
    /// GNU Assembler (GAS) - Linux/macOS
    Gas,
    /// Clang/LLVM 集成汇编器
    Clang,
}

/// 输出目标文件格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputObjectFormat {
    /// ELF64 (Linux/BSD)
    Elf64,
    /// Mach-O 64 (macOS)
    MachO64,
    /// PE/COFF 64 (Windows)
    Pe64,
    /// WebAssembly
    Wasm,
}

impl OutputObjectFormat {
    /// 根据操作系统获取默认格式
    pub fn from_os(os: TargetOS) -> Self {
        match os {
            TargetOS::Linux => OutputObjectFormat::Elf64,
            TargetOS::MacOS => OutputObjectFormat::MachO64,
            TargetOS::Windows => OutputObjectFormat::Pe64,
        }
    }

    /// 获取文件扩展名
    pub fn extension(&self) -> &'static str {
        match self {
            OutputObjectFormat::Elf64 => "o",
            OutputObjectFormat::MachO64 => "o",
            OutputObjectFormat::Pe64 => "obj",
            OutputObjectFormat::Wasm => "wasm",
        }
    }
}

/// 汇编器配置
#[derive(Debug, Clone)]
pub struct AssemblerConfig {
    /// 汇编器可执行文件路径（None 表示使用 PATH）
    pub assembler_path: Option<PathBuf>,
    /// 输出格式
    pub format: OutputObjectFormat,
    /// 额外的命令行参数
    pub extra_flags: Vec<String>,
    /// 是否生成调试信息
    pub debug_info: bool,
}

impl Default for AssemblerConfig {
    fn default() -> Self {
        Self {
            assembler_path: None,
            format: OutputObjectFormat::Pe64,
            extra_flags: Vec::new(),
            debug_info: false,
        }
    }
}

impl AssemblerConfig {
    /// 创建指定操作系统的配置
    pub fn for_os(os: TargetOS) -> Self {
        Self {
            format: OutputObjectFormat::from_os(os),
            ..Default::default()
        }
    }
}

// ============================================================================
// Visual Studio 工具查找
// ============================================================================

/// Windows 平台默认的 ml64.exe 路径
#[cfg(windows)]
const DEFAULT_ML64_PATH: &str = r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64\ml64.exe";

/// Windows 平台默认的 link.exe 路径
#[cfg(windows)]
const DEFAULT_LINK_PATH: &str = r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\14.44.35207\bin\Hostx64\x64\link.exe";

/// 在 Visual Studio 安装目录中查找指定工具
///
/// 支持 Community, Professional, Enterprise 和 BuildTools 版本。
fn find_tool_in_visual_studio(tool_name: &str) -> Option<PathBuf> {
    // 首先尝试使用硬编码的默认路径
    #[cfg(windows)]
    {
        if tool_name == "ml64.exe" {
            let default_path = PathBuf::from(DEFAULT_ML64_PATH);
            if default_path.exists() {
                log::debug!(
                    "Using hardcoded {} path: {}",
                    tool_name,
                    default_path.display()
                );
                return Some(default_path);
            }
        }
    }

    // 常见的 Visual Studio 安装位置
    let search_paths = [
        // Visual Studio 2022 (Community, Professional, Enterprise, BuildTools)
        r"C:\Program Files\Microsoft Visual Studio\2022",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2022",
        // Visual Studio 2019
        r"C:\Program Files\Microsoft Visual Studio\2019",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2019",
        // Visual Studio 2017
        r"C:\Program Files\Microsoft Visual Studio\2017",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2017",
    ];

    for base_path in &search_paths {
        if let Ok(entries) = std::fs::read_dir(base_path) {
            for entry in entries.flatten() {
                let edition_path = entry.path();
                // 支持 Community, Professional, Enterprise, BuildTools 等版本
                // 查找 VC/Tools/MSVC/<version>/bin/Hostx64/x64/<tool>
                let msvc_path = edition_path.join("VC").join("Tools").join("MSVC");
                if let Ok(versions) = std::fs::read_dir(&msvc_path) {
                    for version in versions.flatten() {
                        let tool_path = version
                            .path()
                            .join("bin")
                            .join("Hostx64")
                            .join("x64")
                            .join(tool_name);
                        if tool_path.exists() {
                            log::debug!("Found {} at: {}", tool_name, tool_path.display());
                            return Some(tool_path);
                        }
                    }
                }
            }
        }
    }

    log::debug!("{} not found in Visual Studio installations", tool_name);
    None
}

impl ExternalAssembler {
    /// 获取可执行文件名
    pub fn executable_name(&self) -> &'static str {
        match self {
            ExternalAssembler::Masm => "ml64",
            ExternalAssembler::Nasm => "nasm",
            ExternalAssembler::Gas => "as",
            ExternalAssembler::Clang => "clang",
        }
    }

    /// 获取汇编器路径
    ///
    /// 对于 MASM (ml64)，会自动检测 Visual Studio 安装路径。
    /// 对于其他汇编器，在 PATH 中查找。
    pub fn find_executable(&self) -> Option<PathBuf> {
        // 首先尝试在 PATH 中查找
        if let Ok(path) = which::which(self.executable_name()) {
            return Some(path);
        }

        // 对于 MASM，尝试在 Visual Studio 安装目录中查找
        if *self == ExternalAssembler::Masm {
            return Self::find_masm_in_visual_studio();
        }

        None
    }

    /// 在 Visual Studio 安装目录中查找 ml64.exe
    fn find_masm_in_visual_studio() -> Option<PathBuf> {
        find_tool_in_visual_studio("ml64.exe")
    }

    /// 使用配置创建汇编器实例
    pub fn with_config(self, config: AssemblerConfig) -> ConfiguredAssembler {
        ConfiguredAssembler {
            assembler: self,
            config,
        }
    }
}

impl Assembler for ExternalAssembler {
    fn assemble(&self, asm: &str, output: &Path) -> NativeResult<()> {
        self.with_config(AssemblerConfig::default())
            .assemble(asm, output)
    }

    fn name(&self) -> &'static str {
        match self {
            ExternalAssembler::Masm => "MASM (ml64)",
            ExternalAssembler::Nasm => "NASM",
            ExternalAssembler::Gas => "GAS",
            ExternalAssembler::Clang => "Clang",
        }
    }

    fn is_available(&self) -> bool {
        self.find_executable().is_some()
    }
}

/// 配置后的汇编器
pub struct ConfiguredAssembler {
    assembler: ExternalAssembler,
    config: AssemblerConfig,
}

impl ConfiguredAssembler {
    /// 创建新的配置汇编器
    pub fn new(assembler: ExternalAssembler, config: AssemblerConfig) -> Self {
        Self { assembler, config }
    }
}

impl Assembler for ConfiguredAssembler {
    fn assemble(&self, asm: &str, output: &Path) -> NativeResult<()> {
        // 根据汇编器类型确定文件扩展名
        let ext = match self.assembler {
            ExternalAssembler::Masm => "asm",
            ExternalAssembler::Nasm => "asm",
            ExternalAssembler::Gas => "s",
            ExternalAssembler::Clang => "s",
        };

        // 创建临时汇编文件
        let temp_asm = output.with_extension(ext);
        std::fs::write(&temp_asm, asm)?;

        let result = self.assemble_file(&temp_asm, output);

        // 清理临时文件
        let _ = std::fs::remove_file(&temp_asm);

        result
    }

    fn name(&self) -> &'static str {
        self.assembler.name()
    }

    fn is_available(&self) -> bool {
        self.assembler.is_available()
    }
}

impl ConfiguredAssembler {
    /// 汇编文件
    fn assemble_file(&self, input: &Path, output: &Path) -> NativeResult<()> {
        let executable = self
            .config
            .assembler_path
            .as_ref()
            .cloned()
            .unwrap_or_else(|| self.assembler.executable_name().into());

        let args = self.build_args(input, output);

        log::debug!("Running {} with args: {:?}", executable.display(), args);

        let output_result = Command::new(&executable)
            .args(&args)
            .output()
            .map_err(|e| {
                NativeError::CodegenError(format!(
                    "Failed to execute {}: {}",
                    executable.display(),
                    e
                ))
            })?;

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            let stdout = String::from_utf8_lossy(&output_result.stdout);
            return Err(NativeError::CodegenError(format!(
                "{} failed:\n{}\n{}",
                self.assembler.name(),
                stderr,
                stdout
            )));
        }

        Ok(())
    }

    /// 构建命令行参数
    fn build_args(&self, input: &Path, output: &Path) -> Vec<String> {
        let mut args = Vec::new();

        match self.assembler {
            ExternalAssembler::Masm => {
                // ml64 参数
                // /c - 只汇编，不链接
                // /Fo - 输出文件名
                // /Zi - 生成调试信息
                args.push("/c".to_string());

                if self.config.debug_info {
                    args.push("/Zi".to_string());
                }

                args.push("/Fo".to_string());
                args.push(output.to_string_lossy().to_string());

                // 额外包含路径（可选）
                args.extend(self.config.extra_flags.clone());

                args.push(input.to_string_lossy().to_string());
            }
            ExternalAssembler::Nasm => {
                // NASM 参数
                match self.config.format {
                    OutputObjectFormat::Elf64 => {
                        args.push("-f".to_string());
                        args.push("elf64".to_string());
                    }
                    OutputObjectFormat::MachO64 => {
                        args.push("-f".to_string());
                        args.push("macho64".to_string());
                    }
                    OutputObjectFormat::Pe64 => {
                        args.push("-f".to_string());
                        args.push("win64".to_string());
                    }
                    OutputObjectFormat::Wasm => {
                        // NASM 不支持 Wasm 输出
                    }
                }

                if self.config.debug_info {
                    args.push("-g".to_string());
                }

                args.push("-o".to_string());
                args.push(output.to_string_lossy().to_string());
                args.push(input.to_string_lossy().to_string());
            }
            ExternalAssembler::Gas => {
                // GNU Assembler 参数
                match self.config.format {
                    OutputObjectFormat::Elf64 => {
                        args.push("--64".to_string());
                    }
                    OutputObjectFormat::MachO64 => {
                        // macOS 使用 as，不需要特殊标志
                    }
                    OutputObjectFormat::Pe64 => {
                        // GAS 通常不用于 Windows
                    }
                    OutputObjectFormat::Wasm => {}
                }

                if self.config.debug_info {
                    args.push("--gdwarf-2".to_string());
                }

                args.push("-o".to_string());
                args.push(output.to_string_lossy().to_string());
                args.push(input.to_string_lossy().to_string());
            }
            ExternalAssembler::Clang => {
                // Clang 参数
                args.push("-c".to_string());

                match self.config.format {
                    OutputObjectFormat::Elf64 => {
                        args.push("-target".to_string());
                        args.push("x86_64-linux-gnu".to_string());
                    }
                    OutputObjectFormat::MachO64 => {
                        args.push("-target".to_string());
                        args.push("x86_64-apple-darwin".to_string());
                    }
                    OutputObjectFormat::Pe64 => {
                        args.push("-target".to_string());
                        args.push("x86_64-pc-windows-msvc".to_string());
                    }
                    OutputObjectFormat::Wasm => {
                        args.push("-target".to_string());
                        args.push("wasm32".to_string());
                    }
                }

                if self.config.debug_info {
                    args.push("-g".to_string());
                }

                args.push("-o".to_string());
                args.push(output.to_string_lossy().to_string());
                args.push(input.to_string_lossy().to_string());
            }
        }

        args
    }
}

// ============================================================================
// Microsoft Linker 集成
// ============================================================================

/// Microsoft Linker 配置
#[derive(Debug, Clone)]
pub struct LinkerConfig {
    /// link.exe 路径（None 表示使用 PATH）
    pub linker_path: Option<PathBuf>,
    /// 额外的库路径
    pub lib_paths: Vec<PathBuf>,
    /// 额外的库
    pub libraries: Vec<String>,
    /// 是否生成调试信息
    pub debug_info: bool,
    /// 子系统类型
    pub subsystem: Subsystem,
    /// 入口点
    pub entry_point: Option<String>,
}

impl Default for LinkerConfig {
    fn default() -> Self {
        Self {
            linker_path: None,
            lib_paths: Vec::new(),
            libraries: Vec::new(),
            debug_info: false,
            subsystem: Subsystem::Console,
            entry_point: None,
        }
    }
}

/// Windows 子系统类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Subsystem {
    /// 控制台应用程序
    Console,
    /// Windows GUI 应用程序
    Windows,
}

impl Subsystem {
    pub fn as_str(&self) -> &'static str {
        match self {
            Subsystem::Console => "CONSOLE",
            Subsystem::Windows => "WINDOWS",
        }
    }
}

/// Microsoft Linker (link.exe)
pub struct MicrosoftLinker {
    config: LinkerConfig,
}

impl MicrosoftLinker {
    /// 创建新的链接器实例
    pub fn new(config: LinkerConfig) -> Self {
        Self { config }
    }

    /// 查找 link.exe 路径
    ///
    /// 首先在 PATH 中查找，然后在 Visual Studio 安装目录中查找。
    pub fn find_linker() -> Option<PathBuf> {
        // 首先尝试在 PATH 中查找（排除 Git 的 link.exe）
        if let Ok(path) = which::which("link") {
            // 检查是否是 Microsoft 的 link.exe（而不是 Git 的）
            if let Some(parent) = path.parent() {
                if !parent.to_string_lossy().contains("Git") {
                    return Some(path);
                }
            }
        }

        // 在 Visual Studio 安装目录中查找
        Self::find_linker_in_visual_studio()
    }

    /// 在 Visual Studio 安装目录中查找 link.exe
    fn find_linker_in_visual_studio() -> Option<PathBuf> {
        // 首先尝试使用硬编码的默认路径
        #[cfg(windows)]
        {
            let default_path = PathBuf::from(DEFAULT_LINK_PATH);
            if default_path.exists() {
                log::debug!("Using hardcoded link.exe path: {}", default_path.display());
                return Some(default_path);
            }
        }

        // 常见的 Visual Studio 安装位置
        let search_paths = [
            // Visual Studio 2022 (Community, Professional, Enterprise, BuildTools)
            r"C:\Program Files\Microsoft Visual Studio\2022",
            r"C:\Program Files (x86)\Microsoft Visual Studio\2022",
            // Visual Studio 2019
            r"C:\Program Files\Microsoft Visual Studio\2019",
            r"C:\Program Files (x86)\Microsoft Visual Studio\2019",
            // Visual Studio 2017
            r"C:\Program Files\Microsoft Visual Studio\2017",
            r"C:\Program Files (x86)\Microsoft Visual Studio\2017",
        ];

        for base_path in &search_paths {
            if let Ok(entries) = std::fs::read_dir(base_path) {
                for entry in entries.flatten() {
                    let edition_path = entry.path();
                    // 查找 VC/Tools/MSVC/<version>/bin/Hostx64/x64/link.exe
                    let msvc_path = edition_path.join("VC").join("Tools").join("MSVC");
                    if let Ok(versions) = std::fs::read_dir(&msvc_path) {
                        for version in versions.flatten() {
                            let link_path = version
                                .path()
                                .join("bin")
                                .join("Hostx64")
                                .join("x64")
                                .join("link.exe");
                            if link_path.exists() {
                                log::debug!("Found link.exe at: {}", link_path.display());
                                return Some(link_path);
                            }
                        }
                    }
                }
            }
        }

        log::debug!("link.exe not found in Visual Studio installations");
        None
    }

    /// 检查 link.exe 是否可用
    pub fn is_available() -> bool {
        Self::find_linker().is_some()
    }

    /// 链接目标文件生成可执行文件
    pub fn link(&self, objects: &[&Path], output: &Path) -> NativeResult<()> {
        let linker = self
            .config
            .linker_path
            .as_ref()
            .cloned()
            .or_else(|| Self::find_linker())
            .unwrap_or_else(|| "link".into());

        let args = self.build_link_args(objects, output);

        log::debug!("Running link.exe with args: {:?}", args);

        let output_result = Command::new(&linker)
            .args(&args)
            .output()
            .map_err(|e| NativeError::CodegenError(format!("Failed to execute link.exe: {}", e)))?;

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            let stdout = String::from_utf8_lossy(&output_result.stdout);
            return Err(NativeError::CodegenError(format!(
                "link.exe failed:\n{}\n{}",
                stderr, stdout
            )));
        }

        Ok(())
    }

    fn build_link_args(&self, objects: &[&Path], output: &Path) -> Vec<String> {
        let mut args = Vec::new();

        // /OUT: - 输出文件
        args.push(format!("/OUT:{}", output.to_string_lossy()));

        // /SUBSYSTEM: - 子系统
        args.push(format!("/SUBSYSTEM:{}", self.config.subsystem.as_str()));

        // /ENTRY: - 入口点
        if let Some(entry) = &self.config.entry_point {
            args.push(format!("/ENTRY:{}", entry));
        }

        // /DEBUG - 生成调试信息
        if self.config.debug_info {
            args.push("/DEBUG".to_string());
        }

        // /LIBPATH: - 库路径
        for lib_path in &self.config.lib_paths {
            args.push(format!("/LIBPATH:{}", lib_path.to_string_lossy()));
        }

        // 目标文件
        for obj in objects {
            args.push(obj.to_string_lossy().to_string());
        }

        // 额外的库
        for lib in &self.config.libraries {
            args.push(format!("{}.lib", lib));
        }

        // 默认库
        args.push("kernel32.lib".to_string());
        args.push("user32.lib".to_string());

        args
    }
}

// ============================================================================
// 单元测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assembler_names() {
        assert_eq!(ExternalAssembler::Masm.executable_name(), "ml64");
        assert_eq!(ExternalAssembler::Nasm.executable_name(), "nasm");
        assert_eq!(ExternalAssembler::Gas.executable_name(), "as");
        assert_eq!(ExternalAssembler::Clang.executable_name(), "clang");
    }

    #[test]
    fn test_output_format_from_os() {
        assert_eq!(
            OutputObjectFormat::from_os(TargetOS::Linux),
            OutputObjectFormat::Elf64
        );
        assert_eq!(
            OutputObjectFormat::from_os(TargetOS::MacOS),
            OutputObjectFormat::MachO64
        );
        assert_eq!(
            OutputObjectFormat::from_os(TargetOS::Windows),
            OutputObjectFormat::Pe64
        );
    }

    #[test]
    fn test_output_format_extension() {
        assert_eq!(OutputObjectFormat::Elf64.extension(), "o");
        assert_eq!(OutputObjectFormat::MachO64.extension(), "o");
        assert_eq!(OutputObjectFormat::Pe64.extension(), "obj");
        assert_eq!(OutputObjectFormat::Wasm.extension(), "wasm");
    }

    #[test]
    fn test_configured_assembler_masm_args() {
        let config = AssemblerConfig {
            format: OutputObjectFormat::Pe64,
            debug_info: true,
            ..Default::default()
        };
        let assembler = ConfiguredAssembler::new(ExternalAssembler::Masm, config);

        let input = Path::new("test.asm");
        let output = Path::new("test.obj");
        let args = assembler.build_args(input, output);

        assert!(args.contains(&"/c".to_string()));
        assert!(args.contains(&"/Zi".to_string()));
        assert!(args.iter().any(|a| a.starts_with("/Fo")));
    }

    #[test]
    fn test_configured_assembler_nasm_args() {
        let config = AssemblerConfig {
            format: OutputObjectFormat::Elf64,
            debug_info: true,
            ..Default::default()
        };
        let assembler = ConfiguredAssembler::new(ExternalAssembler::Nasm, config);

        let input = Path::new("test.asm");
        let output = Path::new("test.o");
        let args = assembler.build_args(input, output);

        assert!(args.contains(&"-f".to_string()));
        assert!(args.contains(&"elf64".to_string()));
        assert!(args.contains(&"-g".to_string()));
    }

    #[test]
    fn test_configured_assembler_gas_args() {
        let config = AssemblerConfig {
            format: OutputObjectFormat::Elf64,
            ..Default::default()
        };
        let assembler = ConfiguredAssembler::new(ExternalAssembler::Gas, config);

        let input = Path::new("test.s");
        let output = Path::new("test.o");
        let args = assembler.build_args(input, output);

        assert!(args.contains(&"--64".to_string()));
    }

    #[test]
    fn test_linker_config_default() {
        let config = LinkerConfig::default();
        assert_eq!(config.subsystem, Subsystem::Console);
        assert!(config.entry_point.is_none());
    }

    #[test]
    fn test_linker_build_args() {
        let config = LinkerConfig {
            debug_info: true,
            entry_point: Some("main".to_string()),
            ..Default::default()
        };
        let linker = MicrosoftLinker::new(config);

        let objects = vec![Path::new("test.obj")];
        let output = Path::new("test.exe");
        let args = linker.build_link_args(&objects, output);

        assert!(args.iter().any(|a| a.starts_with("/OUT:")));
        assert!(args.iter().any(|a| a.starts_with("/SUBSYSTEM:")));
        assert!(args.contains(&"/DEBUG".to_string()));
        assert!(args.iter().any(|a| a.starts_with("/ENTRY:")));
    }
}
