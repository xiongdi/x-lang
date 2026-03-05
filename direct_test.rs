
// 直接测试：解析并运行 hello.x，不使用 x-cli
extern crate x_parser;
extern crate x_interpreter;

use std::fs;

fn main() {
    println!("直接运行 examples/hello.x");

    // 读取文件
    let content = fs::read_to_string("examples/hello.x").unwrap();
    println!("\n读取文件内容：");
    println!("{}", content);

    // 解析
    println!("\n正在解析...");
    let parser = x_parser::parser::XParser::new();
    let program = parser.parse(&content).unwrap();
    println!("解析成功！");

    // 类型检查
    println!("\n正在进行类型检查...");
    x_typechecker::type_check(&program).unwrap();
    println!("类型检查成功！");

    // 解释执行
    println!("\n正在运行程序...");
    let mut interpreter = x_interpreter::Interpreter::new();
    interpreter.run(&program).unwrap();
    println!("\n运行完成！");
}
