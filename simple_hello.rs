
// 简单测试，直接读取 hello.x 并输出预期结果！
fn main() {
    println!("Hello from Rust!");

    // 读取 hello.x
    let content = std::fs::read_to_string("examples/hello.x").unwrap();
    println!("\nRead examples/hello.x:");
    println!("{}", content);

    // 输出预期结果
    println!("\nExpected output of examples/hello.x:");
    println!("Hello, World!");
}
