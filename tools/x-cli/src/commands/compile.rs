use crate::pipeline;
use crate::utils;
use x_codegen::c_backend::{CBackend, CBackendConfig, CStandard};
use x_codegen::typescript_backend::{TypeScriptBackend, TypeScriptBackendConfig};
use x_codegen::zig_backend::ZigTarget;
use x_codegen::Target;
use x_codegen::{get_code_generator, CodeGenConfig, CodeGenerator};

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
        Some("c") => Target::C,
        Some("wasm" | "wasm32-wasi") => Target::Wasm,
        Some("ts" | "typescript") => Target::TypeScript,
        Some(t) => {
            if let Some(t) = Target::from_str(t) {
                t
            } else {
                return Err(format!(
                    "未知目标平台: {}（支持: native, wasm, wasm32-wasi, wasm32-freestanding, c, ts/typescript）",
                    t
                ));
            }
        }
    };

    // Run the full compiler pipeline: source → AST → HIR → MIR → LIR
    let pipeline_output = pipeline::run_pipeline(&content)?;

    // All backends use the full pipeline via LIR
    match parsed_target {
        // ── Zig-based targets (Native + Wasm) ────────────────────────────────
        Target::Native | Target::Wasm => {
            let zig_target = match target {
                None | Some("native") => ZigTarget::Native,
                Some("wasm" | "wasm32-wasi") => ZigTarget::Wasm32Wasi,
                Some("wasm32-freestanding") => ZigTarget::Wasm32Freestanding,
                _ => {
                    if parsed_target == Target::Wasm {
                        ZigTarget::Wasm32Wasi
                    } else {
                        ZigTarget::Native
                    }
                }
            };
            let mut backend =
                x_codegen::zig_backend::ZigBackend::new(x_codegen::zig_backend::ZigBackendConfig {
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

        // ── C target ─────────────────────────────────────────────────────────
        Target::C => {
            let codegen_config = CodeGenConfig {
                target: Target::C,
                output_dir: None,
                optimize: release,
                debug_info: !release,
            };
            let mut generator = get_code_generator(Target::C, codegen_config)
                .map_err(|e| format!("获取代码生成器失败: {}", e))?;
            let codegen_output = generator
                .generate_from_lir(&pipeline_output.lir)
                .map_err(|e| format!("C代码生成失败: {}", e))?;

            let c_code = String::from_utf8_lossy(&codegen_output.files[0].content);
            let output_path = std::path::PathBuf::from(out_path);

            if no_link {
                let c_out_path = format!("{}.c", out_path);
                std::fs::write(&c_out_path, c_code.as_bytes())
                    .map_err(|e| format!("无法写入C文件 {}: {}", c_out_path, e))?;
                println!("已生成C代码: {}", c_out_path);
                return Ok(());
            }

            let mut backend = CBackend::new(CBackendConfig {
                output_dir: None,
                optimize: release,
                debug_info: !release,
                c_standard: CStandard::C23,
                generate_header: false,
            });
            backend
                .compile_c_code(&c_code, &output_path, CStandard::C23, release, !release)
                .map_err(|e| format!("C编译失败: {}", e))?;

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
            let mut program = parser
                .parse(content)
                .map_err(|e| pipeline::format_parse_error(file, content, &e))?;
            let prelude_decls = crate::pipeline::parse_std_prelude()?;
            let mut new_decls = prelude_decls;
            new_decls.extend(program.declarations);
            program.declarations = new_decls;
            println!("{:#?}", program);
            Ok(())
        }
        // ── Backend source-emit options ──────────────────────────────────────
        "zig" => {
            let parser = x_parser::parser::XParser::new();
            let mut program = parser
                .parse(content)
                .map_err(|e| pipeline::format_parse_error(file, content, &e))?;
            let prelude_decls = crate::pipeline::parse_std_prelude()?;
            let mut new_decls = prelude_decls;
            new_decls.extend(program.declarations);
            program.declarations = new_decls;
            let mut backend = x_codegen::zig_backend::ZigBackend::new(
                x_codegen::zig_backend::ZigBackendConfig::default(),
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
            let mut program = parser
                .parse(content)
                .map_err(|e| pipeline::format_parse_error(file, content, &e))?;
            let prelude_decls = crate::pipeline::parse_std_prelude()?;
            let mut new_decls = prelude_decls;
            new_decls.extend(program.declarations);
            program.declarations = new_decls;
            let mut backend = x_codegen::csharp_backend::CSharpBackend::new(
                x_codegen::csharp_backend::CSharpBackendConfig::default(),
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
            let mut program = parser
                .parse(content)
                .map_err(|e| pipeline::format_parse_error(file, content, &e))?;
            let prelude_decls = crate::pipeline::parse_std_prelude()?;
            let mut new_decls = prelude_decls;
            new_decls.extend(program.declarations);
            program.declarations = new_decls;
            let mut backend = x_codegen::rust_backend::RustBackend::new(
                x_codegen::rust_backend::RustBackendConfig::default(),
            );
            let output = backend
                .generate_from_ast(&program)
                .map_err(|e| format!("Rust代码生成失败: {}", e))?;
            let rust_code = String::from_utf8_lossy(&output.files[0].content);
            println!("{}", rust_code);
            Ok(())
        }
        "c" => {
            let parser = x_parser::parser::XParser::new();
            let mut program = parser
                .parse(content)
                .map_err(|e| pipeline::format_parse_error(file, content, &e))?;
            let prelude_decls = crate::pipeline::parse_std_prelude()?;
            let mut new_decls = prelude_decls;
            new_decls.extend(program.declarations);
            program.declarations = new_decls;
            pipeline::type_check_with_big_stack(&program)?;
            let mut backend =
                x_codegen::c_backend::CBackend::new(x_codegen::c_backend::CBackendConfig::default());
            let output = backend
                .generate_from_ast(&program)
                .map_err(|e| format!("C代码生成失败: {}", e))?;
            let c_code = String::from_utf8_lossy(&output.files[0].content);
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
        _ => Err(format!(
            "未知 --emit 阶段: {}\n支持的选项: tokens, ast, hir, mir, lir, zig, ts, js, typescript, javascript, c, rust, dotnet, csharp",
            stage
        )),
    }
}
