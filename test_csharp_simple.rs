use x_codegen::csharp_backend::{CSharpBackend, CSharpBackendConfig};
use x_parser::ast::{BinaryOp, Expression, Literal, Program, Statement, VariableDecl};

fn main() {
    // 简单测试程序：Hello World + 算术运算
    let program = Program {
        declarations: vec![],
        statements: vec![
            // println("Hello from X Language .NET Backend!")
            Statement::Expression(Expression::Call(
                Box::new(Expression::Variable("println".to_string())),
                vec![Expression::Literal(Literal::String(
                    "Hello from X Language .NET Backend!".to_string(),
                ))],
            )),
            // let a = 10
            Statement::Variable(VariableDecl {
                name: "a".to_string(),
                is_mutable: false,
                type_annot: None,
                initializer: Some(Expression::Literal(Literal::Integer(10))),
            }),
            // let b = 20
            Statement::Variable(VariableDecl {
                name: "b".to_string(),
                is_mutable: false,
                type_annot: None,
                initializer: Some(Expression::Literal(Literal::Integer(20))),
            }),
            // println("a + b = " + (a + b))
            Statement::Expression(Expression::Call(
                Box::new(Expression::Variable("println".to_string())),
                vec![Expression::Binary(
                    BinaryOp::Add,
                    Box::new(Expression::Literal(Literal::String("a + b = ".to_string()))),
                    Box::new(Expression::Binary(
                        BinaryOp::Add,
                        Box::new(Expression::Variable("a".to_string())),
                        Box::new(Expression::Variable("b".to_string())),
                    )),
                )],
            )),
        ],
    };

    // 生成 C# 代码
    let mut backend = CSharpBackend::new(CSharpBackendConfig::default());
    let output = backend.generate_from_ast(&program).unwrap();
    let csharp_code = String::from_utf8_lossy(&output.files[0].content);

    println!("=== 生成的 C# 代码 ===");
    println!("{}", csharp_code);
    println!("======================");

    // 保存到文件
    std::fs::write("TestProgram.cs", csharp_code).unwrap();
    println!("已保存到 TestProgram.cs");

    // 尝试编译运行
    println!("正在尝试编译和运行...");
    let status = std::process::Command::new("dotnet")
        .args(["script", "TestProgram.cs"])
        .status()
        .unwrap();

    if status.success() {
        println!("✅ .NET 后端测试成功！代码生成和运行正常");
    } else {
        println!("❌ 运行失败，需要先安装 dotnet script 工具: dotnet tool install -g dotnet-script");
    }
}
