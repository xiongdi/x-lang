# AGENTS.md — compiler/x-interpreter/

**Updated:** 2026-04-22 | **Execution:** AST Interpreter

---

## OVERVIEW

Tree-walking interpreter - executes x_parser::ast::Program directly without MIR/LIR. Fallback execution path.

---

## KEY TYPES

| Type | Purpose |
|------|---------|
| `Interpreter` | Main interpreter struct |
| `Value` | Runtime values (integers, floats, strings, Array, Rc) |
| `InterpreterError` | Runtime errors |

---

## ENTRY POINT

```rust
Interpreter::new()
run(&mut self, program: &Program) -> Result<(), InterpreterError>
```

---

## PIPELINE

Input: AST (after type checking via x-typechecker)
Output: Execution result
Used by: tools/x-cli/src/commands/run.rs

---

## DEPENDENCIES

- **Input:** x_parser::ast::Program (type-checked)
- **Used by:** CLI run command

---

## CONVENTIONS

1. Semantic alignment with language spec
2. No backend-only features needed

---

## TESTS

```bash
cd compiler && cargo test -p x-interpreter
```

---

## NEXT STEPS

- For full pipeline: see tools/x-cli
