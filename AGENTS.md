# AGENTS.md

## Critical Rules

- **DESIGN_GOALS.md is the highest-priority document.** All design decisions must follow it. If other docs conflict, trust DESIGN_GOALS.md.
- **Never modify files in `examples/`** - these are user-maintained. If example code fails, fix the compiler, not the example.

## Build & Test

```bash
# Build CLI
cd tools/x-cli && cargo build

# Run .x file (interpret)
cd tools/x-cli && cargo run -- run <file.x>

# Compile with Zig backend (most mature)
cd tools/x-cli && cargo run -- compile <file.x> -o output

# Run all compiler tests
cd compiler && cargo test

# Single crate test
cd compiler && cargo test -p x-parser <test_name>

# Format
cargo fmt
```

## Architecture

- **IR pipeline**: AST → HIR → MIR (Perceus analysis here) → LIR → codegen
- **Entry point**: `tools/x-cli` orchestrates the full pipeline
- **Major crates**: `compiler/x-{lexer,parser,hir,mir,lir,typechecker,codegen}`, `tools/x-cli`, `library/stdlib`
- **Zig backend** is most mature; requires Zig 0.13.0+ in PATH

## Feature Implementation Order

1. Update `SPEC.md` (language spec)
2. Update `x-lexer` (new tokens)
3. Update `x-parser` (AST nodes)
4. Update `x-hir` (if new IR)
5. Update `x-typechecker` (semantics)
6. Update `x-codegen` or `x-interpreter` (codegen/runtime)
7. Add tests in `tests/` (TOML-based)

## Key Conventions

- Use `log::debug!` for internal details, avoid `println!` in library code
- Format with `cargo fmt`
- Test at crate level: `cargo test -p <crate>`
