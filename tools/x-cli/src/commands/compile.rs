use crate::pipeline;
use crate::utils;
use x_codegen::Target;
use x_codegen::{CodeGenConfig, CodeGenerator};
use x_codegen_zig::{ZigBackend, ZigBackendConfig, ZigTarget};
use x_codegen_typescript::{TypeScriptBackend, TypeScriptBackendConfig};
use x_codegen_csharp::{CSharpBackend, CSharpConfig};
use x_codegen_rust::{RustBackend, RustBackendConfig};
use x_codegen_native::{NativeBackend, NativeBackendConfig};

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
        None | Some("native") => Target::Native,
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
        // ── Native 后端 - 直接生成机器码，无需外部编译器 ────────────────────
        Target::Native => {
            // Native 后端直接生成机器码
            use x_codegen_native::{TargetArch, TargetOS};
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
                format: x_codegen_native::OutputFormat::Executable,
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
            let obj_path = temp_dir.join("x_native_output.obj");

            // 写入汇编文件
            std::fs::write(&asm_path, asm_code.as_bytes())
                .map_err(|e| format!("无法写入汇编文件: {}", e))?;

            // 输出路径 - 避免双重扩展名
            let output_path = if os == TargetOS::Windows {
                let path = std::path::PathBuf::from(out_path);
                // 检查是否已经有扩展名
                if path.extension().map_or(false, |e| e == "exe") {
                    path
                } else {
                    path.with_extension("exe")
                }
            } else {
                std::path::PathBuf::from(out_path)
            };

            // 查找 VS Build Tools 路径
            let vs_root = std::path::PathBuf::from("C:\\Program Files (x86)\\Microsoft Visual Studio\\2022\\BuildTools\\VC\\Tools\\MSVC");
            let vs_version = std::fs::read_dir(&vs_root)
                .map_err(|e| format!("无法找到 VS Build Tools: {}", e))?
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
                .next()
                .ok_or("无法找到 VS 版本目录")?;
            let bin_path = vs_version.path().join("bin").join("Hostx64").join("x64");

            // 优先使用 Windows 自带的 ml64 和 link（MASM），NASM 作为备选
            let ml64_path = bin_path.join("ml64.exe");
            let link_path = bin_path.join("link.exe");

            // 检查 ml64 是否存在
            let use_masm = ml64_path.exists();

            if use_masm {
                // 使用 MASM + link.exe（Windows 自带工具）
                // MASM 汇编
                let ml_status = std::process::Command::new(&ml64_path)
                    .arg("/c")
                    .arg("/Fo")
                    .arg(&obj_path)
                    .arg(&asm_path)
                    .status()
                    .map_err(|e| format!("无法运行 ml64.exe: {}", e))?;

                if !ml_status.success() {
                    return Err("MASM 汇编失败".to_string());
                }

                // 链接 - 链接 C 运行时库
                let out_arg = format!("/OUT:{}", output_path.display());
                let lib_path = vs_version.path().join("lib").join("x64");
                let ucrt_path = std::path::PathBuf::from("C:\\Program Files (x86)\\Windows Kits\\10\\Lib");

                // 查找最新的 UCRT 版本
                let ucrt_version = std::fs::read_dir(&ucrt_path)
                    .ok()
                    .and_then(|entries| {
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
                        .arg(&obj_path)
                        .arg(format!("/LIBPATH:{}", lib_path.display()))
                        .arg(format!("/LIBPATH:{}", ucrt_lib.display()))
                        .arg(format!("/LIBPATH:{}", um_lib.display()))
                        .arg("ucrt.lib")      // Universal C Runtime
                        .arg("libvcruntime.lib")  // Visual C++ Runtime
                        .arg("legacy_stdio_definitions.lib");  // Legacy stdio symbols
                    cmd.status()
                } else {
                    // Fallback without UCRT path
                    std::process::Command::new(&link_path)
                        .arg("/SUBSYSTEM:CONSOLE")
                        .arg("/ENTRY:main")
                        .arg(&out_arg)
                        .arg(&obj_path)
                        .status()
                };
                let link_status = link_status.map_err(|e| format!("无法运行 link.exe: {}", e))?;

                if !link_status.success() {
                    return Err("链接失败".to_string());
                }
            } else {
                // 备选：使用 NASM + link.exe
                let nasm_path = if std::path::Path::new("C:\\tools\\nasm-2.16.03\\nasm.exe").exists() {
                    Some(std::path::PathBuf::from("C:\\tools\\nasm-2.16.03\\nasm.exe"))
                } else {
                    which::which("nasm").ok()
                };

                if let Some(nasm) = nasm_path {
                    let nasm_status = std::process::Command::new(&nasm)
                        .arg("-f")
                        .arg("win64")
                        .arg("-o")
                        .arg(&obj_path)
                        .arg(&asm_path)
                        .status()
                        .map_err(|e| format!("无法运行 NASM: {}", e))?;

                    if !nasm_status.success() {
                        return Err("NASM 汇编失败".to_string());
                    }

                    // 使用 link.exe 链接 - 链接 C 运行时库
                    let out_arg = format!("/OUT:{}", output_path.display());
                    let lib_path = vs_version.path().join("lib").join("x64");
                    let ucrt_path = std::path::PathBuf::from("C:\\Program Files (x86)\\Windows Kits\\10\\Lib");

                    // 查找最新的 UCRT 版本
                    let ucrt_version = std::fs::read_dir(&ucrt_path)
                        .ok()
                        .and_then(|entries| {
                            entries
                                .filter_map(|e| e.ok())
                                .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
                                .filter(|e| e.file_name().to_string_lossy().starts_with("10."))
                                .max_by_key(|e| e.file_name().to_string_lossy().to_string())
                        });

                    let link_status = if let Some(ucrt_ver) = ucrt_version {
                        let ucrt_lib = ucrt_ver.path().join("ucrt").join("x64");
                        let mut cmd = std::process::Command::new(&link_path);
                        cmd.arg("/SUBSYSTEM:CONSOLE")
                            .arg(&out_arg)
                            .arg(&obj_path)
                            .arg(format!("/LIBPATH:{}", lib_path.display()))
                            .arg(format!("/LIBPATH:{}", ucrt_lib.display()))
                            .arg("ucrt.lib")
                            .arg("libvcruntime.lib");
                        cmd.status()
                    } else {
                        std::process::Command::new(&link_path)
                            .arg("/SUBSYSTEM:CONSOLE")
                            .arg(&out_arg)
                            .arg(&obj_path)
                            .status()
                    }
                    .map_err(|e| format!("无法运行 link.exe: {}", e))?;

                    if !link_status.success() {
                        return Err("链接失败".to_string());
                    }
                } else {
                    return Err("未找到 ml64.exe 或 NASM，请安装 Visual Studio Build Tools 或 NASM".to_string());
                }
            }

            // 设置可执行权限（非 Windows）
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&output_path)?.permissions();
                perms.set_mode(perms.mode() | 0o755);
                std::fs::set_permissions(&output_path, perms)?;
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
            let mut backend =
                ZigBackend::new(ZigBackendConfig {
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
            let parser = x_parser::parser::XParser::new();
            let program = parser
                .parse(content)
                .map_err(|e| pipeline::format_parse_error(file, content, &e))?;
            let mut backend = CSharpBackend::new(
                CSharpConfig::default(),
            );
            let output = backend
                .generate_from_ast(&program)
                .map_err(|e| format!("C#代码生成失败: {}", e))?;
            let csharp_code = String::from_utf8_lossy(&output.files[0].content);
            println!("{}", csharp_code);
            Ok(())
        }
        "rust" => {
            let parser = x_parser::parser::XParser::new();
            let program = parser
                .parse(content)
                .map_err(|e| pipeline::format_parse_error(file, content, &e))?;
            let mut backend = RustBackend::new(
                RustBackendConfig::default(),
            );
            let output = backend
                .generate_from_ast(&program)
                .map_err(|e| format!("Rust代码生成失败: {}", e))?;
            let rust_code = String::from_utf8_lossy(&output.files[0].content);
            println!("{}", rust_code);
            Ok(())
        }
        "c" => {
            Err("C 后端尚未实现".to_string())
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
            use x_codegen_native::{NativeBackend, NativeBackendConfig, TargetArch, TargetOS};
            let arch = if cfg!(target_arch = "x86_64") {
                TargetArch::X86_64
            } else {
                return Err("Only x86_64 is currently supported for native emit".to_string());
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
                format: x_codegen_native::OutputFormat::Assembly,
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
