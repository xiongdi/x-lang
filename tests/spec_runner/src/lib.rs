use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SpecTestError {
    #[error("Failed to read test file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("Test '{name}' failed: {message}")]
    TestFailed { name: String, message: String },

    #[error("Compilation failed but was expected to succeed: {stderr}")]
    UnexpectedCompileFail { stderr: String },

    #[error("Expected compilation failure but it succeeded")]
    ExpectedCompileFail,

    #[error("Exit code mismatch: expected {expected}, got {actual}")]
    ExitCodeMismatch { expected: i32, actual: i32 },

    #[error("Stdout mismatch:\nExpected: {expected}\nActual: {actual}")]
    StdoutMismatch { expected: String, actual: String },

    #[error("Stderr mismatch:\nExpected: {expected}\nActual: {actual}")]
    StderrMismatch { expected: String, actual: String },

    #[error("Error message does not contain '{expected}'\nActual: {actual}")]
    ErrorMessageMismatch { expected: String, actual: String },
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpecTest {
    pub name: String,
    pub source: String,
    #[serde(default)]
    pub exit_code: i32,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub compile_fail: bool,
    #[serde(default)]
    pub error_contains: String,
    #[serde(default)]
    pub spec: Vec<String>,
    #[serde(default = "default_category")]
    pub category: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub target: String,
    #[serde(default)]
    pub skip: bool,
    #[serde(default)]
    pub skip_reason: String,
}

fn default_category() -> String {
    "general".to_string()
}

#[derive(Debug, Deserialize)]
pub struct SpecTestFile {
    pub tests: Option<Vec<SpecTest>>,
}

impl SpecTest {
    pub fn from_file(path: &Path) -> Result<Vec<SpecTest>, SpecTestError> {
        let content = fs::read_to_string(path)?;
        let trimmed = content.trim();

        if trimmed.starts_with("name =") || trimmed.starts_with("name=") {
            let test: SpecTest = toml::from_str(&content)?;
            Ok(vec![test])
        } else if trimmed.starts_with("[[tests]]") {
            let file: SpecTestFile = toml::from_str(&content)?;
            Ok(file.tests.unwrap_or_default())
        } else {
            let test: SpecTest = toml::from_str(&content)?;
            Ok(vec![test])
        }
    }

    pub fn run(&self, x_cli_path: &Path, temp_dir: &Path) -> Result<(), SpecTestError> {
        if self.skip {
            println!("  ⏭️  Skipping: {}", self.skip_reason);
            return Ok(());
        }

        let source_file = temp_dir.join(format!("{}.x", self.name));
        fs::write(&source_file, &self.source)?;

        let output = Command::new(x_cli_path)
            .arg("run")
            .arg(&source_file)
            .output()
            .map_err(|e| SpecTestError::TestFailed {
                name: self.name.clone(),
                message: format!("Failed to execute x-cli: {}", e),
            })?;

        self.check_output(output)
    }

    fn check_output(&self, output: Output) -> Result<(), SpecTestError> {
        let exit_code = output.status.code().unwrap_or(-1);
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if self.compile_fail {
            if exit_code == 0 {
                return Err(SpecTestError::ExpectedCompileFail);
            }

            if !self.error_contains.is_empty() {
                if !stderr.contains(&self.error_contains) {
                    return Err(SpecTestError::ErrorMessageMismatch {
                        expected: self.error_contains.clone(),
                        actual: stderr,
                    });
                }
            }

            return Ok(());
        }

        if exit_code != self.exit_code {
            return Err(SpecTestError::ExitCodeMismatch {
                expected: self.exit_code,
                actual: exit_code,
            });
        }

        if !self.stdout.is_empty() && stdout.trim() != self.stdout.trim() {
            return Err(SpecTestError::StdoutMismatch {
                expected: self.stdout.clone(),
                actual: stdout,
            });
        }

        if !self.stderr.is_empty() && stderr.trim() != self.stderr.trim() {
            return Err(SpecTestError::StderrMismatch {
                expected: self.stderr.clone(),
                actual: stderr,
            });
        }

        Ok(())
    }
}

pub struct SpecTestRunner {
    x_cli_path: PathBuf,
    temp_dir: PathBuf,
}

use std::path::PathBuf;

impl SpecTestRunner {
    pub fn new(x_cli_path: PathBuf) -> Result<Self, SpecTestError> {
        let temp_dir = std::env::temp_dir().join("x-lang-spec-tests");
        fs::create_dir_all(&temp_dir)?;

        Ok(Self {
            x_cli_path,
            temp_dir,
        })
    }

    pub fn run_directory(&self, dir: &Path) -> Result<TestSummary, SpecTestError> {
        let mut summary = TestSummary::default();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().map_or(false, |ext| ext == "toml") {
                match self.run_test_file(&path) {
                    Ok(file_summary) => {
                        summary.passed += file_summary.passed;
                        summary.failed += file_summary.failed;
                        summary.skipped += file_summary.skipped;
                    }
                    Err(e) => {
                        eprintln!("Error running test file {:?}: {}", path, e);
                        summary.failed += 1;
                    }
                }
            }
        }

        Ok(summary)
    }

    fn run_test_file(&self, path: &Path) -> Result<TestSummary, SpecTestError> {
        let tests = SpecTest::from_file(path)?;
        let mut summary = TestSummary::default();

        println!("\n📄 Running tests from: {}", path.display());

        for test in tests {
            print!("  ▶️  {} ... ", test.name);

            match test.run(&self.x_cli_path, &self.temp_dir) {
                Ok(()) => {
                    println!("✅");
                    summary.passed += 1;
                }
                Err(SpecTestError::TestFailed { message, .. }) if message.contains("Skipping") => {
                    println!("⏭️  Skipped");
                    summary.skipped += 1;
                }
                Err(e) => {
                    println!("❌");
                    eprintln!("    Error: {}", e);
                    summary.failed += 1;
                }
            }
        }

        Ok(summary)
    }
}

#[derive(Debug, Default)]
pub struct TestSummary {
    pub passed: u64,
    pub failed: u64,
    pub skipped: u64,
}

impl TestSummary {
    pub fn total(&self) -> u64 {
        self.passed + self.failed + self.skipped
    }

    pub fn success_rate(&self) -> f64 {
        if self.total() == 0 {
            0.0
        } else {
            (self.passed as f64 / self.total() as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_test() {
        let content = r#"
name = "simple_test"
source = "println(42)"
stdout = "42\n"
"#;
        let test: SpecTest = toml::from_str(content).unwrap();
        assert_eq!(test.name, "simple_test");
        assert_eq!(test.source, "println(42)");
        assert_eq!(test.stdout, "42\n");
        assert_eq!(test.exit_code, 0);
        assert!(!test.compile_fail);
    }

    #[test]
    fn test_parse_compile_fail_test() {
        let content = r#"
name = "type_error"
source = "let x: integer = 3.14"
compile_fail = true
error_contains = "type mismatch"
"#;
        let test: SpecTest = toml::from_str(content).unwrap();
        assert_eq!(test.name, "type_error");
        assert!(test.compile_fail);
        assert_eq!(test.error_contains, "type mismatch");
    }

    #[test]
    fn test_parse_multiple_tests() {
        let content = r#"
[[tests]]
name = "test1"
source = "println(1)"

[[tests]]
name = "test2"
source = "println(2)"
"#;
        let file: SpecTestFile = toml::from_str(content).unwrap();
        let tests = file.tests.unwrap();
        assert_eq!(tests.len(), 2);
        assert_eq!(tests[0].name, "test1");
        assert_eq!(tests[1].name, "test2");
    }
}
