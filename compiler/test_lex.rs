extern crate x_lexer;
use x_lexer::{Lexer, Token};

fn main() {
    let src = r#""${a} + ${b} = ${a + b}""#;
    println!("Lexing: {}", src);
    let mut lexer = Lexer::new(src);
    let mut i = 0;
    while let Some(result) = lexer.next() {
        match result {
            Ok((token, span)) => {
                println!("[{}] span {}..{}: {:?}", i, span.start, span.end, token);
                i += 1;
            }
            Err(e) => {
                println!("[{}] ERROR: {}", i, e);
                i += 1;
            }
        }
    }
}
