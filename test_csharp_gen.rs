use x_codegen::csharp_backend::{CSharpBackend, CSharpBackendConfig};
use x_parser::ast::{
    BinaryOp, Block, Expression, FunctionDecl, IfStatement, Literal, Program, Statement,
    VariableDecl, WhileStatement,
};

fn main() {
    // Create a test program
    let program = Program {
        declarations: vec![],
        statements: vec![
            // let x = 5
            Statement::Variable(VariableDecl {
                name: "x".to_string(),
                is_mutable: false,
                type_annot: None,
                initializer: Some(Expression::Literal(Literal::Integer(5))),
            }),
            // let y = 10
            Statement::Variable(VariableDecl {
                name: "y".to_string(),
                is_mutable: false,
                type_annot: None,
                initializer: Some(Expression::Literal(Literal::Integer(10))),
            }),
            // println("x = " + x)
            Statement::Expression(Expression::Call(
                Box::new(Expression::Variable("println".to_string())),
                vec![Expression::Binary(
                    BinaryOp::Concat,
                    Box::new(Expression::Literal(Literal::String("x = ".to_string()))),
                    Box::new(Expression::Variable("x".to_string())),
                )],
            )),
            // println("y = " + y)
            Statement::Expression(Expression::Call(
                Box::new(Expression::Variable("println".to_string())),
                vec![Expression::Binary(
                    BinaryOp::Concat,
                    Box::new(Expression::Literal(Literal::String("y = ".to_string()))),
                    Box::new(Expression::Variable("y".to_string())),
                )],
            )),
            // if x < y { println("x is less than y") }
            Statement::If(IfStatement {
                condition: Expression::Binary(
                    BinaryOp::Less,
                    Box::new(Expression::Variable("x".to_string())),
                    Box::new(Expression::Variable("y".to_string())),
                ),
                then_block: Block {
                    statements: vec![Statement::Expression(Expression::Call(
                        Box::new(Expression::Variable("println".to_string())),
                        vec![Expression::Literal(Literal::String(
                            "x is less than y".to_string(),
                        ))],
                    ))],
                },
                else_block: None,
            }),
            // let mutable i = 1
            Statement::Variable(VariableDecl {
                name: "i".to_string(),
                is_mutable: true,
                type_annot: None,
                initializer: Some(Expression::Literal(Literal::Integer(1))),
            }),
            // while i <= 5 { println(i); i = i + 1 }
            Statement::While(WhileStatement {
                condition: Expression::Binary(
                    BinaryOp::LessEqual,
                    Box::new(Expression::Variable("i".to_string())),
                    Box::new(Expression::Literal(Literal::Integer(5))),
                ),
                body: Block {
                    statements: vec![
                        Statement::Expression(Expression::Call(
                            Box::new(Expression::Variable("println".to_string())),
                            vec![Expression::Variable("i".to_string())],
                        )),
                        Statement::Expression(Expression::Assign(
                            Box::new(Expression::Variable("i".to_string())),
                            Box::new(Expression::Binary(
                                BinaryOp::Add,
                                Box::new(Expression::Variable("i".to_string())),
                                Box::new(Expression::Literal(Literal::Integer(1))),
                            )),
                        )),
                    ],
                },
            }),
        ],
    };

    // Generate C# code
    let mut backend = CSharpBackend::new(CSharpBackendConfig::default());
    let output = backend.generate_from_ast(&program).unwrap();
    let csharp_code = String::from_utf8_lossy(&output.files[0].content);

    println!("Generated C# code:");
    println!("{}", csharp_code);
}
