use x_parser::parser::XParser;
use x_interpreter::Interpreter;

fn main() {
    let source = "1 + 1";
    let parser = XParser::new();
    
    println!("Testing interpreter with source: {}", source);
    
    match parser.parse(source) {
        Ok(program) => {
            let mut interpreter = Interpreter::new();
            match interpreter.run(&program) {
                Ok(_) => println!("Interpreter test completed successfully!"),
                Err(e) => println!("Interpreter error: {:?}", e),
            }
        }
        Err(e) => {
            println!("Parser error: {:?}", e);
        }
    }
}
