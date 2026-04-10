// Error Handling Example
// Demonstrates the error handling mechanisms in X

import std::prelude::*;
import std::error::*;
import std::errors::*;
import std::fs::*;

// Example: File operations with proper error handling
function read_config(path: string) -> Result<string, ErrorStack> {
    match fs::open(path, OpenMode::Read) {
        Err(err) => Err(error_at("failed to open config: " ++ err, "config.x", 10)),
        Ok(mut file) => {
            let content_result = file.read_to_string();
            file.close();
            
            match content_result {
                Err(err) => Err(error_at("failed to read config: " ++ err, "config.x", 16)),
                Ok(content) => Ok(content),
            }
        },
    }
}

// Example: Division with error handling
function safe_divide(a: Int, b: Int) -> Result<Int, ErrorStack> {
    when b == 0 {
        Err(error_with_code("division by zero", 1001))
    } else {
        Ok(a / b)
    }
}

// Example: User lookup with error handling
record User {
    id: Int,
    name: string,
    email: string,
}

function find_user(id: Int) -> Result<User, ErrorStack> {
    // Simulated user lookup
    when id < 0 {
        Err(validation_error_value("id", "must be positive", (id as string)).validation_to_error_stack())
    } else if id == 0 {
        Err(error_with_code("user not found", 404))
    } else {
        Ok(User {
            id: id,
            name: "User " ++ (id as string),
            email: "user" ++ (id as string) ++ "@example.com",
        })
    }
}

// Example: Using the ? operator for error propagation
function calculate_average(a: Int, b: Int) -> Result<Int, ErrorStack> {
    // Using explicit pattern matching instead of ? operator
    // since we need to handle errors properly
    match safe_divide(a + b, 2) {
        Err(err) => Err(err.add_context("calculate_average failed")),
        Ok(result) => Ok(result),
    }
}

// Example: Error chain building
function process_user_data(user_id: Int) -> Result<string, ErrorStack> {
    match find_user(user_id) {
        Err(err) => Err(err.add_context("process_user_data failed")),
        Ok(user) => {
            // Process user data
            let email = user.email;
            Ok("Processed user: " ++ user.name ++ " <" ++ email ++ ">")
        },
    }
}

// Example: Handling multiple error types
function read_and_parse(path: string) -> Result<Int, ErrorStack> {
    match read_config(path) {
        Err(err) => Err(err.add_context("read_and_parse failed")),
        Ok(content) => {
            // Try to parse as integer
            // In a real implementation, we would have a parse function
            Ok(42)  // Simulated parse result
        },
    }
}

// Main function demonstrating error handling
export function main() -> Int {
    println("Error Handling Examples");
    println("=======================");
    println("");
    
    // Example 1: Safe division
    println("1. Safe Division:");
    match safe_divide(10, 2) {
        Ok(result) => println("   10 / 2 = " ++ (result as string)),
        Err(err) => println("   Error: " ++ err.message),
    }
    
    match safe_divide(10, 0) {
        Ok(result) => println("   10 / 0 = " ++ (result as string)),
        Err(err) => {
            println("   Error: " ++ err.message);
            match err.error_code {
                Some(code) => println("   Code: " ++ (code as string)),
                None => {},
            }
        },
    }
    println("");
    
    // Example 2: User lookup
    println("2. User Lookup:");
    match find_user(1) {
        Ok(user) => println("   Found: " ++ user.name ++ " <" ++ user.email ++ ">"),
        Err(err) => println("   Error: " ++ err.message),
    }
    
    match find_user(0) {
        Ok(user) => println("   Found: " ++ user.name),
        Err(err) => println("   Error: " ++ err.message),
    }
    
    match find_user(-1) {
        Ok(user) => println("   Found: " ++ user.name),
        Err(err) => println("   Error: " ++ err.message),
    }
    println("");
    
    // Example 3: Error chain
    println("3. Error Chain:");
    match process_user_data(0) {
        Ok(msg) => println("   " ++ msg),
        Err(err) => {
            println("   Error: " ++ err.message);
            match err.source {
                Some(src) => println("   Caused by: " ++ src.message),
                None => {},
            }
        },
    }
    println("");
    
    // Example 4: Error builder
    println("4. Error Builder:");
    let custom_err = error_builder("custom error message")
        .with_code(500)
        .at("example.x", 100)
        .build();
    println("   Message: " ++ custom_err.message);
    match custom_err.error_code {
        Some(code) => println("   Code: " ++ (code as string)),
        None => {},
    }
    println("   Location: " ++ custom_err.file ++ ":" ++ (custom_err.line as string));
    println("");
    
    // Example 5: IO Errors
    println("5. IO Errors:");
    let io_err = file_not_found("/etc/nonexistent.conf");
    println("   Type: IoError");
    println("   Kind: file not found");
    println("   Path: " ++ io_err.path);
    println("   Message: " ++ io_err.message);
    println("");
    
    // Example 6: Parse Errors
    println("6. Parse Errors:");
    let parse_err = parse_error_at("unexpected token '}'", "test.x", 42, 10, "let x = }");
    println("   Message: " ++ parse_err.message);
    println("   Location: " ++ parse_err.file ++ ":" ++ (parse_err.line as string) ++ ":" ++ (parse_err.column as string));
    println("   Source: " ++ parse_err.source_line);
    println("");
    
    // Example 7: Type Errors
    println("7. Type Errors:");
    let type_err = type_error("Int", "String");
    println("   Expected: " ++ type_err.expected);
    println("   Actual: " ++ type_err.actual);
    println("   Message: " ++ type_err.message);
    println("");
    
    // Example 8: Runtime Errors
    println("8. Runtime Errors:");
    let rt_err = recoverable_runtime_error("temporary failure", 503);
    println("   Message: " ++ rt_err.message);
    println("   Code: " ++ (rt_err.code as string));
    println("   Recoverable: " ++ (rt_err.recoverable as string));
    println("");
    
    // Example 9: Validation Errors
    println("9. Validation Errors:");
    let val_err = validation_error_value("email", "invalid format", "not-an-email");
    println("   Field: " ++ val_err.field);
    println("   Message: " ++ val_err.message);
    println("   Value: " ++ val_err.value);
    println("");
    
    // Example 10: Network Errors
    println("10. Network Errors:");
    let net_err = network_error_url("connection refused", "https://api.example.com");
    println("   Message: " ++ net_err.message);
    println("   URL: " ++ net_err.url);
    println("   Retryable: " ++ (net_err.retryable as string));
    println("");
    
    // Example 11: Timeout Errors
    println("11. Timeout Errors:");
    let timeout_err = timeout_error("database query", 5000);
    println("   Operation: " ++ timeout_err.operation);
    println("   Timeout: " ++ (timeout_err.timeout_ms as string) ++ "ms");
    println("");
    
    // Example 12: Error Categories
    println("12. Error Categories:");
    let base_io = base_error(ErrorCategory::Io, "io failed");
    let base_parse = base_error(ErrorCategory::Parse, "parse failed");
    let base_type = base_error(ErrorCategory::Type, "type mismatch");
    let base_runtime = base_error(ErrorCategory::Runtime, "runtime error");
    println("   Categories: Io, Parse, Type, Runtime, Network, Database, Validation, etc.");
    println("");
    
    println("All examples completed successfully!");
    0
}
