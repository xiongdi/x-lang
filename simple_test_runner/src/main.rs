use x_parser::parser::XParser;
use x_interpreter::Interpreter;
use std::fs;
use std::path::Path;

fn main() {
    let test_dir = "../test";
    let test_files = find_test_files(test_dir);
    
    println!("找到 {} 个测试文件", test_files.len());
    
    let mut passed = 0;
    let mut failed = 0;
    
    for test_file in test_files {
        println!("运行测试: {}", test_file.display());
        match run_test(&test_file) {
            Ok(_) => {
                println!("✓ 测试通过");
                passed += 1;
            }
            Err(e) => {
                println!("✗ 测试失败: {}", e);
                failed += 1;
            }
        }
        println!();
    }
    
    println!("测试结果: {} 通过, {} 失败", passed, failed);
}

fn find_test_files(dir: &str) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    let paths = fs::read_dir(dir).expect("无法读取测试目录");
    
    for path in paths {
        let path = path.expect("无法读取路径").path();
        if path.is_dir() {
            files.extend(find_test_files(path.to_str().unwrap()));
        } else if path.extension().map_or(false, |ext| ext == "x") {
            files.push(path);
        }
    }
    
    files
}

fn run_test(file: &Path) -> Result<(), String> {
    let content = fs::read_to_string(file).map_err(|e| format!("无法读取文件: {}", e))?;
    
    let parser = XParser::new();
    let program = parser.parse(&content).map_err(|e| format!("解析错误: {}", e))?;
    
    let mut interpreter = Interpreter::new();
    interpreter.run(&program).map_err(|e| format!("运行错误: {}", e))?;
    
    Ok(())
}
