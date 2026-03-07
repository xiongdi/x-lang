use x_parser::parser::XParser;
use x_interpreter::Interpreter;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 || args[1] != "run" {
        println!("Usage: simple_x_cli run <file.x>");
        return;
    }
    
    let file = &args[2];
    let content = match std::fs::read_to_string(file) {
        Ok(content) => content,
        Err(e) => {
            println!("无法读取文件 {}: {}", file, e);
            return;
        }
    };
    
    let parser = XParser::new();
    let program = match parser.parse(&content) {
        Ok(program) => program,
        Err(e) => {
            println!("解析错误: {}", e);
            return;
        }
    };
    
    let mut interpreter = Interpreter::new();
    match interpreter.run(&program) {
        Ok(_) => println!("运行成功"),
        Err(e) => println!("运行失败: {}", e),
    }
}
