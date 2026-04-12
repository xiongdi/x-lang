
use x_lexer::{Lexer, Token};

fn main() {
    let input = r#""${a} + ${b} = ${a + b}""#;
    println!("Input: {}", input);
    println!();
    println!("Tokens:");
    println!();

    let mut lexer = Lexer::new(input);
    let mut i = 0;

    loop {
        match lexer.next_token() {
            Ok((token, span)) => {
                println!("  {:3}: {:<25} @ {:?}", i, format!("{:?}", token), span);
                i += 1;
                if let Token::Eof | Token::StringQuote = token {
                    break;
                }
            }
            Err(e) => {
                println!("  ERROR: {:?}", e);
                break;
            }
        }
    }
}
