fn main() {
    let s = "a${b}c";
    let chars = s.chars().peekable();
    let mut lex = chars.clone();
    // peek current
    println!("Current: {:?}", lex.peek().copied()); // Should be 'a'
    let mut cloned = lex.clone();
    cloned.next();
    println!("Next after clone: {:?}", cloned.peek().copied()); // Should be '$'
}
