use thiserror::Error;

/// 词法分析错误
#[derive(Error, Debug, PartialEq, Clone)]
pub enum LexError {
    #[error("无效的标记 '{0}' (位置 {1})")]
    InvalidToken(char, usize),
    #[error("字符串字面量未闭合（缺少 \" 结束标记）")]
    UnclosedString,
    #[error("字符字面量未闭合（缺少 ' 结束标记）")]
    UnclosedChar,
    #[error("数字格式无效：在位置 {0} 处 '{1}' 不是有效的数字字面量")]
    InvalidNumber(usize, String),
    #[error("字符串包含无效的 Unicode 转义序列: {0}")]
    InvalidUnicodeEscape(String),
    #[error("字符转义序列无效: {0}")]
    InvalidCharEscape(String),
}
