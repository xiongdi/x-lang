# AGENTS.md — tests/

**Updated:** 2026-04-22 | **Type:** Integration Test Suite

---

## OVERVIEW

X language integration test suite - validates compilation pipeline outputs for all language features. TOML-based spec files drive test execution across lexical, types, expressions, statements, functions, OOP, effects, modules, patterns, memory, and metaprogramming.

---

## TEST STRUCTURE

| Category | Directory | Coverage |
|---------|----------|----------|
| Lexical | `lexical/` | Keywords, identifiers, literals, operators, comments |
| Types | `types/` | Basic types, compound types, generics, Option/Result |
| Expressions | `expressions/` | Arithmetic, logic, comparison, pipe, error handling |
| Statements | `statements/` | Variable declaration, assignment, control flow, loops |
| Functions | `functions/` | Function definition, parameters, closures, effects |
| OOP | `oop/` | Classes, inheritance, traits, access control |
| Effects | `effects/` | Effect annotations, async, needs/given |
| Modules | `modules/` | Import, export, visibility |
| Patterns | `patterns/` | Pattern matching, exhaustiveness |
| Memory | `memory/` | Ownership, references, weak references |
| Metaprogramming | `metaprogramming/` | Constants, generics, macros |

---

## TEST FILE FORMAT

TOML format with:
- `name`, `description`, `category`, `spec` (spec doc references)
- `source` - X language test code
- `[expect]` - compile behavior, exit_code
- `[expect.tokens]`, `[expect.ast]`, `[expect.hir]`, `[expect.mir]`, `[expect.lir]` - stage validation
- `[expect.runtime]` - expected output

---

## COMMANDS

```bash
# All tests
python tests/run_tests.py

# Specific category
python tests/run_tests.py --category lexical

# Single test
python tests/run_tests.py tests/lexical/keywords/basic.toml

# Verbose
python tests/run_tests.py --verbose

# List tests
python tests/run_tests.py --list
```

---

## PIPELINE COORDINATION

- Integration with: `compiler/x-test-integration/` crate
- CLI path: `tools/x-cli`
- Python deps: `tomli` (TOML parsing)

---

## ADDING TESTS

1. Create `.toml` file in appropriate category directory
2. Fill metadata + source code
3. Define expected behavior
4. Run `python tests/run_tests.py` to validate

---

## NEXT STEPS

- For unit tests: `cd compiler && cargo test -p <crate>`
- For integration: `python tests/run_tests.py`
- For CLI: `cd tools/x-cli && cargo run -- run ../../examples/hello.x`
