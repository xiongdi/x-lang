
// 简单测试，直接使用库
fn main() {
    println!("Testing...");

    // 尝试读取并解析 hello.x
    let content = std::fs::read_to_string("examples/hello.x").unwrap();
    println!("Read hello.x:\n{}", content);
    println!("\nFile read successfully!");
}
