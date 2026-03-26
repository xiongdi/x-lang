//! 目标平台和文件类型定义

/// 支持的目标平台（十大后端）
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Target {
    // === 源码生成后端 ===
    /// TypeScript - 浏览器/Node.js/Deno
    TypeScript,
    /// Python 源代码 - .py 文件
    Python,
    /// Rust 源代码 - .rs 文件
    Rust,
    /// Go 源代码 - 云原生/网络编程
    Go,
    /// Swift 源代码 - Apple 生态
    Swift,

    // === 字节码/IR 后端 ===
    /// Java虚拟机（JVM）- Java字节码
    Jvm,
    /// .NET平台 - CIL字节码
    DotNet,

    // === 原生编译后端 ===
    /// Zig 后端 - Native/Wasm，通过 Zig 编译器
    Zig,
    /// LLVM 后端 - 直接生成 LLVM IR
    Llvm,
    /// Native 后端 - LIR 直译机器码，无需外部编译器
    Native,
}

impl Target {
    /// 获取目标平台的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            Target::TypeScript => "typescript",
            Target::Python => "python",
            Target::Rust => "rust",
            Target::Go => "go",
            Target::Swift => "swift",
            Target::Jvm => "jvm",
            Target::DotNet => "dotnet",
            Target::Zig => "zig",
            Target::Llvm => "llvm",
            Target::Native => "native",
        }
    }

    /// 从字符串解析目标平台
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "ts" | "typescript" => Some(Target::TypeScript),
            "python" | "py" => Some(Target::Python),
            "rust" | "rs" => Some(Target::Rust),
            "go" | "golang" => Some(Target::Go),
            "swift" => Some(Target::Swift),
            "jvm" | "java" => Some(Target::Jvm),
            "dotnet" | "net" | "cil" => Some(Target::DotNet),
            "zig" => Some(Target::Zig),
            "llvm" => Some(Target::Llvm),
            "native" => Some(Target::Native),
            _ => None,
        }
    }

    /// 获取目标平台的默认文件扩展名
    pub fn default_extension(&self) -> &'static str {
        match self {
            Target::TypeScript => "ts",
            Target::Python => "py",
            Target::Rust => "rs",
            Target::Go => "go",
            Target::Swift => "swift",
            Target::Jvm => "jar",
            Target::DotNet => "dll",
            Target::Zig => "zig",
            Target::Llvm => "ll",
            Target::Native => "exe",
        }
    }

    /// 检查目标平台是否需要外部编译器
    pub fn requires_external_compiler(&self) -> bool {
        matches!(
            self,
            Target::TypeScript |
            Target::Python |
            Target::Rust |
            Target::Go |
            Target::Swift |
            Target::Jvm |
            Target::DotNet |
            Target::Zig |
            Target::Llvm
        )
    }

    /// 检查目标平台是否需要运行时
    pub fn requires_runtime(&self) -> bool {
        matches!(
            self,
            Target::TypeScript | Target::Python | Target::Jvm | Target::DotNet
        )
    }

    /// 检查目标平台是否生成源代码
    pub fn is_source_backend(&self) -> bool {
        matches!(
            self,
            Target::TypeScript | Target::Python | Target::Rust | Target::Go | Target::Swift
        )
    }

    /// 检查目标平台是否生成字节码/IR
    pub fn is_bytecode_backend(&self) -> bool {
        matches!(self, Target::Jvm | Target::DotNet | Target::Llvm)
    }

    /// 检查目标平台是否生成原生代码
    pub fn is_native_backend(&self) -> bool {
        matches!(self, Target::Zig | Target::Native)
    }

    /// 获取后端 crate 名称
    pub fn backend_crate(&self) -> &'static str {
        match self {
            Target::TypeScript => "x-codegen-typescript",
            Target::Python => "x-codegen-python",
            Target::Rust => "x-codegen-rust",
            Target::Go => "x-codegen-go",
            Target::Swift => "x-codegen-swift",
            Target::Jvm => "x-codegen-jvm",
            Target::DotNet => "x-codegen-dotnet",
            Target::Zig => "x-codegen-zig",
            Target::Llvm => "x-codegen-llvm",
            Target::Native => "x-codegen-native",
        }
    }
}

/// 输出文件类型
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FileType {
    /// 可执行文件（.exe）
    Executable,
    /// 目标文件（.o/.obj）
    ObjectFile,
    /// JVM字节码（.class）
    JvmBytecode,
    /// JAR文件（.jar）
    JarFile,
    /// .NET程序集（.dll/.exe）
    DotNetAssembly,
    /// .NET模块（.netmodule）
    DotNetModule,
    /// TypeScript文件（.ts）
    TypeScript,
    /// JavaScript文件（.js）
    JavaScript,
    /// Python 源代码（.py）
    Python,
    /// Zig 源代码（.zig）
    Zig,
    /// Rust 源代码（.rs）
    Rust,
    /// Go 源代码（.go）
    Go,
    /// Swift 源代码（.swift）
    Swift,
    /// C# 源代码（.cs）
    CSharp,
    /// Java 源代码（.java）
    Java,
    /// LLVM IR（.ll）
    LlvmIr,
    /// LLVM Bitcode（.bc）
    LlvmBitcode,
    /// WebAssembly文件（.wasm）
    Wasm,
    /// WebAssembly文本（.wat）
    Wat,
    /// 汇编文件（.s/.asm）
    Assembly,
}

impl FileType {
    /// 获取文件类型的默认扩展名
    pub fn extension(&self) -> &'static str {
        match self {
            FileType::Executable => "exe",
            FileType::ObjectFile => "o",
            FileType::JvmBytecode => "class",
            FileType::JarFile => "jar",
            FileType::DotNetAssembly => "dll",
            FileType::DotNetModule => "netmodule",
            FileType::TypeScript => "ts",
            FileType::JavaScript => "js",
            FileType::Python => "py",
            FileType::Zig => "zig",
            FileType::Rust => "rs",
            FileType::Go => "go",
            FileType::Swift => "swift",
            FileType::CSharp => "cs",
            FileType::Java => "java",
            FileType::LlvmIr => "ll",
            FileType::LlvmBitcode => "bc",
            FileType::Wasm => "wasm",
            FileType::Wat => "wat",
            FileType::Assembly => "s",
        }
    }

    /// 获取文件类型的描述
    pub fn description(&self) -> &'static str {
        match self {
            FileType::Executable => "Executable",
            FileType::ObjectFile => "Object file",
            FileType::JvmBytecode => "JVM bytecode",
            FileType::JarFile => "JAR file",
            FileType::DotNetAssembly => ".NET assembly",
            FileType::DotNetModule => ".NET module",
            FileType::TypeScript => "TypeScript source",
            FileType::JavaScript => "JavaScript source",
            FileType::Python => "Python source",
            FileType::Zig => "Zig source",
            FileType::Rust => "Rust source",
            FileType::Go => "Go source",
            FileType::Swift => "Swift source",
            FileType::CSharp => "C# source",
            FileType::Java => "Java source",
            FileType::LlvmIr => "LLVM IR",
            FileType::LlvmBitcode => "LLVM bitcode",
            FileType::Wasm => "WebAssembly",
            FileType::Wat => "WebAssembly text",
            FileType::Assembly => "Assembly",
        }
    }
}
