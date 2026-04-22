// 词法分析器库

pub mod errors;
pub mod span;
pub mod token;

use errors::LexError;
use span::Span;
use token::Token;

/// UTF-8 BOM 字节序列
const UTF8_BOM: [u8; 3] = [0xEF, 0xBB, 0xBF];

/// 词法分析器状态
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LexerState {
    Normal,
    String,
    MultilineString,
    RawString,
    StringInterpolate,
}

/// 词法分析器
#[derive(Clone)]
pub struct Lexer<'a> {
    pub input: &'a str,
    pub chars: std::iter::Peekable<std::str::Chars<'a>>,
    pub position: usize,
    pub state_stack: Vec<LexerState>,
    pub recovery_mode: bool,
    pub skipped_positions: Vec<(usize, char)>,
    /// 收集到的词法错误
    pub errors: Vec<(LexError, Span)>,
    cached_next: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let input = if input.len() >= 3 && &input.as_bytes()[..3] == UTF8_BOM {
            &input[3..]
        } else {
            input
        };

        let chars = input.chars().peekable();
        let mut cloned = chars.clone();
        cloned.next();
        let next = cloned.peek().copied();

        Self {
            input,
            chars,
            position: 0,
            state_stack: vec![LexerState::Normal],
            recovery_mode: false,
            skipped_positions: Vec::new(),
            errors: Vec::new(),
            cached_next: next,
        }
    }

    pub fn current_char(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn peek_next(&mut self) -> Option<char> {
        self.cached_next
    }

    pub fn peek_nth(&mut self, n: usize) -> Option<char> {
        let mut cloned = self.chars.clone();
        for _ in 0..n {
            cloned.next();
        }
        cloned.peek().copied()
    }

    fn next_char(&mut self) {
        if let Some(ch) = self.chars.next() {
            self.position += ch.len_utf8();
        }
        let mut cloned = self.chars.clone();
        cloned.next();
        self.cached_next = cloned.peek().copied();
    }

    fn current_state(&self) -> LexerState {
        *self.state_stack.last().unwrap_or(&LexerState::Normal)
    }

    fn push_state(&mut self, state: LexerState) {
        self.state_stack.push(state);
    }

    fn pop_state(&mut self) -> LexerState {
        self.state_stack.pop().unwrap_or(LexerState::Normal)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) -> bool {
        if self.current_char() == Some('/') && self.peek_next() == Some('/') {
            self.next_char();
            self.next_char();
            while let Some(ch) = self.current_char() {
                self.next_char();
                if ch == '\n' { break; }
            }
            true
        } else {
            false
        }
    }

    fn skip_block_comment(&mut self) -> bool {
        if self.current_char() == Some('/') && self.peek_next() == Some('*') {
            self.next_char();
            self.next_char();
            let mut depth = 1usize;
            while depth > 0 {
                match self.current_char() {
                    Some('/') if self.peek_next() == Some('*') => {
                        self.next_char(); self.next_char();
                        depth += 1;
                    }
                    Some('*') if self.peek_next() == Some('/') => {
                        self.next_char(); self.next_char();
                        depth -= 1;
                    }
                    Some(_) => { self.next_char(); }
                    None => break,
                }
            }
            true
        } else {
            false
        }
    }

    fn parse_identifier(&mut self) -> Result<Token, LexError> {
        let mut ident = String::new();
        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                ident.push(ch);
                self.next_char();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "let" => Ok(Token::Let),
            "mut" => Ok(Token::Mut),
            "mutable" => Ok(Token::Mutable),
            "val" => Ok(Token::Val),
            "var" => Ok(Token::Var),
            "const" | "constant" => Ok(Token::Constant),
            "function" => Ok(Token::Function),
            "async" => Ok(Token::Async),
            "class" => Ok(Token::Class),
            "struct" => Ok(Token::Struct),
            "enum" => Ok(Token::Enum),
            "extends" => Ok(Token::Extends),
            "trait" => Ok(Token::Trait),
            "interface" => Ok(Token::Interface),
            "implement" => Ok(Token::Implement),
            "implements" => Ok(Token::Implements),
            "abstract" => Ok(Token::Abstract),
            "super" => Ok(Token::Super),
            "type" => Ok(Token::Type),
            "newtype" => Ok(Token::Newtype),
            "new" => Ok(Token::New),
            "virtual" => Ok(Token::Virtual),
            "override" => Ok(Token::Override),
            "final" => Ok(Token::Final),
            "static" => Ok(Token::Static),
            "private" => Ok(Token::Private),
            "public" => Ok(Token::Public),
            "protected" => Ok(Token::Protected),
            "module" => Ok(Token::Module),
            "internal" => Ok(Token::Internal),
            "import" => Ok(Token::Import),
            "export" => Ok(Token::Export),
            "return" => Ok(Token::Return),
            "if" => Ok(Token::If),
            "then" => Ok(Token::Then),
            "else" => Ok(Token::Else),
            "for" => Ok(Token::For),
            "each" => Ok(Token::Each),
            "in" => Ok(Token::In),
            "while" => Ok(Token::While),
            "break" => Ok(Token::Break),
            "continue" => Ok(Token::Continue),
            "match" => Ok(Token::Match),
            "when" => Ok(Token::When),
            "is" => Ok(Token::Is),
            "where" => Ok(Token::Where),
            "and" => Ok(Token::And),
            "or" => Ok(Token::Or),
            "not" => Ok(Token::Not),
            "eq" => Ok(Token::Eq),
            "ne" => Ok(Token::Ne),
            "true" => Ok(Token::True),
            "false" => Ok(Token::False),
            "null" => Ok(Token::Null),
            "effect" => Ok(Token::Effect),
            "self" => Ok(Token::SelfLower),
            "Self" => Ok(Token::SelfUpper),
            "concurrently" => Ok(Token::Concurrently),
            "record" => Ok(Token::Record),
            "constructor" => Ok(Token::Constructor),
            "perform" => Ok(Token::Perform),
            "operation" => Ok(Token::Operation),
            "await" => Ok(Token::Await),
            "needs" => Ok(Token::Needs),
            "given" => Ok(Token::Given),
            "wait" => Ok(Token::Wait),
            "together" => Ok(Token::Concurrently),
            "race" => Ok(Token::Race),
            "timeout" => Ok(Token::Timeout),
            "atomic" => Ok(Token::Atomic),
            "retry" => Ok(Token::Retry),
            "use" => Ok(Token::Use),
            "with" => Ok(Token::With),
            "throws" => Ok(Token::Throws),
            "try" => Ok(Token::Try),
            "catch" => Ok(Token::Catch),
            "finally" => Ok(Token::Finally),
            "throw" => Ok(Token::Throw),
            "handle" => Ok(Token::Handle),
            "defer" => Ok(Token::Defer),
            "yield" => Ok(Token::Yield),
            "loop" => Ok(Token::Loop),
            "as" => Ok(Token::As),
            "extern" => Ok(Token::Extern),
            "foreign" => Ok(Token::Foreign),
            "external" => Ok(Token::External),
            "unsafe" => Ok(Token::Unsafe),
            _ => Ok(Token::Ident(ident)),
        }
    }

    fn parse_operator(&mut self) -> Result<Token, LexError> {
        let ch = self.current_char().ok_or(LexError::UnclosedChar)?;
        self.next_char();
        let next = self.current_char();

        match ch {
            '=' => if next == Some('=') { self.next_char(); Ok(Token::DoubleEquals) }
                   else if next == Some('>') { self.next_char(); Ok(Token::FatArrow) }
                   else { Ok(Token::Equals) },
            '!' => if next == Some('=') { self.next_char(); Ok(Token::NotEquals) }
                   else { Ok(Token::NotOperator) },
            '<' => if next == Some('=') { self.next_char(); Ok(Token::LessThanEquals) }
                   else if next == Some('<') {
                       self.next_char();
                       if self.current_char() == Some('=') { self.next_char(); Ok(Token::LeftShiftEquals) }
                       else { Ok(Token::LeftShift) }
                   }
                   else { Ok(Token::LessThan) },
            '>' => if next == Some('=') { self.next_char(); Ok(Token::GreaterThanEquals) }
                   else if next == Some('>') {
                       self.next_char();
                       if self.current_char() == Some('=') { self.next_char(); Ok(Token::RightShiftEquals) }
                       else { Ok(Token::RightShift) }
                   }
                   else { Ok(Token::GreaterThan) },
            '+' => if next == Some('=') { self.next_char(); Ok(Token::PlusEquals) }
                   else { Ok(Token::Plus) },
            '-' => if next == Some('>') { self.next_char(); Ok(Token::Arrow) }
                   else if next == Some('=') { self.next_char(); Ok(Token::MinusEquals) }
                   else { Ok(Token::Minus) },
            '*' => if next == Some('=') { self.next_char(); Ok(Token::AsteriskEquals) }
                   else { Ok(Token::Asterisk) },
            '/' => if next == Some('=') { self.next_char(); Ok(Token::SlashEquals) }
                   else { Ok(Token::Slash) },
            '%' => if next == Some('=') { self.next_char(); Ok(Token::PercentEquals) }
                   else { Ok(Token::Percent) },
            '^' => if next == Some('=') { self.next_char(); Ok(Token::CaretEquals) }
                   else { Ok(Token::Caret) },
            ':' => if next == Some(':') { self.next_char(); Ok(Token::DoubleColon) }
                   else { Ok(Token::Colon) },
            '.' => if next == Some('.') {
                       self.next_char();
                       if self.current_char() == Some('=') { self.next_char(); Ok(Token::RangeInclusive) }
                       else { Ok(Token::RangeExclusive) }
                   }
                   else { Ok(Token::Dot) },
            ',' => Ok(Token::Comma),
            ';' => Ok(Token::Semicolon),
            '(' => Ok(Token::LeftParen),
            ')' => Ok(Token::RightParen),
            '{' => Ok(Token::LeftBrace),
            '}' => Ok(Token::RightBrace),
            '[' => Ok(Token::LeftBracket),
            ']' => Ok(Token::RightBracket),
            '|' => if next == Some('|') { self.next_char(); Ok(Token::OrOr) }
                   else if next == Some('>') { self.next_char(); Ok(Token::Pipe) }
                   else if next == Some('=') { self.next_char(); Ok(Token::PipeEquals) }
                   else { Ok(Token::VerticalBar) },
            '&' => if next == Some('&') { self.next_char(); Ok(Token::AndAnd) }
                   else if next == Some('=') { self.next_char(); Ok(Token::AmpersandEquals) }
                   else { Ok(Token::Ampersand) },
            '~' => Ok(Token::Tilde),
            '?' => if next == Some('?') { self.next_char(); Ok(Token::DoubleQuestionMark) }
                   else if next == Some('.') { self.next_char(); Ok(Token::QuestionMarkDot) }
                   else { Ok(Token::QuestionMark) },
            '@' => Ok(Token::AtSign),
            '#' => Ok(Token::Hash),
            _ => Err(LexError::InvalidToken(ch, self.position - 1)),
        }
    }

    pub fn next_token(&mut self) -> Result<(Token, Span), LexError> {
        loop {
            match self.current_state() {
                LexerState::Normal => {
                    self.skip_whitespace();
                    let start = self.position;
                    match self.current_char() {
                        Some('/') => {
                            if self.skip_line_comment() || self.skip_block_comment() {
                                continue;
                            }
                            let result = self.parse_operator();
                            let end = self.position;
                            match result {
                                Ok(t) => return Ok((t, Span::new(start, end))),
                                Err(e) => {
                                    self.errors.push((e.clone(), Span::new(start, end)));
                                    continue;
                                }
                            }
                        }
                        Some(ch) if ch.is_alphabetic() || ch == '_' => {
                            let result = self.parse_identifier();
                            let end = self.position;
                            match result {
                                Ok(t) => return Ok((t, Span::new(start, end))),
                                Err(e) => {
                                    self.errors.push((e.clone(), Span::new(start, end)));
                                    continue;
                                }
                            }
                        }
                        Some(ch) if ch.is_ascii_digit() => {
                            let result = self.parse_number();
                            let end = self.position;
                            match result {
                                Ok(t) => return Ok((t, Span::new(start, end))),
                                Err(e) => {
                                    self.errors.push((e.clone(), Span::new(start, end)));
                                    continue;
                                }
                            }
                        }
                        Some('"') => {
                            self.next_char();
                            if self.current_char() == Some('"') && self.peek_next() == Some('"') {
                                self.next_char();
                                self.next_char();
                                self.push_state(LexerState::MultilineString);
                                return Ok((
                                    Token::MultilineStringQuote,
                                    Span::new(start, self.position),
                                ));
                            }
                            self.push_state(LexerState::String);
                            return Ok((Token::StringQuote, Span::new(start, self.position)));
                        }
                        Some('\'') => {
                            let result = self.parse_char();
                            match result {
                                Ok(t) => return Ok((t, Span::new(start, self.position))),
                                Err(e) => {
                                    self.errors.push((e, Span::new(start, self.position)));
                                    continue;
                                }
                            }
                        }
                        Some('}') => {
                            // Check if we're in StringInterpolate
                            if self.state_stack.len() >= 2
                                && self.state_stack[self.state_stack.len() - 2]
                                    == LexerState::StringInterpolate
                            {
                                self.pop_state(); // Pop Normal
                                self.pop_state(); // Pop StringInterpolate
                                self.next_char();
                                return Ok((Token::InterpolateEnd, Span::new(start, self.position)));
                            }
                            let result = self.parse_operator();
                            match result {
                                Ok(t) => return Ok((t, Span::new(start, self.position))),
                                Err(e) => {
                                    self.errors.push((e, Span::new(start, self.position)));
                                    continue;
                                }
                            }
                        }
                        Some(ch) if "~!@#$%^&*()_+{[]|;:,.<>?\\-=".contains(ch) => {
                            let result = self.parse_operator();
                            let end = self.position;
                            match result {
                                Ok(t) => return Ok((t, Span::new(start, end))),
                                Err(e) => {
                                    self.errors.push((e, Span::new(start, end)));
                                    continue;
                                }
                            }
                        }
                        Some(ch) => {
                            let start_err = self.position;
                            self.next_char();
                            let end_err = self.position;
                            self.errors.push((
                                LexError::InvalidToken(ch, start_err),
                                Span::new(start_err, end_err),
                            ));
                            continue;
                        }
                        None => return Ok((Token::Eof, Span::new(start, start))),
                    }
                }
                LexerState::String | LexerState::MultilineString => {
                    let start = self.position;
                    let current = self.current_char();
                    let state = self.current_state();

                    if state == LexerState::String && current == Some('"') {
                        self.next_char();
                        self.pop_state();
                        return Ok((Token::StringQuote, Span::new(start, self.position)));
                    }
                    if state == LexerState::MultilineString
                        && current == Some('"')
                        && self.peek_next() == Some('"')
                        && self.peek_nth(2) == Some('"')
                    {
                        self.next_char();
                        self.next_char();
                        self.next_char();
                        self.pop_state();
                        return Ok((Token::MultilineStringQuote, Span::new(start, self.position)));
                    }
                    if current.is_none() {
                        self.pop_state();
                        self.errors.push((LexError::UnclosedString, Span::new(start, start)));
                        return Ok((Token::Eof, Span::new(start, start)));
                    }

                    let result = self.parse_string_content();
                    match result {
                        Ok(t) => return Ok((t, Span::new(start, self.position))),
                        Err(e) => {
                            self.errors.push((e, Span::new(start, self.position)));
                            continue;
                        }
                    }
                }
                LexerState::StringInterpolate => {
                    self.push_state(LexerState::Normal);
                    continue;
                }
                _ => {
                    self.pop_state();
                    continue;
                }
            }
        }
    }

    fn parse_number(&mut self) -> Result<Token, LexError> {
        let mut s = String::new();
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() || c == '_' { s.push(c); self.next_char(); }
            else { break; }
        }
        if self.current_char() == Some('.') && self.peek_next().map_or(false, |c| c.is_ascii_digit()) {
            s.push('.'); self.next_char();
            while let Some(c) = self.current_char() {
                if c.is_ascii_digit() || c == '_' { s.push(c); self.next_char(); }
                else { break; }
            }
            return Ok(Token::Float(s));
        }
        Ok(Token::DecimalInt(s))
    }

    fn parse_string_content(&mut self) -> Result<Token, LexError> {
        let mut content = String::new();
        while let Some(ch) = self.current_char() {
            if ch == '"' { break; }
            if ch == '$' {
                if self.peek_next() == Some('{') {
                    if !content.is_empty() { return Ok(Token::StringContent(content)); }
                    self.next_char(); self.next_char();
                    self.push_state(LexerState::StringInterpolate);
                    return Ok(Token::InterpolateStart);
                } else if self.peek_next().map_or(false, |c| c.is_alphabetic() || c == '_') {
                    if !content.is_empty() { return Ok(Token::StringContent(content)); }
                    self.next_char(); // consume $
                    let mut name = String::new();
                    while let Some(c) = self.current_char() {
                        if c.is_alphanumeric() || c == '_' { name.push(c); self.next_char(); }
                        else { break; }
                    }
                    return Ok(Token::Ident(name));
                }
            }
            if ch == '\\' {
                self.next_char();
                if let Some(ec) = self.current_char() {
                    match ec {
                        'n' => content.push('\n'), 't' => content.push('\t'),
                        'r' => content.push('\r'), '\\' => content.push('\\'),
                        '"' => content.push('"'), _ => content.push(ec),
                    }
                    self.next_char(); continue;
                }
            }
            content.push(ch); self.next_char();
        }
        Ok(Token::StringContent(content))
    }

    fn parse_char(&mut self) -> Result<Token, LexError> {
        self.next_char();
        let ch = self.current_char().ok_or(LexError::UnclosedChar)?;
        self.next_char();
        if self.current_char() != Some('\'') { return Err(LexError::UnclosedChar); }
        self.next_char();
        Ok(Token::CharContent(ch.to_string()))
    }

    fn parse_raw_string_content(&mut self) -> Result<Token, LexError> {
        let mut content = String::new();
        while let Some(ch) = self.current_char() {
            if ch == '`' { self.next_char(); self.pop_state(); return Ok(Token::StringContent(content)); }
            content.push(ch); self.next_char();
        }
        self.pop_state(); Err(LexError::UnclosedString)
    }
}

#[derive(Clone)]
pub struct TokenIterator<'a> {
    lexer: Lexer<'a>,
    peeked: Option<Result<(Token, Span), LexError>>,
    /// 暂存的回退 Token 栈
    push_back_stack: Vec<Result<(Token, Span), LexError>>,
    pub last_span: Option<Span>,
}

impl<'a> TokenIterator<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input),
            peeked: None,
            push_back_stack: Vec::new(),
            last_span: None,
        }
    }

    /// 将一个 Token 回退到流中
    pub fn push_back(&mut self, item: Result<(Token, Span), LexError>) {
        if let Some(p) = self.peeked.take() {
            self.push_back_stack.push(p);
        }
        self.push_back_stack.push(item);
    }

    /// 获取解析过程中产生的所有错误
    pub fn get_errors(&self) -> Vec<(LexError, Span)> {
        self.lexer.errors.clone()
    }

    pub fn peek(&mut self) -> Option<&Result<(Token, Span), LexError>> {
        if self.peeked.is_none() && !self.push_back_stack.is_empty() {
            self.peeked = self.push_back_stack.pop();
        }

        if self.peeked.is_none() {
            self.peeked = match self.lexer.next_token() {
                Ok((Token::Eof, _)) => None,
                Ok(ok) => Some(Ok(ok)),
                Err(e) => Some(Err(e)),
            };
        }
        self.peeked.as_ref()
    }
}

impl<'a> Iterator for TokenIterator<'a> {
    type Item = Result<(Token, Span), LexError>;
    fn next(&mut self) -> Option<Self::Item> {
        let item = if let Some(p) = self.peeked.take() {
            Some(p)
        } else if !self.push_back_stack.is_empty() {
            self.push_back_stack.pop()
        } else {
            match self.lexer.next_token() {
                Ok((Token::Eof, _)) => None,
                Ok(ok) => Some(Ok(ok)),
                Err(e) => Some(Err(e)),
            }
        };

        if let Some(Ok((_, span))) = item.as_ref() {
            self.last_span = Some(*span);
        }
        item
    }
}

pub fn new_lexer(input: &str) -> TokenIterator<'_> { TokenIterator::new(input) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lex_basic() {
        let input = "let x = 42;";
        let tokens: Vec<_> = new_lexer(input).filter_map(Result::ok).map(|(t, _)| t).collect();
        assert_eq!(tokens.len(), 5);
    }
    #[test]
    fn test_lex_string() {
        let input = r#""hello""#;
        let tokens: Vec<_> = new_lexer(input).filter_map(Result::ok).map(|(t, _)| t).collect();
        assert_eq!(tokens.len(), 3);
        assert!(matches!(tokens[0], Token::StringQuote));
        assert!(matches!(&tokens[1], Token::StringContent(s) if s == "hello"));
        assert!(matches!(tokens[2], Token::StringQuote));
    }

    #[test]
    fn test_lex_multiple_errors() {
        let input = "let x = @; let y = #; let z = 1.2.3;";
        let mut iter = new_lexer(input);
        let _tokens: Vec<_> = iter.by_ref().collect(); // Consume all tokens
        let errors = iter.get_errors();

        // Should find at least 2 errors (@ and # are not valid tokens in many contexts, 
        // though @ and # ARE in our parse_operator... wait)
        
        // Let's use something definitely invalid: control characters or random symbols not in parse_operator
        let input2 = "let a = \x01; let b = \x02;";
        let mut iter2 = new_lexer(input2);
        let _tokens2: Vec<_> = iter2.by_ref().collect();
        let errors2 = iter2.get_errors();
        assert_eq!(errors2.len(), 2);
        assert!(matches!(errors2[0].0, LexError::InvalidToken('\x01', _)));
        assert!(matches!(errors2[1].0, LexError::InvalidToken('\x02', _)));
    }
}
