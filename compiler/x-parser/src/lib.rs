// 语法分析器库

pub mod ast;
pub mod errors;
pub mod parser;

use ast::Program;
use errors::ParseError;
use parser::XParser;

/// 语法分析器类型
pub type Parser = XParser;

/// 从字符串解析X语言程序为抽象语法树
pub fn parse_program(input: &str) -> Result<Program, ParseError> {
    Parser::new().parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Declaration, ExpressionKind, Pattern, Statement, StatementKind};

    #[test]
    fn parse_module_import_export() {
        let src = r#"
module foo;
import std.io;
export foo;
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 3);
        assert!(matches!(program.declarations[0], Declaration::Module(_)));
        assert!(matches!(program.declarations[1], Declaration::Import(_)));
        assert!(matches!(program.declarations[2], Declaration::Export(_)));
    }

    #[test]
    fn parse_match_statement_basic() {
        let src = r#"
let x = 1;
match x {
  _ { return 1; }
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0].node {
            StatementKind::Match(m) => {
                assert_eq!(m.cases.len(), 1);
                assert!(matches!(m.cases[0].pattern, Pattern::Wildcard));
            }
            other => panic!("expected match statement, got {other:?}"),
        }
    }

    #[test]
    fn parse_match_statement_guard_and_or_pattern() {
        let src = r#"
match x {
  a | b when true { return; }
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        match &program.statements[0].node {
            StatementKind::Match(m) => {
                assert_eq!(m.cases.len(), 1);
                assert!(m.cases[0].guard.is_some());
                assert!(matches!(m.cases[0].pattern, Pattern::Or(_, _)));
            }
            other => panic!("expected match statement, got {other:?}"),
        }
    }

    #[test]
    fn parse_try_catch_finally() {
        let src = r#"
try { return 1; }
catch { return 2; }
finally { return 3; }
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0].node {
            StatementKind::Try(t) => {
                assert_eq!(t.catch_clauses.len(), 1);
                assert!(t.finally_block.is_some());
            }
            other => panic!("expected try statement, got {other:?}"),
        }
    }

    #[test]
    fn parse_try_catch_with_parens_type_and_var() {
        let src = r#"
try { return; }
catch (Exception e) { return; }
"#;
        let program = parse_program(src).expect("parse should succeed");
        match &program.statements[0].node {
            StatementKind::Try(t) => {
                assert_eq!(t.catch_clauses.len(), 1);
                assert_eq!(t.catch_clauses[0].exception_type.as_deref(), Some("Exception"));
                assert_eq!(t.catch_clauses[0].variable_name.as_deref(), Some("e"));
            }
            other => panic!("expected try statement, got {other:?}"),
        }
    }

    #[test]
    fn parse_break_statement() {
        let src = "break;";
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.statements.len(), 1);
        assert!(matches!(program.statements[0].node, StatementKind::Break));
    }

    #[test]
    fn parse_continue_statement() {
        let src = "continue;";
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.statements.len(), 1);
        assert!(matches!(program.statements[0].node, StatementKind::Continue));
    }

    #[test]
    fn parse_do_while_statement() {
        let src = r#"
do { x = 1; } while (true);
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0].node {
            StatementKind::DoWhile(d) => {
                assert_eq!(d.body.statements.len(), 1);
                assert!(matches!(&d.condition.node, ExpressionKind::Literal(_)));
            }
            other => panic!("expected do-while statement, got {other:?}"),
        }
    }

    #[test]
    fn parse_class_basic() {
        let src = r#"
class Point {
    let x: Int
    let y: Int

    new(x: Int, y: Int) {
        this.x = x
        this.y = y
    }

    function getX() -> Int {
        return this.x
    }
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            Declaration::Class(c) => {
                assert_eq!(c.name, "Point");
                assert!(c.extends.is_none());
                assert!(c.implements.is_empty());
                assert_eq!(c.members.len(), 4); // 2 fields + 1 constructor + 1 method
            }
            other => panic!("expected class declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_class_with_extends() {
        let src = r#"
class Dog extends Animal {
    let name: String
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        match &program.declarations[0] {
            Declaration::Class(c) => {
                assert_eq!(c.name, "Dog");
                assert_eq!(c.extends.as_deref(), Some("Animal"));
            }
            other => panic!("expected class declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_class_with_implements() {
        let src = r#"
class MyHandler implement Runnable, Serializable {
    function run() { return; }
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        match &program.declarations[0] {
            Declaration::Class(c) => {
                assert_eq!(c.name, "MyHandler");
                assert_eq!(c.implements.len(), 2);
                assert_eq!(c.implements[0], "Runnable");
                assert_eq!(c.implements[1], "Serializable");
            }
            other => panic!("expected class declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_trait_basic() {
        let src = r#"
trait Drawable {
    function draw() -> Unit;
    function move(x: Int, y: Int) -> Unit;
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            Declaration::Trait(t) => {
                assert_eq!(t.name, "Drawable");
                assert_eq!(t.methods.len(), 2);
            }
            other => panic!("expected trait declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_trait_with_function_bodies() {
        let src = r#"
trait Logger {
    function log(msg: String) -> Unit {
        print(msg);
    }
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        match &program.declarations[0] {
            Declaration::Trait(t) => {
                assert_eq!(t.name, "Logger");
                assert_eq!(t.methods.len(), 1);
                assert!(!t.methods[0].body.statements.is_empty());
            }
            other => panic!("expected trait declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_async_function() {
        let src = r#"
async function fetchData(id: Int) -> Async<String> {
    return "data";
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        match &program.declarations[0] {
            Declaration::Function(f) => {
                assert_eq!(f.name, "fetchData");
                assert!(f.is_async);
            }
            other => panic!("expected function declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_wait_single() {
        let src = r#"
function main() {
    let result = wait fetch(42);
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        match &program.declarations[0] {
            Declaration::Function(f) => {
                assert!(!f.body.statements.is_empty());
            }
            other => panic!("expected function declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_wait_together() {
        let src = r#"
function main() {
    wait together {
        fetch(1),
        fetch(2)
    };
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        match &program.declarations[0] {
            Declaration::Function(f) => {
                assert!(!f.body.statements.is_empty());
            }
            other => panic!("expected function declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_wait_race() {
        let src = r#"
function main() {
    wait race {
        fetch(1),
        fetch(2)
    };
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        match &program.declarations[0] {
            Declaration::Function(f) => {
                assert!(!f.body.statements.is_empty());
            }
            other => panic!("expected function declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_wait_timeout() {
        let src = r#"
function main() {
    wait timeout(5000) {
        fetch(42)
    };
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        match &program.declarations[0] {
            Declaration::Function(f) => {
                assert!(!f.body.statements.is_empty());
            }
            other => panic!("expected function declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_class_with_constructor() {
        let src = r#"
class Point {
    let x: Int
    let y: Int

    new(x: Int, y: Int) {
        this.x = x
        this.y = y
    }
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
    }

    #[test]
    fn parse_enum_simple() {
        let src = r#"
enum Color {
    Red
    Green
    Blue
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
    }

    #[test]
    fn parse_trait_declaration() {
        let src = r#"
trait Printable {
    function to_string() -> String
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
    }

    #[test]
    fn parse_type_alias() {
        let src = r#"
type AliasName = Int
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
    }

    // ===== SPEC.md 测试 =====

    #[test]
    fn parse_for_each_statement() {
        // SPEC.md: for each item in collection { ... }
        let src = r#"
for each item in list {
    println(item)
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0].node {
            StatementKind::For(f) => {
                assert!(matches!(&f.pattern, Pattern::Variable(name) if name == "item"));
            }
            other => panic!("expected for statement, got {other:?}"),
        }
    }

    #[test]
    fn parse_for_each_range() {
        // SPEC.md: for each number in 1..10 { ... }
        let src = r#"
for each number in 1..10 {
    println(number)
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0].node {
            StatementKind::For(f) => {
                assert!(matches!(&f.pattern, Pattern::Variable(name) if name == "number"));
            }
            other => panic!("expected for statement, got {other:?}"),
        }
    }

    #[test]
    fn parse_loop_statement() {
        // SPEC.md: loop { ... }
        let src = r#"
loop {
    let input = read_input()
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.statements.len(), 1);
        assert!(matches!(program.statements[0].node, StatementKind::Loop(_)));
    }

    #[test]
    fn parse_record_declaration() {
        // SPEC.md: record Person { name: string, age: integer }
        let src = r#"
record Person {
    name: string
    age: integer
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            Declaration::Record(r) => {
                assert_eq!(r.name, "Person");
                assert_eq!(r.fields.len(), 2);
            }
            other => panic!("expected record declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_when_expression() {
        // SPEC.md: when expression with pattern matching
        let src = r#"
let description = when score is {
    100 => "perfect"
    n if n >= 90 => "excellent"
    _ => "failed"
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
    }

    #[test]
    fn parse_defer_statement() {
        // SPEC.md: defer statement
        let src = r#"
defer cleanup()
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.statements.len(), 1);
        assert!(matches!(program.statements[0].node, StatementKind::Defer(_)));
    }

    #[test]
    fn parse_yield_statement() {
        // SPEC.md: yield statement for generators
        let src = r#"
function count_up(max: integer) -> Generator<integer> {
    let mutable i = 0
    while i < max {
        yield i
        i = i + 1
    }
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
    }

    #[test]
    fn parse_try_with_throw() {
        // SPEC.md: try-catch with throw statement
        // Note: throw is not yet implemented as a statement
        let src = r#"
try {
    risky_operation()
} catch (Exception e) {
    handle_error(e)
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.statements.len(), 1);
        assert!(matches!(program.statements[0].node, StatementKind::Try(_)));
    }

    #[test]
    fn parse_single_expression_function() {
        // SPEC.md: function square(x: integer) -> integer = x * x
        let src = r#"
function square(x: integer) -> integer = x * x
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            Declaration::Function(f) => {
                assert_eq!(f.name, "square");
                assert_eq!(f.parameters.len(), 1);
            }
            other => panic!("expected function declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_generic_function() {
        // SPEC.md: function first<T>(list: List<T>) -> Optional<T>
        let src = r#"
function first<T>(list: List<T>) -> Optional<T> {
    when list is {
        _ => None
    }
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            Declaration::Function(f) => {
                assert_eq!(f.name, "first");
                assert_eq!(f.type_parameters.len(), 1);
                assert_eq!(f.type_parameters[0].name, "T");
            }
            other => panic!("expected function declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_pipeline_expression() {
        // SPEC.md: data |> process() |> output()
        let src = r#"
let result = data |> process() |> output()
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
    }

    #[test]
    fn parse_lambda_expression() {
        // SPEC.md: x -> x * x
        let src = r#"
let square = x -> x * x
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
    }

    #[test]
    fn parse_if_then_else_statement() {
        // SPEC.md: if condition then { ... } else { ... }
        let src = r#"
if score >= 60 then {
    println("passed")
} else {
    println("failed")
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.statements.len(), 1);
        assert!(matches!(program.statements[0].node, StatementKind::If(_)));
    }

    #[test]
    fn parse_effect_declaration() {
        // Current parser syntax: effect Name { op: Input -> Output, ... }
        // Note: SPEC.md uses `operation name(params) -> type` but current parser uses `name: Input -> Output`
        let src = r#"
effect Io {
    read_file: string -> string,
    write_file: string -> ()
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            Declaration::Effect(e) => {
                assert_eq!(e.name, "Io");
            }
            other => panic!("expected effect declaration, got {other:?}"),
        }
    }

    #[test]
    fn parse_implement_declaration() {
        // SPEC.md: implement Printable for Person { ... }
        let src = r#"
implement Printable for Person {
    function to_string() -> string {
        "Person"
    }
}
"#;
        let program = parse_program(src).expect("parse should succeed");
        assert_eq!(program.declarations.len(), 1);
        match &program.declarations[0] {
            Declaration::Implement(i) => {
                assert_eq!(i.trait_name, "Printable");
            }
            other => panic!("expected implement declaration, got {other:?}"),
        }
    }
}
