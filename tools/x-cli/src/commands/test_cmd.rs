use crate::pipeline;
use crate::project::Project;
use crate::utils;
use colored::*;
use std::path::PathBuf;
use std::time::Instant;

#[allow(unused_variables)]
pub fn exec(
    filter: Option<&str>,
    release: bool,
    lib: bool,
    doc: bool,
    no_run: bool,
    jobs: Option<u32>,
) -> Result<(), String> {
    if filter == Some("integration") {
        return run_integration_tests(false);
    }

    let project = Project::find()?;
    let start = Instant::now();

    utils::status(
        "Testing",
        &format!(
            "{} v{} ({})",
            project.name(),
            project.version(),
            project.root.display()
        ),
    );

    let mut test_files = Vec::new();
    test_files.extend(project.test_files());

    if lib {
        test_files.extend(project.source_files());
    }

    if test_files.is_empty() {
        utils::note("未找到测试文件");
        utils::note("在 tests/ 目录下创建 .x 文件来添加测试");
        return Ok(());
    }

    if let Some(pattern) = filter {
        test_files.retain(|p| p.to_str().is_some_and(|s| s.contains(pattern)));
    }

    let mut passed = 0;
    let mut failed = 0;
    let mut errors = Vec::new();

    for path in &test_files {
        let name = path
            .strip_prefix(&project.root)
            .unwrap_or(path)
            .display()
            .to_string();

        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                errors.push(format!("{}: 无法读取: {}", name, e));
                failed += 1;
                continue;
            }
        };

        let parser = x_parser::parser::XParser::new();
        match parser.parse(&content) {
            Ok(program) => {
                if let Err(e) = x_typechecker::type_check(&program) {
                    errors.push(format!("{}: 类型检查失败: {}", name, e));
                    failed += 1;
                    continue;
                }

                if !no_run {
                    let mut interpreter = x_interpreter::Interpreter::new();
                    match interpreter.run(&program) {
                        Ok(()) => {
                            println!("test {} ... {}", name, "ok".green());
                            passed += 1;
                        }
                        Err(e) => {
                            println!("test {} ... {}", name, "FAILED".red());
                            errors.push(format!("{}: {}", name, e));
                            failed += 1;
                        }
                    }
                } else {
                    println!("test {} ... {}", name, "ok (no run)".yellow());
                    passed += 1;
                }
            }
            Err(e) => {
                println!("test {} ... {}", name, "FAILED".red());
                errors.push(pipeline::format_parse_error(
                    &path.display().to_string(),
                    &content,
                    &e,
                ));
                failed += 1;
            }
        }
    }

    let elapsed = start.elapsed();
    println!();

    if !errors.is_empty() {
        println!("failures:");
        for err in &errors {
            println!("  {}", err);
        }
        println!();
    }

    let result_str = if failed > 0 {
        format!("{}", "FAILED".red().bold())
    } else {
        format!("{}", "ok".green().bold())
    };

    println!(
        "test result: {}. {} passed; {} failed; finished in {}",
        result_str,
        passed,
        failed,
        utils::elapsed_str(elapsed)
    );

    if failed > 0 {
        Err(format!("{} 个测试失败", failed))
    } else {
        Ok(())
    }
}

pub fn run_integration_tests(verbose: bool) -> Result<(), String> {
    use std::process::Command;

    let integration_dir = PathBuf::from("tests/integration");
    if !integration_dir.exists() {
        utils::note("integration tests directory not found: tests/integration");
        utils::note("create tests in: tests/integration/<category>/<test>.x");
        return Ok(());
    }

    let x_cli_path = find_x_cli()?;

    println!("\n{}", "=".repeat(60));
    println!("X Language Integration Test Suite");
    println!("{}", "=".repeat(60));

    let mut passed = 0usize;
    let mut failed = 0usize;
    let skipped = 0usize;
    let mut total = 0usize;

    let categories = ["basic", "types", "functions", "patterns", "stdlib"];

    for category in &categories {
        let category_dir = integration_dir.join(category);
        if !category_dir.exists() {
            continue;
        }

        println!("\n{} tests:", category);

        for entry in walkdir::WalkDir::new(&category_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "x"))
        {
            let path = entry.path();
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown");

            total += 1;

            if verbose {
                print!("  {}::{} ... ", category, name);
            } else {
                print!("  {} ... ", name);
            }

            let result = Command::new(&x_cli_path).arg("run").arg(path).output();

            match result {
                Ok(output) => {
                    let exit_code = output.status.code().unwrap_or(-1);
                    if exit_code == 0 {
                        println!("{}", "ok".green());
                        passed += 1;
                    } else {
                        println!("{}", "FAILED".red());
                        if verbose {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            for line in stderr.lines().take(5) {
                                println!("    {}", line);
                            }
                        }
                        failed += 1;
                    }
                }
                Err(e) => {
                    println!("{}", "ERROR".red());
                    if verbose {
                        println!("    {}", e);
                    }
                    failed += 1;
                }
            }
        }
    }

    println!("\n{}", "=".repeat(60));
    print!("test result: ");
    if failed > 0 {
        print!("{}", "FAILED".red().bold());
    } else {
        print!("{}", "ok".green().bold());
    }
    println!(
        ". {} passed; {} failed; {} skipped; {} total",
        passed, failed, skipped, total
    );
    println!("{}", "=".repeat(60));

    if failed > 0 {
        Err(format!("{} integration tests failed", failed))
    } else {
        Ok(())
    }
}

fn find_x_cli() -> Result<PathBuf, String> {
    let candidates = [
        PathBuf::from("tools/target/release/x.exe"),
        PathBuf::from("tools/target/debug/x.exe"),
        PathBuf::from("target/release/x.exe"),
        PathBuf::from("target/debug/x.exe"),
        PathBuf::from("x.exe"),
        PathBuf::from("x"),
    ];

    for candidate in candidates {
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    if let Ok(path) = which::which("x") {
        return Ok(path);
    }

    Err("Could not find x-cli. Build it first: cd tools/x-cli && cargo build --release".to_string())
}
