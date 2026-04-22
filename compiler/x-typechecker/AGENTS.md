# AGENTS.md — compiler/x-typechecker/

**Updated:** 2026-04-22 | **Stage:** Type Checking

---

## OVERVIEW

Type checking & semantic analysis - operates on x_parser::ast::Program, produces diagnostics or failure.

---

## KEY TYPES

| Function | Purpose |
|---------|---------|
| `type_check(program) -> Result<(), TypeError>` | Single-error stop style |
| `type_check_with_env(program) -> Result<TypeEnv, TypeError>` | When env needed |
| `TypeCheckResult` | Multi-error collection |

| Type | Purpose |
|------|---------|
| `TypeEnv` | Type environment mapping |
| `TypeError` | Error type |
| `Severity` | Error severity levels |
| `ErrorCategory` | Error classification |

---

## PIPELINE

Input: x_parser::ast::Program
Output: TypeEnv (on success) → x-hir
Error location: x_lexer::span::Span

---

## CONVENTIONS

1. Use Span for error locations
2. TypeCheckResult for multiple errors
3. Severity levels: Error, Warning, Note

---

## DEPENDENCIES

- **Input:** x_parser::ast::Program
- **Error info:** x_lexer::span::Span
- **Output:** x-hir

---

## TESTS

```bash
cd compiler && cargo test -p x-typechecker
```

---

## NEXT STEPS

- For type inference details: see x-hir
- For error handling: see errors.rs
