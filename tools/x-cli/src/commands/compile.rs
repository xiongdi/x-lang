use crate::pipeline;
use crate::utils;
use x_codegen::CodeGenerator;
use x_codegen::Target;
use x_codegen_asm::{NativeBackend, NativeBackendConfig, TargetArch};
use x_codegen_csharp::{CSharpBackend, CSharpConfig};
use x_codegen_rust::{RustBackend, RustBackendConfig};
use x_codegen_typescript::{TypeScriptBackend, TypeScriptBackendConfig};
use x_codegen_zig::{ZigBackend, ZigBackendConfig, ZigTarget};

#[allow(unused_variables)]
pub fn exec(
    file: &str,
    output: Option<&str>,
    emit: Option<&str>,
    no_link: bool,
    release: bool,
    target: Option<&str>,
) -> Result<(), String> {
    let content =
        std::fs::read_to_string(file).map_err(|e| format!("无法读取文件 {}: {}", file, e))?;

    if let Some(stage) = emit {
        return emit_stage(file, &content, stage);
    }

    let out_path = output.unwrap_or_else(|| file.strip_suffix(".x").unwrap_or(file));

    // Parse target
    let parsed_target = match target {
        None | Some("native") => Target::Asm,
        Some("wasm" | "wasm32-wasi" | "wasm32-freestanding") => Target::Zig,
        Some("ts" | "typescript") => Target::TypeScript,
        Some("zig") => Target::Zig,
        Some(t) => {
            if let Some(t) = Target::from_str(t) {
                t
            } else {
                return Err(format!(
                    "未知目标平台: {}（支持: native, wasm, wasm32-wasi, wasm32-freestanding, zig, ts/typescript）",
                    t
                ));
            }
        }
    };

    // Run the full compiler pipeline: source → AST → HIR → MIR → LIR
    let pipeline_output = pipeline::run_pipeline(&content)?;

    // All backends use the full pipeline via LIR
    match parsed_target {
        // ── Native 后端 - 直接生成机器码，无需外部编译器 ───────────────────
        Target::Asm => {
            // Native 后端直接生成机器码
            use x_codegen_asm::{TargetArch, TargetOS};
            // 自动检测当前架构和操作系统
            let arch = if cfg!(target_arch = "x86_64") {
                TargetArch::X86_64
            } else if cfg!(target_arch = "aarch64") {
                TargetArch::AArch64
            } else {
                return Err(format!(
                    "Native 后端尚不支持此架构: {}",
                    std::env::consts::ARCH
                ));
            };
            let os = if cfg!(target_os = "windows") {
                TargetOS::Windows
            } else if cfg!(target_os = "linux") {
                TargetOS::Linux
            } else if cfg!(target_os = "macos") {
                TargetOS::MacOS
            } else {
                return Err(format!(
                    "Native 后端尚不支持此操作系统: {}",
                    std::env::consts::OS
                ));
            };

            let mut backend = NativeBackend::new(NativeBackendConfig {
                output_dir: None,
                optimize: release,
                debug_info: !release,
                arch,
                format: x_codegen_asm::OutputFormat::Executable,
                os,
            });

            let codegen_output = backend
                .generate_from_lir(&pipeline_output.lir)
                .map_err(|e| format!("Native代码生成失败: {}", e))?;

            // 获取汇编代码
            let asm_code = String::from_utf8_lossy(&codegen_output.files[0].content);

            // 创建临时目录
            let temp_dir = std::env::temp_dir();
            let asm_path = temp_dir.join("x_native_output.asm");
            let obj_path = temp_dir.join("x_native_output.o");

            // 写入汇编文件
            std::fs::write(&asm_path, asm_code.as_bytes())
                .map_err(|e| format!("无法写入汇编文件: {}", e))?;

            // 输出路径
            let output_path = if os == TargetOS::Windows {
                let path = std::path::PathBuf::from(out_path);
                if path.extension().is_some_and(|e| e == "exe") {
                    path
                } else {
                    path.with_extension("exe")
                }
            } else {
                std::path::PathBuf::from(out_path)
            };

            // 跨平台汇编和链接
            match os {
                TargetOS::Windows => {
                    // Windows: 尝试使用 MSVC 工具链或 MinGW/clang
                    assemble_and_link_windows(&asm_path, &obj_path, &output_path)?;
                }
                TargetOS::MacOS => {
                    // macOS: 使用 clang/ld (Xcode toolchain)
                    assemble_and_link_macos(&asm_path, &obj_path, &output_path, arch)?;
                }
                TargetOS::Linux => {
                    // Linux: 使用 gcc/clang（目标 triple 须与生成汇编的架构一致）
                    assemble_and_link_linux(&asm_path, &obj_path, &output_path, arch)?;
                }
            }

            // 设置可执行权限（非 Windows）
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&output_path)
                    .map_err(|e| e.to_string())?
                    .permissions();
                perms.set_mode(perms.mode() | 0o755);
                std::fs::set_permissions(&output_path, perms).map_err(|e| e.to_string())?;
            }

            // 清理临时文件
            let _ = std::fs::remove_file(&asm_path);
            let _ = std::fs::remove_file(&obj_path);

            println!("编译成功: {}", output_path.display());
        }
        // ── Zig-based targets (Native + Wasm) ────────────────────────────────
        Target::Zig => {
            let zig_target = match target {
                None | Some("native") => ZigTarget::Native,
                Some("wasm" | "wasm32-wasi") => ZigTarget::Wasm32Wasi,
                Some("wasm32-freestanding") => ZigTarget::Wasm32Freestanding,
                _ => ZigTarget::Native,
            };
            let mut backend = ZigBackend::new(ZigBackendConfig {
                output_dir: None,
                optimize: release,
                debug_info: !release,
                target: zig_target,
            });
            let codegen_output = backend
                .generate_from_lir(&pipeline_output.lir)
                .map_err(|e| format!("Zig代码生成失败: {}", e))?;

            let zig_code = String::from_utf8_lossy(&codegen_output.files[0].content);
            let output_path = std::path::PathBuf::from(out_path);

            if let Some(t_str) = target {
                if t_str != "native" {
                    if let Some(tgt) = match t_str {
                        "wasm" | "wasm32-wasi" => Some(ZigTarget::Wasm32Wasi),
                        "wasm32-freestanding" => Some(ZigTarget::Wasm32Freestanding),
                        _ => None,
                    } {
                        utils::status("Target", tgt.as_zig_target());
                    }
                }
            }

            backend
                .compile_zig_code(&zig_code, &output_path)
                .map_err(|e| format!("Zig编译失败: {}", e))?;

            println!("编译成功: {}", output_path.display());
        }

        // ── TypeScript target ─────────────────────────────────────────────────
        // TypeScript is the primary JS-family target.
        // With --no-link: emit a .ts file only.
        // Without --no-link: also invoke `tsc` to compile .ts → .js
        Target::TypeScript => {
            let mut backend = TypeScriptBackend::new(TypeScriptBackendConfig {
                output_dir: None,
                optimize: release,
                debug_info: !release,
            });
            let codegen_output = backend
                .generate_from_lir(&pipeline_output.lir)
                .map_err(|e| format!("TypeScript代码生成失败: {}", e))?;

            let ts_code = String::from_utf8_lossy(&codegen_output.files[0].content);

            // Always write the .ts source
            let ts_out_path = format!("{}.ts", out_path);
            std::fs::write(&ts_out_path, ts_code.as_bytes())
                .map_err(|e| format!("无法写入TypeScript文件 {}: {}", ts_out_path, e))?;

            if no_link {
                println!("已生成TypeScript代码: {}", ts_out_path);
                return Ok(());
            }

            // Compile .ts → .js via tsc
            let out_dir = std::path::Path::new(out_path)
                .parent()
                .unwrap_or(std::path::Path::new("."));
            let out_stem = std::path::Path::new(out_path)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("index");

            match TypeScriptBackend::compile_ts_to_js(&ts_code, out_dir, out_stem) {
                Ok(js_path) => {
                    println!("编译成功: {}", js_path.display());
                }
                Err(e) => {
                    // tsc not available — the .ts file is still useful
                    println!("已生成TypeScript代码: {}", ts_out_path);
                    println!("提示: 安装TypeScript后可编译为JavaScript: npm install -g typescript");
                    println!("      然后运行: tsc {}", ts_out_path);
                    return Err(format!("TypeScript编译失败: {}", e));
                }
            }
        }

        _ => {
            return Err(format!(
                "目标平台 {:?} 尚不支持完整编译到可执行文件。\n\
                 支持的目标: native, wasm, c, ts/typescript",
                parsed_target
            ));
        }
    }

    Ok(())
}

fn emit_stage(file: &str, content: &str, stage: &str) -> Result<(), String> {
    match stage.to_lowercase().as_str() {
        "tokens" => {
            let mut lexer = x_lexer::Lexer::new(content);
            loop {
                match lexer.next_token() {
                    Ok((token, span)) => {
                        println!("{:?}  @ {}..{}", token, span.start, span.end);
                        if matches!(token, x_lexer::token::Token::Eof) {
                            break;
                        }
                    }
                    Err(e) => {
                        return Err(format!("词法错误: {:?}", e));
                    }
                }
            }
            Ok(())
        }
        "ast" => {
            let parser = x_parser::parser::XParser::new();
            let program = parser
                .parse(content)
                .map_err(|e| pipeline::format_parse_error(file, content, &e))?;
            println!("{:#?}", program);
            Ok(())
        }
        // ── Backend source-emit options ──────────────────────────────────────
        "zig" => {
            let parser = x_parser::parser::XParser::new();
            let program = parser
                .parse(content)
                .map_err(|e| pipeline::format_parse_error(file, content, &e))?;

            let mut backend = ZigBackend::new(
                ZigBackendConfig::default(),
            );
            // --emit zig still uses AST for quick inspection; main pipeline uses LIR
            let output = backend
                .generate_from_ast(&program)
                .map_err(|e| format!("Zig代码生成失败: {}", e))?;
            let zig_code = String::from_utf8_lossy(&output.files[0].content);
            println!("{}", zig_code);
            Ok(())
        }
        // TypeScript / JavaScript: both emit from LIR for accuracy
        "ts" | "typescript" | "js" | "javascript" => {
            let output = pipeline::run_pipeline(content)?;
            let mut backend =
                TypeScriptBackend::new(TypeScriptBackendConfig::default());
            let codegen_output = backend
                .generate_from_lir(&output.lir)
                .map_err(|e| format!("TypeScript代码生成失败: {}", e))?;
            let ts_code = String::from_utf8_lossy(&codegen_output.files[0].content);
            println!("{}", ts_code);
            Ok(())
        }
        "dotnet" | "csharp" => {
            // 使用 LIR 进行代码生成（符合编译流水线要求）
            let output = pipeline::run_pipeline(content)?;
            let mut backend = CSharpBackend::new(
                CSharpConfig::default(),
            );
            let codegen_output = backend
                .generate_from_lir(&output.lir)
                .map_err(|e| format!("C#代码生成失败: {}", e))?;
            let csharp_code = String::from_utf8_lossy(&codegen_output.files[0].content);
            println!("{}", csharp_code);
            Ok(())
        }
        "rust" => {
            // 使用 LIR 进行代码生成（符合编译流水线要求）
            let output = pipeline::run_pipeline(content)?;
            let mut backend = RustBackend::new(
                RustBackendConfig::default(),
            );
            let codegen_output = backend
                .generate_from_lir(&output.lir)
                .map_err(|e| format!("Rust代码生成失败: {}", e))?;
            let rust_code = String::from_utf8_lossy(&codegen_output.files[0].content);
            println!("{}", rust_code);
            Ok(())
        }
        "c" => {
            // C 后端使用 Rust 后端生成 C 风格代码
            let output = pipeline::run_pipeline(content)?;
            let mut backend = RustBackend::new(
                RustBackendConfig::default(),
            );
            let codegen_output = backend
                .generate_from_lir(&output.lir)
                .map_err(|e| format!("C代码生成失败: {}", e))?;
            let c_code = String::from_utf8_lossy(&codegen_output.files[0].content);
            println!("{}", c_code);
            Ok(())
        }
        // ── IR dump options ───────────────────────────────────────────────────
        "hir" => {
            let output = pipeline::run_pipeline(content)?;
            println!("{:#?}", output.hir);
            Ok(())
        }
        "mir" => {
            let output = pipeline::run_pipeline(content)?;
            println!("{:#?}", output.mir);
            Ok(())
        }
        "lir" => {
            let output = pipeline::run_pipeline(content)?;
            println!("{:#?}", output.lir);
            Ok(())
        }
        "asm" | "assembly" | "native" => {
            // 输出 Native 后端汇编
            let output = pipeline::run_pipeline(content)?;
            use x_codegen_asm::{NativeBackend, NativeBackendConfig, TargetArch, TargetOS};
            let arch = if cfg!(target_arch = "x86_64") {
                TargetArch::X86_64
            } else if cfg!(target_arch = "aarch64") {
                TargetArch::AArch64
            } else {
                return Err(format!("Native 后端尚不支持此架构: {}", std::env::consts::ARCH));
            };
            let os = if cfg!(target_os = "windows") {
                TargetOS::Windows
            } else if cfg!(target_os = "linux") {
                TargetOS::Linux
            } else if cfg!(target_os = "macos") {
                TargetOS::MacOS
            } else {
                return Err(format!("Unsupported OS: {}", std::env::consts::OS));
            };
            let mut backend = NativeBackend::new(NativeBackendConfig {
                output_dir: None,
                optimize: false,
                debug_info: true,
                arch,
                format: x_codegen_asm::OutputFormat::Assembly,
                os,
            });
            let codegen_output = backend
                .generate_from_lir(&output.lir)
                .map_err(|e| format!("Native代码生成失败: {}", e))?;
            let asm = String::from_utf8_lossy(&codegen_output.files[0].content);
            println!("{}", asm);
            Ok(())
        }
        _ => Err(format!(
            "未知 --emit 阶段: {}\n支持的选项: tokens, ast, hir, mir, lir, zig, ts, js, asm, typescript, javascript, c, rust, dotnet, csharp",
            stage
        )),
    }
}

// ── 跨平台汇编和链接函数 ─────────────────────────────────────────────────────

/// Windows: 尝试使用 clang/clang++ 或 MinGW 工具链
fn assemble_and_link_windows(
    asm_path: &std::path::Path,
    obj_path: &std::path::Path,
    output_path: &std::path::Path,
) -> Result<(), String> {
    // 尝试 clang (LLVM/clang-cl)
    let clang_path = which::which("clang").ok();
    let clangxx_path = which::which("clang++").ok();

    if let (Some(clang), Some(clangxx)) = (clang_path, clangxx_path) {
        // 使用 clang/clang++ 汇编和链接
        let assemble_status = std::process::Command::new(&clang)
            .arg("-c")
            .arg("-o")
            .arg(obj_path)
            .arg(asm_path)
            .arg("-target")
            .arg("x86_64-pc-windows-msvc")
            .status()
            .map_err(|e| format!("无法运行 clang: {}", e))?;

        if assemble_status.success() {
            // 链接
            let link_status = std::process::Command::new(&clangxx)
                .arg("-o")
                .arg(output_path)
                .arg(obj_path)
                .status()
                .map_err(|e| format!("无法运行 clang++: {}", e))?;

            if link_status.success() {
                return Ok(());
            }
        }
    }

    // 尝试 MinGW gcc
    let gcc_path = which::which("gcc").ok();
    let gxx_path = which::which("g++").ok();

    if let (Some(gcc), Some(gxx)) = (gcc_path, gxx_path) {
        let assemble_status = std::process::Command::new(&gcc)
            .arg("-c")
            .arg("-o")
            .arg(obj_path)
            .arg(asm_path)
            .status()
            .map_err(|e| format!("无法运行 gcc: {}", e))?;

        if assemble_status.success() {
            let link_status = std::process::Command::new(&gxx)
                .arg("-o")
                .arg(output_path)
                .arg(obj_path)
                .status()
                .map_err(|e| format!("无法运行 g++: {}", e))?;

            if link_status.success() {
                return Ok(());
            }
        }
    }

    // 尝试 MSVC 工具链 (ml64 + link)
    let vs_root = std::path::PathBuf::from(
        "C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Tools\\MSVC",
    );
    if let Ok(vs_version) = std::fs::read_dir(&vs_root) {
        let vs_version = vs_version
            .filter_map(|e| e.ok())
            .find(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false));

        if let Some(vs_ver) = vs_version {
            let bin_path = vs_ver.path().join("bin").join("Hostx64").join("x64");
            let ml64_path = bin_path.join("ml64.exe");
            let link_path = bin_path.join("link.exe");

            if ml64_path.exists() && link_path.exists() {
                // 使用 MASM 汇编
                let ml_status = std::process::Command::new(&ml64_path)
                    .arg("/c")
                    .arg("/Fo")
                    .arg(obj_path)
                    .arg(asm_path)
                    .status()
                    .map_err(|e| format!("无法运行 ml64.exe: {}", e))?;

                if !ml_status.success() {
                    return Err("MASM 汇编失败".to_string());
                }

                // 使用 link.exe 链接
                let out_arg = format!("/OUT:{}", output_path.display());
                let lib_path = vs_ver.path().join("lib").join("x64");
                let ucrt_path =
                    std::path::PathBuf::from("C:\\Program Files (x86)\\Windows Kits\\10\\Lib");

                let ucrt_version = std::fs::read_dir(&ucrt_path).ok().and_then(|entries| {
                    entries
                        .filter_map(|e| e.ok())
                        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
                        .filter(|e| e.file_name().to_string_lossy().starts_with("10."))
                        .max_by_key(|e| e.file_name().to_string_lossy().to_string())
                });

                let link_status = if let Some(ucrt_ver) = ucrt_version {
                    let ucrt_lib = ucrt_ver.path().join("ucrt").join("x64");
                    let um_lib = ucrt_ver.path().join("um").join("x64");
                    let mut cmd = std::process::Command::new(&link_path);
                    cmd.arg("/SUBSYSTEM:CONSOLE")
                        .arg("/ENTRY:main")
                        .arg(&out_arg)
                        .arg(obj_path)
                        .arg(format!("/LIBPATH:{}", lib_path.display()))
                        .arg(format!("/LIBPATH:{}", ucrt_lib.display()))
                        .arg(format!("/LIBPATH:{}", um_lib.display()))
                        .arg("ucrt.lib")
                        .arg("libvcruntime.lib")
                        .arg("legacy_stdio_definitions.lib");
                    cmd.status()
                } else {
                    std::process::Command::new(&link_path)
                        .arg("/SUBSYSTEM:CONSOLE")
                        .arg("/ENTRY:main")
                        .arg(&out_arg)
                        .arg(obj_path)
                        .status()
                };

                let link_result = link_status.map_err(|e| format!("无法运行 link.exe: {}", e))?;
                if link_result.success() {
                    return Ok(());
                }
            }
        }
    }

    Err(
        "Windows 平台汇编/链接失败：请安装 clang、MinGW (gcc/g++) 或 Visual Studio Build Tools"
            .to_string(),
    )
}

/// macOS: 使用 clang/ld (Xcode toolchain)
fn assemble_and_link_macos(
    asm_path: &std::path::Path,
    obj_path: &std::path::Path,
    output_path: &std::path::Path,
    arch: TargetArch,
) -> Result<(), String> {
    // 根据架构生成目标 triple
    let target = match arch {
        TargetArch::X86_64 => "x86_64-apple-darwin",
        TargetArch::AArch64 => "arm64-apple-darwin",
        _ => return Err(format!("macOS 尚不支持此架构: {:?}", arch)),
    };

    // 尝试使用 clang 汇编
    let clang_path = which::which("clang")
        .map_err(|_| "未找到 clang（请安装 Xcode Command Line Tools）".to_string())?;

    let assemble_out = std::process::Command::new(&clang_path)
        .arg("-c")
        .arg("-o")
        .arg(obj_path)
        .arg(asm_path)
        .arg("-target")
        .arg(target)
        .output()
        .map_err(|e| format!("无法运行 clang: {}", e))?;

    if !assemble_out.status.success() {
        let clang_stderr = String::from_utf8_lossy(&assemble_out.stderr)
            .trim()
            .to_string();
        // 尝试使用 as + ld
        let as_path = which::which("as").map_err(|_| "未找到 as（汇编器）".to_string())?;
        let ld_path = which::which("ld").map_err(|_| "未找到 ld（链接器）".to_string())?;

        // 使用 as 汇编
        let as_out = std::process::Command::new(&as_path)
            .arg("-o")
            .arg(obj_path)
            .arg(asm_path)
            .output()
            .map_err(|e| format!("无法运行 as: {}", e))?;

        if !as_out.status.success() {
            let as_stderr = String::from_utf8_lossy(&as_out.stderr).trim().to_string();
            return Err(format!(
                "macOS 汇编失败。\nclang stderr:\n{}\n\nas stderr:\n{}",
                if clang_stderr.is_empty() {
                    "(无输出)"
                } else {
                    clang_stderr.as_str()
                },
                if as_stderr.is_empty() {
                    "(无输出)"
                } else {
                    as_stderr.as_str()
                }
            ));
        }

        // 使用 ld 链接（macOS C 入口约定为 _main）
        let entry = "_main";
        let link_status = std::process::Command::new(&ld_path)
            .arg("-o")
            .arg(output_path)
            .arg(obj_path)
            .arg("-e")
            .arg(entry)
            .arg("-macosx_version_min")
            .arg("10.15")
            .status()
            .map_err(|e| format!("无法运行 ld: {}", e))?;

        if !link_status.success() {
            return Err("macOS 链接失败".to_string());
        }

        return Ok(());
    }

    // 使用 clang 链接（显式 -target，避免与汇编阶段不一致）
    let link_status = std::process::Command::new(&clang_path)
        .arg("-o")
        .arg(output_path)
        .arg(obj_path)
        .arg("-target")
        .arg(target)
        .status()
        .map_err(|e| format!("链接失败: {}", e))?;

    if !link_status.success() {
        return Err("macOS 链接失败".to_string());
    }

    Ok(())
}

/// Linux: 使用 gcc/clang
fn assemble_and_link_linux(
    asm_path: &std::path::Path,
    obj_path: &std::path::Path,
    output_path: &std::path::Path,
    arch: TargetArch,
) -> Result<(), String> {
    let target = match arch {
        TargetArch::X86_64 => "x86_64-linux-gnu",
        TargetArch::AArch64 => "aarch64-linux-gnu",
        _ => {
            return Err(format!("Linux 原生汇编尚不支持此架构: {:?}", arch));
        }
    };

    // 优先使用 clang
    let clang_path = which::which("clang").ok();
    let gcc_path = which::which("gcc").ok();

    let (assembler, linker, use_clang_target) = if let Some(clang) = clang_path {
        (clang.clone(), clang, true)
    } else if let Some(gcc) = gcc_path {
        (gcc.clone(), gcc, false)
    } else {
        return Err("未找到 clang 或 gcc".to_string());
    };

    // 汇编（`-target` 为 clang 驱动选项；纯 gcc 依赖默认主机 triple）
    let mut assemble_cmd = std::process::Command::new(&assembler);
    assemble_cmd.arg("-c").arg("-o").arg(obj_path).arg(asm_path);
    if use_clang_target {
        assemble_cmd.arg("-target").arg(target);
    }
    let assemble_status = assemble_cmd
        .status()
        .map_err(|e| format!("汇编失败: {}", e))?;

    if !assemble_status.success() {
        return Err("Linux 汇编失败".to_string());
    }

    // 链接
    let mut link_cmd = std::process::Command::new(&linker);
    link_cmd.arg("-o").arg(output_path).arg(obj_path);
    if use_clang_target {
        link_cmd.arg("-target").arg(target);
    }
    let link_status = link_cmd.status().map_err(|e| format!("链接失败: {}", e))?;

    if !link_status.success() {
        return Err("Linux 链接失败".to_string());
    }

    Ok(())
}
