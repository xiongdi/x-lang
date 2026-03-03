# 第1章 词法结构

## 1.1 字符集

### 定义
X语言源文本使用Unicode字符集。

```
Unicode = { U+0000, U+0001, ..., U+10FFFF }
```

### 输入解析
```
Source → SourceChar*
SourceChar → Unicode \ { CR, LF, NEL, LS, PS }
           | Newline
Newline → CR | LF | CR LF | NEL | LS | PS
```

## 1.2 词法记号

### 记号定义
```
Token → Keyword
       | Identifier
       | Literal
       | Operator
       | Punctuator
       | Comment
```

### 1.2.1 关键字

#### 定义
关键字是保留的标识符，不能用作变量名。

```
Keyword → 'let' | 'mut'
         | 'fun' | 'async'
         | 'class' | 'extends' | 'trait' | 'type'
         | 'if' | 'else' | 'for' | 'in' | 'while'
         | 'return' | 'when' | 'is' | 'where'
         | 'and' | 'or' | 'not'
         | 'true' | 'false' | 'null' | 'none' | 'some'
         | 'ok' | 'err'
         | 'needs' | 'given' | 'wait' | 'together' | 'race' | 'timeout'
         | 'atomic' | 'retry' | 'use' | 'with' | 'throws'
         | 'try' | 'catch' | 'finally' | 'throw'
         | 'new' | 'virtual' | 'override' | 'final'
         | 'private' | 'public' | 'protected'
         | 'module' | 'internal' | 'import' | 'export'
```

#### 分类
```
DeclarationKeyword = { 'let', 'mut', 'fun', 'async',
                      'class', 'trait', 'type', 'module' }
ControlKeyword = { 'if', 'else', 'for', 'in', 'while',
                  'return', 'match' }
EffectKeyword = { 'needs', 'given', 'wait', 'atomic',
                 'try', 'catch', 'finally', 'throw' }
LiteralKeyword = { 'true', 'false', 'null', 'none', 'some', 'ok', 'err' }
```

### 1.2.2 标识符

#### 定义
```
Identifier → IdentifierStart IdentifierPart*
IdentifierStart → UnicodeLetter | '_'
IdentifierPart → IdentifierStart | UnicodeDigit | '-'
UnicodeLetter → { U+0041..U+005A, U+0061..U+007A, ... }  // 所有Unicode字母
UnicodeDigit → { U+0030..U+0039 }  // 0-9
```

#### 示例
```
ValidIdentifiers = { x, user_name, user-name, _temp, item123 }
InvalidIdentifiers = { 123abc, -user, let }
```

### 1.2.3 字面量

#### 整数字面量
```
IntegerLiteral → DecimalLiteral
               | HexLiteral
               | OctalLiteral
               | BinaryLiteral

DecimalLiteral → NonZeroDigit ( '_'? Digit )*
HexLiteral → '0x' HexDigit ( '_'? HexDigit )*
OctalLiteral → '0o' OctalDigit ( '_'? OctalDigit )*
BinaryLiteral → '0b' BinaryDigit ( '_'? BinaryDigit )*

NonZeroDigit → '1' | '2' | ... | '9'
Digit → '0' | NonZeroDigit
HexDigit → Digit | 'a' | 'b' | 'c' | 'd' | 'e' | 'f'
           | 'A' | 'B' | 'C' | 'D' | 'E' | 'F'
OctalDigit → '0' | '1' | ... | '7'
BinaryDigit → '0' | '1'
```

#### 浮点数字面量
```
FloatLiteral → ( Digit+ '.' Digit* ExponentPart?
              | '.' Digit+ ExponentPart?
              | Digit+ ExponentPart )
ExponentPart → ( 'e' | 'E' ) ( '+' | '-' )? Digit+
```

#### 字符串字面量
```
StringLiteral → '"' StringCharacter* '"'
              | '"""' MultiLineStringCharacter* '"""'

StringCharacter → SourceCharacter \ { '"', '\\' }
                | EscapeSequence
MultiLineStringCharacter → SourceCharacter \ { '"""' }
                          | EscapeSequence

EscapeSequence → '\\' ( 'n' | 't' | 'r' | '0' | '"' | '\'' | '\\' )
```

#### 布尔字面量
```
BooleanLiteral → 'true' | 'false'
```

### 1.2.4 运算符与标点符号

#### 运算符定义
```
Operator → '+' | '-' | '*' | '/' | '%' | '^'
         | '==' | '!=' | '<' | '>' | '<=' | '>='
         | '&&' | '||' | '!'
         | '+' '=' | '-' '=' | '*' '=' | '/' '=' | '%' '=' | '^' '='
         | '|>' | '.' | ':' | '::' | '->' | '=>'
         | '..' | '..='
```

#### 标点符号定义
```
Punctuator → '(' | ')' | '{' | '}' | '[' | ']'
           | ',' | ';' | '|' | '&' | '~' | '?' | '@' | '#'
```

## 1.3 注释

### 定义
注释是被忽略的文本，用于代码文档化。

```
Comment → LineComment | BlockComment
```

### 1.3.1 单行注释

#### 数学定义
```
LineComment → '//' LineCommentChar* Newline?
LineCommentChar → SourceCharacter \ Newline
```

#### 自然语言说明
使用 `//` 开头，直到行尾。例如：
```x
// 这是一个单行注释
let x = 42  // 可以跟在代码后面
```

### 1.3.2 多行注释

#### 数学定义
```
BlockComment → '/**' BlockCommentChar* '*/'
BlockCommentChar → SourceCharacter
                 | BlockComment  // 支持嵌套
```

#### 自然语言说明
使用 `/**` 开头，`*/` 结尾，可以跨行。支持嵌套。例如：
```x
/**
 * 这是多行注释
 * 可以有多行
 */
fun main() {
    /**
     * 支持嵌套注释
     */
    let x = 42
}
```

## 1.4 空白字符

### 定义
空白字符是被忽略的字符，用于分隔记号。

```
Whitespace → ( ' ' | '\t' | '\r' | '\n' | '\x0B' | '\x0C' )+
```

### 处理规则
```
TokenStream → ( Whitespace? Token Whitespace? )* Whitespace?
```

## 1.5 词法分析器状态机

### 状态转换
```
State → Normal | String | MultiLineString | Comment | BlockComment

δ(Normal, '/') = if peek = '/' → LineComment
                  if peek = '*' and peek2 = '*' → BlockComment
                  else → Operator
δ(Normal, '"') = if '"' '"' follows → MultiLineString
                  else → String
δ(String, '"') = Normal
δ(MultiLineString, '"' '"' '"') = Normal
δ(LineComment, Newline) = Normal
δ(BlockComment, '*' '/') = Normal
```

---

**本章规范采用数学语言定义词法结构，简洁清晰明了。**
