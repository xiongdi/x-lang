// Error Handling Test Suite
// Tests for error types, propagation, and stack traces

module tests.error_handling

import std::prelude::*;
import std::error::*;
import std::errors::*;
import std::panic::*;

// ============================================================================
// Test: Basic Error Creation
// ============================================================================

function test_simple_error() -> Bool {
    let err = simple_error("something went wrong");
    assert(err.message == "something went wrong");
    assert(err.error_code == None);
    assert(err.file == "");
    assert(err.line == 0);
    true
}

function test_error_with_code() -> Bool {
    let err = error_with_code("error with code", 42);
    assert(err.message == "error with code");
    match err.error_code {
        Some(code) => assert(code == 42),
        None => panic("expected error code"),
    }
    true
}

function test_error_at_location() -> Bool {
    let err = error_at("error at location", "test.x", 10);
    assert(err.message == "error at location");
    assert(err.file == "test.x");
    assert(err.line == 10);
    true
}

// ============================================================================
// Test: Error Stack Chaining
// ============================================================================

function test_error_chain() -> Bool {
    let inner = simple_error("inner error");
    let outer = full_error("outer error", Some(1), "test.x", 5, Some(inner));
    
    assert(outer.message == "outer error");
    match outer.source {
        Some(src) => assert(src.message == "inner error"),
        None => panic("expected source error"),
    }
    true
}

function test_add_context() -> Bool {
    let inner = simple_error("original error");
    let with_context = inner.add_context("additional context");
    
    assert(with_context.message == "additional context: original error");
    match with_context.source {
        Some(src) => assert(src.message == "original error"),
        None => panic("expected source error"),
    }
    true
}

// ============================================================================
// Test: Error Builder
// ============================================================================

function test_error_builder() -> Bool {
    let err = error_builder("builder error")
        .with_code(100)
        .at("test.x", 20)
        .build();
    
    assert(err.message == "builder error");
    match err.error_code {
        Some(code) => assert(code == 100),
        None => panic("expected error code"),
    }
    assert(err.file == "test.x");
    assert(err.line == 20);
    true
}

// ============================================================================
// Test: IO Errors
// ============================================================================

function test_io_error() -> Bool {
    let err = io_error(IoErrorKind::FileNotFound, "file not found: test.txt");
    assert(err.kind == IoErrorKind::FileNotFound);
    assert(err.message == "file not found: test.txt");
    true
}

function test_file_not_found() -> Bool {
    let err = file_not_found("/path/to/file.txt");
    assert(err.kind == IoErrorKind::FileNotFound);
    assert(err.path == "/path/to/file.txt");
    true
}

function test_io_to_error_stack() -> Bool {
    let io_err = permission_denied("/etc/passwd");
    let stack = io_err.io_to_error_stack();
    
    assert(stack.message == "IoError: permission denied: /etc/passwd");
    match stack.error_code {
        Some(code) => assert(code == 2),
        None => panic("expected error code"),
    }
    true
}

// ============================================================================
// Test: Parse Errors
// ============================================================================

function test_parse_error() -> Bool {
    let err = parse_error("unexpected token");
    assert(err.message == "unexpected token");
    assert(err.file == "");
    assert(err.line == 0);
    true
}

function test_parse_error_with_location() -> Bool {
    let err = parse_error_at("unexpected token", "parser.x", 42, 10, "let x = ");
    assert(err.message == "unexpected token");
    assert(err.file == "parser.x");
    assert(err.line == 42);
    assert(err.column == 10);
    assert(err.source_line == "let x = ");
    true
}

// ============================================================================
// Test: Type Errors
// ============================================================================

function test_type_error() -> Bool {
    let err = type_error("Int", "String");
    assert(err.expected == "Int");
    assert(err.actual == "String");
    assert(err.message == "expected Int, got String");
    true
}

function test_type_error_msg() -> Bool {
    let err = type_error_msg("cannot add Int to String");
    assert(err.message == "cannot add Int to String");
    true
}

// ============================================================================
// Test: Runtime Errors
// ============================================================================

function test_runtime_error() -> Bool {
    let err = runtime_error("division by zero");
    assert(err.message == "division by zero");
    assert(err.recoverable == false);
    true
}

function test_recoverable_runtime_error() -> Bool {
    let err = recoverable_runtime_error("temporary failure", 503);
    assert(err.message == "temporary failure");
    assert(err.code == 503);
    assert(err.recoverable == true);
    true
}

// ============================================================================
// Test: Validation Errors
// ============================================================================

function test_validation_error() -> Bool {
    let err = validation_error("email", "invalid email format");
    assert(err.field == "email");
    assert(err.message == "invalid email format");
    true
}

function test_validation_error_value() -> Bool {
    let err = validation_error_value("age", "must be positive", "-5");
    assert(err.field == "age");
    assert(err.message == "must be positive");
    assert(err.value == "-5");
    true
}

// ============================================================================
// Test: Network Errors
// ============================================================================

function test_network_error() -> Bool {
    let err = network_error("connection refused");
    assert(err.message == "connection refused");
    assert(err.retryable == false);
    true
}

function test_network_error_retryable() -> Bool {
    let err = network_error_retryable("timeout", 504);
    assert(err.message == "timeout");
    assert(err.code == 504);
    assert(err.retryable == true);
    true
}

// ============================================================================
// Test: Timeout Errors
// ============================================================================

function test_timeout_error() -> Bool {
    let err = timeout_error("database query", 5000);
    assert(err.operation == "database query");
    assert(err.timeout_ms == 5000);
    true
}

// ============================================================================
// Test: Result and Option Integration
// ============================================================================

function divide(a: Int, b: Int) -> Result<Int, ErrorStack> {
    when b == 0 {
        Err(simple_error("division by zero"))
    } else {
        Ok(a / b)
    }
}

function test_result_propagation() -> Bool {
    let result = divide(10, 2);
    match result {
        Ok(value) => assert(value == 5),
        Err(_) => panic("expected Ok"),
    }
    true
}

function test_result_error() -> Bool {
    let result = divide(10, 0);
    match result {
        Ok(_) => panic("expected Err"),
        Err(err) => assert(err.message == "division by zero"),
    }
    true
}

function safe_divide(a: Int, b: Int) -> Result<Int, ErrorStack> {
    when b == 0 {
        Err(error_with_code("division by zero", 1001))
    } else {
        Ok(a / b)
    }
}

function test_error_propagation_with_context() -> Bool {
    let result = safe_divide(10, 0);
    match result {
        Ok(_) => panic("expected Err"),
        Err(err) => {
            assert(err.message == "division by zero");
            match err.error_code {
                Some(code) => assert(code == 1001),
                None => panic("expected error code"),
            }
        },
    }
    true
}

// ============================================================================
// Test: Option Utilities
// ============================================================================

function test_option_to_result() -> Bool {
    let some: Option<Int> = Some(42);
    let result = option_to_result(some, simple_error("no value"));
    match result {
        Ok(value) => assert(value == 42),
        Err(_) => panic("expected Ok"),
    }
    
    let none: Option<Int> = None;
    let result2 = option_to_result(none, simple_error("no value"));
    match result2 {
        Ok(_) => panic("expected Err"),
        Err(err) => assert(err.message == "no value"),
    }
    true
}

// ============================================================================
// Test: Error Formatting
// ============================================================================

function test_format_error_stack() -> Bool {
    let err = error_with_code("test error", 42);
    let formatted = format_error_stack(err);
    
    assert(formatted.contains("test error"));
    assert(formatted.contains("42"));
    true
}

function test_format_user_error() -> Bool {
    let inner = simple_error("inner error");
    let outer = full_error("outer error", Some(1), "test.x", 5, Some(inner));
    let formatted = format_user_error(outer);
    
    assert(formatted.contains("outer error"));
    assert(formatted.contains("inner error"));
    true
}

// ============================================================================
// Test: Error Categories
// ============================================================================

function test_error_categories() -> Bool {
    let io_err = base_error(ErrorCategory::Io, "io error");
    assert(io_err.category == ErrorCategory::Io);
    
    let parse_err = base_error(ErrorCategory::Parse, "parse error");
    assert(parse_err.category == ErrorCategory::Parse);
    
    let type_err = base_error(ErrorCategory::Type, "type error");
    assert(type_err.category == ErrorCategory::Type);
    
    let runtime_err = base_error(ErrorCategory::Runtime, "runtime error");
    assert(runtime_err.category == ErrorCategory::Runtime);
    
    true
}

// ============================================================================
// Main Test Runner
// ============================================================================

function run_test(name: string, test_fn: function() -> Bool) -> Result<unit, string> {
    match test_fn() {
        true => {
            println("  [PASS] " ++ name);
            Ok(unit)
        },
        false => {
            println("  [FAIL] " ++ name);
            Err("test failed: " ++ name)
        },
    }
}

export function main() -> Int {
    println("Error Handling Test Suite");
    println("=========================");
    println("");
    
    let mut passed = 0;
    let mut failed = 0;
    
    let tests: [(string, function() -> Bool)] = [
        ("simple error creation", test_simple_error),
        ("error with code", test_error_with_code),
        ("error at location", test_error_at_location),
        ("error chain", test_error_chain),
        ("add context", test_add_context),
        ("error builder", test_error_builder),
        ("io error", test_io_error),
        ("file not found", test_file_not_found),
        ("io to error stack", test_io_to_error_stack),
        ("parse error", test_parse_error),
        ("parse error with location", test_parse_error_with_location),
        ("type error", test_type_error),
        ("type error message", test_type_error_msg),
        ("runtime error", test_runtime_error),
        ("recoverable runtime error", test_recoverable_runtime_error),
        ("validation error", test_validation_error),
        ("validation error with value", test_validation_error_value),
        ("network error", test_network_error),
        ("retryable network error", test_network_error_retryable),
        ("timeout error", test_timeout_error),
        ("result propagation", test_result_propagation),
        ("result error", test_result_error),
        ("error propagation with context", test_error_propagation_with_context),
        ("option to result", test_option_to_result),
        ("format error stack", test_format_error_stack),
        ("format user error", test_format_user_error),
        ("error categories", test_error_categories),
    ];
    
    for (name, test_fn) in tests {
        match run_test(name, test_fn) {
            Ok(_) => passed = passed + 1,
            Err(_) => failed = failed + 1,
        }
    }
    
    println("");
    println("=========================");
    println("Tests passed: " ++ (passed as string));
    println("Tests failed: " ++ (failed as string));
    
    when failed > 0 {
        1
    } else {
        0
    }
}
