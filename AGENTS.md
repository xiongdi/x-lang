# PROJECT KNOWLEDGE BASE — X Language Compiler

**Updated:** 2026-04-15 | **Workspace:** dual (compiler/, tools/) | **Language:** Rust (compilation target)

---

## 🎯 OVERVIEW

**X** is a modern general-purpose language with Perceus (compile-time reference counting), no GC, strong types, and multi-backend code generation (Zig, Python, Rust, TypeScript, Java, C#, Swift, Erlang, LLVM, ASM). **Perceus is the centerpiece**—enables zero-copy data structure updates while maintaining safety. Compiler is structured as **frontend (lexer/parser/typechecker) → mid-end (HIR/MIR with Perceus analysis) → lowering (LIR) → backends (10+ codegen targets)**.

---

## 📋 CRITICAL RULES

- **`DESIGN_GOALS.md` is the constitution.** All design decisions must follow it. If any doc conflicts, trust `DESIGN_GOALS.md`.
- **Never modify `examples/`**—user-maintained. If example code fails, fix the compiler, not the example.
- **No exceptions to memory safety.** Perceus analysis must be sound; any reference count error is a compiler bug, not user error.

---

## 🏗️ WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| Add language feature | `compiler/AGENTS.md` → Feature Implementation Order | Spec → lexer → parser → typechecker → codegen/interpreter |
| Fix parser crash | `compiler/x-parser/` | LALRPOP-based; see `x-parser/CLAUDE.md` |
| Optimize code gen | `compiler/x-codegen/` | Abstract interface; backends inherit patterns |
| Perceus reference count bug | `compiler/x-mir/AGENTS.md` | MIR stage does dup/drop/reuse analysis |
| Add new backend | `compiler/x-codegen-{lang}/` | Copy existing backend as template, implement `emit()` |
| CLI arguments / pipeline | `tools/x-cli/AGENTS.md` | `pipeline.rs` orchestrates lexer→parser→...→codegen |
| Test execution | `compiler/` (unit), `tests/spec_runner/` (integration) | See "Build & Test" section |

---

## 🔗 ARCHITECTURE

### Compiler Pipeline (AST → HIR → MIR → LIR → Machine Code)

```
Source Code (.x)
    ↓ [x-lexer]
Tokens
    ↓ [x-parser]
AST (Program)
    ↓ [x-typechecker]
Typed AST
    ↓ [x-hir] (high-level IR)
HIR (semantic tree)
    ↓ [x-mir] (mid-level IR + Perceus analysis)
MIR (control flow graph + dup/drop/reuse marks)
    ↓ [x-lir] (low-level IR)
LIR (platform-neutral machine-like)
    ↓ [x-codegen-{zig,python,rust,...}]
Target Code (Zig/Python/Rust/...) → compile/run
```

### Workspace Structure

```
.
├── compiler/               # Compiler crates (shared workspace)
│   ├── x-lexer/           # Tokenization
│   ├── x-parser/          # AST generation (LALRPOP)
│   ├── x-typechecker/     # Type inference & checking
│   ├── x-hir/             # High-level IR (AST refinement)
│   ├── x-mir/             # Mid-level IR + Perceus analysis ⭐
│   ├── x-lir/             # Low-level IR (before codegen)
│   ├── x-codegen/         # Abstract codegen interface
│   ├── x-codegen-zig/     # Zig backend (most mature) ⭐
│   ├── x-codegen-python/  # Python backend
│   ├── [... 8 more backends: rust, java, csharp, swift, ts, erlang, llvm, asm ...]
│   ├── x-interpreter/     # AST-based interpreter (fallback)
│   └── x-test-integration/# Integration test harness
├── tools/                 # Tools workspace (separate; cross-refs compiler via path)
│   ├── x-cli/            # Main CLI entry point (orchestrates pipeline) ⭐
│   ├── x-lsp/            # Language server
│   └── x-syntax-gen/     # Generates editor syntax files
├── tests/                # Spec runner test harness
├── library/              # Stdlib (mentioned in README; not found in this snapshot)
├── docs/                 # User docs & tutorials
└── examples/             # User-maintained .x files (don't modify)
```

### Key Components (Score → Relevance)

| Crate | Role | Score | AGENTS.md? |
|-------|------|-------|-----------|
| **compiler/** | Pipeline hub (20 crates: 10 backends + IR) | 22 | ✅ Yes |
| **compiler/x-mir** | Perceus reference counting analysis ⭐ | 15 | ✅ Yes |
| **compiler/x-codegen** | Multi-backend abstraction layer | 16 | ✅ Yes |
| **compiler/x-parser** | AST generation (LALRPOP) | 14 | ✅ Yes |
| **tools/x-cli** | CLI orchestration & entry point | 16 | ✅ Yes |
| **compiler/x-typechecker** | Type inference & checking | 12 | ⬜ See parent |
| **compiler/x-lir** | Low-level IR | 10 | ⬜ See parent |
| **tools/x-lsp** | Language server | 8 | ⬜ See parent |

---

## 🛠️ BUILD & TEST

```bash
# Build all (uses both compiler/ and tools/ workspaces)
cd compiler && cargo build
cd tools && cargo build

# Run CLI
cd tools/x-cli && cargo run -- run ../../examples/hello.x
cd tools/x-cli && cargo run -- compile ../../examples/hello.x -o hello

# Test all crates
cd compiler && cargo test
cd tools && cargo test

# Test specific crate
cd compiler && cargo test -p x-parser
cd compiler && cargo test -p x-mir --lib perceus

# Format & lint
cargo fmt
```

**Note**: No Makefile or GitHub Actions workflow present. All builds via cargo commands.

---

## ⚙️ CONVENTIONS

- **Workspace structure**: Dual workspaces (compiler/ and tools/) with cross-workspace path dependencies (tools/Cargo.toml references ../compiler/* crates).
- **Resolver**: Cargo resolver v2 in both workspaces.
- **Shared dependencies**: Centralized in workspace.dependencies (thiserror, log, im, lalrpop, tokio, clap, etc.).
- **No rustfmt.toml / clippy.toml / .editorconfig** found. Default Rust conventions apply.
- **Code style**: `log::debug!` for internal details; avoid `println!` in lib code. `cargo fmt` required.
- **Module layout**: Each crate uses src/lib.rs; no mod.rs files detected (modern Rust convention).

---

## 🚀 FEATURE IMPLEMENTATION ORDER

When adding a new language feature:

1. **Update `SPEC.md`** — Define syntax, semantics, type rules
2. **Update `x-lexer`** — Add tokens (if new symbols needed)
3. **Update `x-parser`** — Add AST nodes, LALRPOP rules
4. **Update `x-typechecker`** — Type inference, semantic checks
5. **Update `x-hir`** — Refine AST if needed (usually pass-through)
6. **Update `x-mir`** — Add MIR constructs if needed; Perceus handling
7. **Update `x-lir`** — Lower MIR to LIR if needed
8. **Update backends** — Extend codegen in x-codegen or specific x-codegen-{lang}
9. **Add tests** — Integration tests in tests/, unit tests in each crate

---

## ⭐ PERCEUS (Memory Management)

**Why Perceus matters**: Compile-time reference counting + reuse analysis eliminates GC pause times while keeping dev experience of high-level languages.

- **Reference counting**: Compiler inserts `dup` (copy ref) and `drop` (release ref) at compile time.
- **Reuse analysis**: When a value's ref count = 1, in-place update replaces allocation.
- **Implementation**: See `compiler/x-mir/AGENTS.md` for Perceus analysis details.
- **Non-negotiable**: Any bug in dup/drop insertion = safety violation. Errors must be reported & fixed immediately.

---

## ⚙️ CODEGEN BACKENDS

| Backend | Location | Status | Notes |
|---------|----------|--------|-------|
| **Zig** | `x-codegen-zig/` | ✅ Mature | Primary; supports native + Wasm |
| **TypeScript** | `x-codegen-typescript/` | 🚧 Early | Browser/Node.js target |
| **Python** | `x-codegen-python/` | 🚧 Early | Python source output |
| **Rust** | `x-codegen-rust/` | 🚧 Early | Rust interop |
| **Java** | `x-codegen-java/` | 🚧 Early | JVM target |
| **C#** | `x-codegen-csharp/` | 🚧 Early | .NET platform |
| **LLVM** | `x-codegen-llvm/` | 🚧 Early | LLVM IR output |
| **Swift** | `x-codegen-swift/` | 🚧 Early | Apple ecosystem |
| **Erlang** | `x-codegen-erlang/` | 🚧 Early | Concurrency/distribution |
| **ASM** | `x-codegen-asm/` | 🚧 Early | Direct assembly |

Each backend inherits from `x-codegen/` trait. To add a backend: copy an existing one, implement `emit()` for your target language, register in CLI.

---

## 🧪 TESTING STRATEGY

- **Unit tests**: In each crate under `#[test]` or via `cargo test -p <crate>`.
- **Integration tests**: `tests/spec_runner/` runs `.x` spec files; see `tests/spec_runner/CLAUDE.md`.
- **No CI automation**: Builds are manual (see "Build & Test" section).

---

## 📚 KEY DOCUMENTS

| Doc | Purpose |
|-----|---------|
| **DESIGN_GOALS.md** | Language philosophy & constraints (read first!) |
| **SPEC.md** | Formal syntax & semantics |
| **README.md** | Quick start & overview |
| **compiler/CLAUDE.md** | Compiler workspace guidelines |
| **tools/CLAUDE.md** | CLI/LSP/tools workspace guidelines |
| Each crate's **CLAUDE.md** | Crate-specific details |

---

## 🔴 ANTI-PATTERNS (THIS PROJECT)

- ❌ **Don't add `println!` in library code**—use `log::debug!` instead.
- ❌ **Don't modify `examples/`**—fix the compiler if example code fails.
- ❌ **Don't skip Perceus analysis in MIR**—every value must have correct dup/drop marks.
- ❌ **Don't bypass type checking**—if compilation succeeds, type safety is guaranteed.
- ❌ **Don't add dependencies without consensus**—use workspace.dependencies for consistency.

---

## 🎯 NEXT STEPS

1. Read `DESIGN_GOALS.md` for language philosophy.
2. Check `compiler/AGENTS.md` for pipeline details if working on features.
3. Check `tools/x-cli/AGENTS.md` if working on CLI or pipeline orchestration.
4. Check `compiler/x-mir/AGENTS.md` if working on Perceus analysis.
5. See respective crate's `CLAUDE.md` for deep implementation details.
