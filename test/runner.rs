use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, PartialEq)]
enum TestResult {
    Pass,
    Fail(String),
}

#[derive(Debug)]
struct TestCase {
    name: String,
    spec: Vec<String>,
    source_file: PathBuf,
    expect: TestExpectation,
}

#[derive(Debug)]
struct TestExpectation {
    parse: String,
    run: String,
    stdout: Option<String>,
}

#[derive(Debug)]
struct TestSection {
    id: String,
    name: String,
    spec_chapter: String,
    cases: Vec<TestCase>,
}

fn main() {
    let test_dir = Path::new("test");
    let mut sections = Vec::new();

    // 遍历测试目录
    for entry in fs::read_dir(test_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        if path.is_dir() && path.file_name().unwrap() != "files" && path.file_name().unwrap() != ".git" {
            let toml_path = path.join("basic.toml");
            if toml_path.exists() {
                if let Ok(section) = parse_test_section(&toml_path) {
                    sections.push(section);
                }
            }
        }
    }

    // 运行测试
    let results = run_tests(&sections);

    // 生成测试报告
    generate_report(&results);
}

fn parse_test_section(path: &Path) -> Result<TestSection, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    
    // 简单的TOML解析
    let mut section_id = String::new();
    let mut section_name = String::new();
    let mut spec_chapter = String::new();
    let mut cases = Vec::new();
    
    let mut current_case = None;
    let mut current_expect: Option<TestExpectation> = None;
    let mut in_section = false;
    let mut in_case = false;
    let mut in_expect = false;
    
    for line in content.lines() {
        let line = line.trim();
        
        if line.starts_with("[section]") {
            in_section = true;
            in_case = false;
            in_expect = false;
        } else if line.starts_with("[[case]]") {
            // 保存之前的case
            if let Some(case) = current_case {
                cases.push(case);
            }
            
            current_case = Some(TestCase {
                name: String::new(),
                spec: Vec::new(),
                source_file: PathBuf::new(),
                expect: TestExpectation {
                    parse: String::new(),
                    run: String::new(),
                    stdout: None,
                },
            });
            in_section = false;
            in_case = true;
            in_expect = false;
        } else if line.starts_with("[case.expect]") {
            in_expect = true;
        } else if in_section {
            if line.starts_with("id = ") {
                section_id = line.split_once('"').unwrap().1.split_once('"').unwrap().0.to_string();
            } else if line.starts_with("name = ") {
                section_name = line.split_once('"').unwrap().1.split_once('"').unwrap().0.to_string();
            } else if line.starts_with("spec_chapter = ") {
                spec_chapter = line.split_once('"').unwrap().1.split_once('"').unwrap().0.to_string();
            }
        } else if in_case && current_case.is_some() {
            let case = current_case.as_mut().unwrap();
            
            if line.starts_with("name = ") {
                case.name = line.split_once('"').unwrap().1.split_once('"').unwrap().0.to_string();
            } else if line.starts_with("spec = ") {
                // 简单处理数组
                let spec_str = line.split_once('[').unwrap().1.split_once(']').unwrap().0;
                case.spec = spec_str.split(',').map(|s| s.trim().trim_matches('"').to_string()).collect();
            } else if line.starts_with("source_file = ") {
                let source_path = line.split_once('"').unwrap().1.split_once('"').unwrap().0;
                case.source_file = PathBuf::from(source_path);
            }
        } else if in_expect && current_case.is_some() {
            let case = current_case.as_mut().unwrap();
            
            if line.starts_with("parse = ") {
                case.expect.parse = line.split_once('"').unwrap().1.split_once('"').unwrap().0.to_string();
            } else if line.starts_with("run = ") {
                case.expect.run = line.split_once('"').unwrap().1.split_once('"').unwrap().0.to_string();
            } else if line.starts_with("stdout = ") {
                let stdout = line.split_once('"').unwrap().1.split_once('"').unwrap().0.to_string();
                case.expect.stdout = Some(stdout);
            }
        }
    }
    
    // 保存最后一个case
    if let Some(case) = current_case {
        cases.push(case);
    }
    
    Ok(TestSection {
        id: section_id,
        name: section_name,
        spec_chapter: spec_chapter,
        cases: cases,
    })
}

fn run_tests(sections: &[TestSection]) -> HashMap<String, HashMap<String, TestResult>> {
    let mut results = HashMap::new();

    for section in sections {
        let mut section_results = HashMap::new();
        
        for case in &section.cases {
            let result = run_test_case(case);
            section_results.insert(case.name.clone(), result);
        }
        
        results.insert(section.id.clone(), section_results);
    }

    results
}

fn run_test_case(case: &TestCase) -> TestResult {
    // 检查源文件是否存在
    if !case.source_file.exists() {
        return TestResult::Fail(format!("Source file not found: {}", case.source_file.display()));
    }

    // 运行x-cli命令执行测试
    let output = Command::new("cargo")
        .arg("run")
        .arg("--package")
        .arg("x-cli")
        .arg("run")
        .arg(&case.source_file)
        .current_dir("c:\\Users\\Administrator\\Documents\\x-lang\\tools")
        .output();

    match output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();

            // 检查是否通过
            if output.status.success() {
                // 检查stdout是否符合预期
                if let Some(expected_stdout) = &case.expect.stdout {
                    if stdout == *expected_stdout {
                        TestResult::Pass
                    } else {
                        TestResult::Fail(format!("Stdout mismatch: expected '{}', got '{}'", expected_stdout, stdout))
                    }
                } else {
                    TestResult::Pass
                }
            } else {
                TestResult::Fail(format!("Command failed with stderr: {}", stderr))
            }
        }
        Err(e) => TestResult::Fail(format!("Failed to execute command: {}", e)),
    }
}

fn generate_report(results: &HashMap<String, HashMap<String, TestResult>>) {
    println!("Test Report");
    println!("============");
    println!();

    let mut total_tests = 0;
    let mut passed_tests = 0;

    for (section_id, section_results) in results {
        println!("Section: {}", section_id);
        println!("{}", "-".repeat(50));

        for (case_name, result) in section_results {
            total_tests += 1;
            match result {
                TestResult::Pass => {
                    println!("✓ {}", case_name);
                    passed_tests += 1;
                }
                TestResult::Fail(msg) => {
                    println!("✗ {}: {}", case_name, msg);
                }
            }
        }

        println!();
    }

    println!("Summary");
    println!("{}", "=".repeat(50));
    println!("Total tests: {}", total_tests);
    println!("Passed tests: {}", passed_tests);
    println!("Failed tests: {}", total_tests - passed_tests);
    println!("Pass rate: {:.2}%", (passed_tests as f64 / total_tests as f64) * 100.0);
}
