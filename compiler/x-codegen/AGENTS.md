# AGENTS.md — compiler/x-codegen/

**Multi-backend codegen abstraction layer.** Defines the interface all backends implement.

---

## OVERVIEW

Trait-based architecture for converting LIR (platform-neutral IR) into target language code.
- Central abstraction: all backends (Zig, Python, Rust, Java, C#, LLVM, Swift, Erlang, ASM, TypeScript) implement the same `Emit` trait
- Shared utilities: indentation, formatting, name mangling, block assembly
- Input: `x-lir` (mid-level representation)
- Output: target language source code (Zig, Python, etc.)

---

## RESPONSIBILITY

1. **Emit trait**: Core interface every backend implements to turn LIR into target code
2. **CodeGenerator orchestration**: Manage emission context, options, output buffering
3. **Shared utilities**: Common patterns for name mangling, indentation, block scoping
4. **Error mapping**: Consistent error reporting across all backends
5. **No backend-specific logic**: Keep this crate language-agnostic; special cases live in concrete backends

---

## WHERE TO LOOK

**To add a new backend:**
1. Copy an existing backend (e.g., `x-codegen-zig/`) as template
2. Implement the `Emit` trait for your target language
3. Register in CLI pipeline (tools/x-cli/src/pipeline.rs)
4. Test with `cargo test -p x-codegen-{lang}`

**To extend the trait:**
1. Modify `Emit` trait definition here
2. Update all 10 backends to implement the new method
3. Add tests for trait completeness

---

## KEY TYPES & TRAITS

| Type | Purpose |
|------|---------|
| `Emit` | Central trait; all backends impl this |
| `CodeGenerator` | Orchestrates emission for a target backend |
| `EmitContext` | Tracks emission state (indentation, scope, etc.) |
| `LirNode` | Input from `x-lir` (values, statements, functions) |
| `CodeBuffer` | Output accumulation (strings or file handles) |

---

## BACKENDS & PATTERNS

All 10 backends follow identical structure:

```
x-codegen-{lang}/
├── src/
│   ├── lib.rs           # Pub API + re-exports
│   ├── emit.rs          # Impl Emit trait
│   ├── context.rs       # Target-specific state
│   └── tests/           # Backend-specific tests
└── Cargo.toml           # Depends on x-codegen + x-lir
```

**To add a new backend:**
1. Create `compiler/x-codegen-{lang}/` directory
2. Copy `Cargo.toml` from existing backend, adjust name
3. Implement `Emit` trait (mirror existing backend's structure)
4. Add to `compiler/Cargo.toml` members list
5. Register in CLI pipeline

---

## DEPENDENCIES

- **Input**: `x-lir` (workspace dependency) — platform-neutral IR
- **No output dependencies**: Generated code is independent; backends don't link to runtime
- **Shared utilities**: Reusable helpers from this crate

---

## CONVENTIONS

1. **Trait implementation**: All backends implement `Emit` identically; signatures non-negotiable
2. **Error handling**: Use `Result<String, EmitError>` for emit() return
3. **Determinism**: emit() must be pure (same input → same output)
4. **Testing**: Each backend has unit tests for code generation correctness
5. **Naming**: Consistent with project conventions (no special characters in emitted names)
6. **Performance**: Avoid unnecessary allocations in hot emit paths

---

## NEXT STEPS

- For **Perceus handling in generated code**, coordinate with x-mir stage
- For **backend-specific docs**, check individual `x-codegen-{lang}/` directories
- For **CLI integration**, see `tools/x-cli/AGENTS.md`
