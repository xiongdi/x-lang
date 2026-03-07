# Standard Library Types Implementation Plan

## [x] Task 1: Remove Option, Result, Some, None, Ok, Err from lexer keywords
- **Priority**: P0
- **Depends On**: None
- **Description**: 
  - Remove all entries for "none", "some", "Some", "ok", "Ok", "err", "Err" from the lexer's parse_identifier method
  - These should be treated as regular identifiers instead of special tokens
- **Success Criteria**:
  - Lexer no longer generates special tokens for these identifiers
  - They are parsed as regular Token::Ident
- **Test Requirements**:
  - `programmatic` TR-1.1: Lexer should generate Token::Ident for "Some", "Ok", "Err", "None"
  - `programmatic` TR-1.2: Lexer should not generate special tokens for these identifiers
- **Notes**: This is the first step to make these types come from the standard library

## [x] Task 2: Remove special handling for Option, Result, Some, None, Ok, Err in parser
- **Priority**: P0
- **Depends On**: Task 1
- **Description**: 
  - Remove special handling for Token::NoneKeyword, Token::Some, Token::Ok, Token::Err in the parser's parse_primary method
  - Remove special handling for Option and Result types in the parse_type method
  - These should be treated as regular identifiers and types
- **Success Criteria**:
  - Parser no longer has special cases for these types
  - They are parsed as regular identifiers and types
- **Test Requirements**:
  - `programmatic` TR-2.1: Parser should parse "Some(42)" as a function call to "Some"
  - `programmatic` TR-2.2: Parser should parse "Option<Int>" as a generic type
- **Notes**: This allows the standard library to define these types

## [x] Task 3: Verify all tests pass with standard library types
- **Priority**: P1
- **Depends On**: Task 1, Task 2
- **Description**: 
  - Run all tests to ensure they pass with the changes
  - Verify that Option, Result, Some, None, Ok, Err are properly handled as standard library types
- **Success Criteria**:
  - All 30 tests pass successfully
  - No parsing errors for Option/Result types or their constructors
- **Test Requirements**:
  - `programmatic` TR-3.1: All tests pass without errors
  - `programmatic` TR-3.2: Tests specifically for Option and Result types pass
- **Notes**: This confirms that the changes work correctly and the standard library provides the necessary types